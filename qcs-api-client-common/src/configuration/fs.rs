//! Utilities for safely writing configuration files to disk.

use std::path::{Path, PathBuf};

use async_tempfile::TempFile;
use tokio::io::AsyncWriteExt as _;

use super::error::{IoErrorWithPath, IoOperation, WriteError};

/// Atomically overwrite the file at `path` with `bytes`.
///
/// The new contents are written to a temporary file which is then renamed over
/// `path`. To keep that rename atomic, the temporary file is staged in the same
/// directory as the *canonical* destination: if `path` (or one of its parent
/// directories) is a symlink crossing a filesystem boundary, staging the
/// temporary file beside the symlink would put it on a different mount point and
/// the rename would fail with a `cross-device link error (OS 18)`. Resolving to
/// the real location first avoids that.
///
/// If the destination already exists its permissions are preserved. Otherwise,
/// on Unix the file is created with `0600` permissions (configuration files may
/// contain secrets and should not be world-readable); on other platforms the
/// default permissions are used. Any missing parent directories are created.
///
/// # Errors
///
/// [`WriteError`] if the destination cannot be resolved, the temporary file
/// cannot be written, or the final rename fails.
pub async fn atomic_write(
    path: impl AsRef<Path> + Send + Sync,
    bytes: &[u8],
) -> Result<(), WriteError> {
    let dest = canonical_destination(path.as_ref()).await?;
    let dest_dir = dest.parent().unwrap_or_else(|| Path::new("."));

    let mut temp_file = TempFile::new_in(dest_dir).await?;
    #[cfg(feature = "tracing")]
    tracing::debug!("staging temporary file at {:?}", temp_file.file_path());

    if let Some(permissions) = permissions_for(&dest).await? {
        temp_file
            .set_permissions(permissions)
            .await
            .map_err(|error| IoErrorWithPath {
                error,
                path: temp_file.file_path().clone(),
                operation: IoOperation::SetPermissions,
            })?;
    }

    temp_file
        .write_all(bytes)
        .await
        .map_err(|error| IoErrorWithPath {
            error,
            path: temp_file.file_path().clone(),
            operation: IoOperation::Write,
        })?;
    temp_file.flush().await.map_err(|error| IoErrorWithPath {
        error,
        path: temp_file.file_path().clone(),
        operation: IoOperation::Flush,
    })?;

    let temp_file_path = temp_file.file_path();
    #[cfg(feature = "tracing")]
    tracing::debug!("atomically replacing {dest:?} with {temp_file_path:?}");
    tokio::fs::rename(temp_file_path, &dest)
        .await
        .map_err(|error| IoErrorWithPath {
            error,
            path: temp_file_path.clone(),
            operation: IoOperation::Rename { dest: dest.clone() },
        })?;

    Ok(())
}

/// Determine the permissions to apply to the file being written.
///
/// An existing file keeps its current permissions. A file that does not yet
/// exist is created with `0600` on Unix and with default permissions elsewhere
/// (returned as `None`, meaning "leave as-is").
async fn permissions_for(dest: &Path) -> Result<Option<std::fs::Permissions>, WriteError> {
    match tokio::fs::metadata(dest).await {
        Ok(metadata) => Ok(Some(metadata.permissions())),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt as _;
                Ok(Some(std::fs::Permissions::from_mode(0o600)))
            }
            #[cfg(not(unix))]
            {
                Ok(None)
            }
        }
        Err(error) => Err(IoErrorWithPath {
            error,
            path: dest.to_path_buf(),
            operation: IoOperation::GetMetadata,
        }
        .into()),
    }
}

/// Resolve `path` to its canonical location on the real filesystem, creating the
/// parent directory if it does not already exist.
///
/// If the destination already exists (including as a symlink) it is resolved to
/// its target, so callers stage temporary files next to — and rename onto — the
/// real file rather than a symlink that may live on a different mount point.
/// Otherwise the parent directory is created and canonicalized, so a symlinked
/// config directory is followed to where its contents actually live.
async fn canonical_destination(path: &Path) -> Result<PathBuf, WriteError> {
    if let Ok(canonical) = tokio::fs::canonicalize(path).await {
        return Ok(canonical);
    }

    let parent = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .map_or_else(|| PathBuf::from("."), Path::to_path_buf);
    tokio::fs::create_dir_all(&parent)
        .await
        .map_err(|error| IoErrorWithPath {
            error,
            path: parent.clone(),
            operation: IoOperation::Write,
        })?;
    let canonical_parent =
        tokio::fs::canonicalize(&parent)
            .await
            .map_err(|error| IoErrorWithPath {
                error,
                path: parent.clone(),
                // `canonicalize` resolves the path via metadata syscalls; reuse the
                // existing operation rather than adding a variant (which would be a
                // breaking change to the public `IoOperation` enum).
                operation: IoOperation::GetMetadata,
            })?;

    let file_name = path.file_name().ok_or_else(|| IoErrorWithPath {
        error: std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "destination path has no file name",
        ),
        path: path.to_path_buf(),
        operation: IoOperation::Write,
    })?;
    Ok(canonical_parent.join(file_name))
}

#[cfg(test)]
mod tests {
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::{atomic_write, canonical_destination};

    fn unique_test_root(label: &str) -> std::path::PathBuf {
        std::env::temp_dir().join(format!(
            "qcs-common-atomic-write-test-{label}-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("system clock should be after unix epoch")
                .as_nanos()
        ))
    }

    #[tokio::test]
    async fn destination_resolves_next_to_the_target_creating_missing_dirs() {
        let root = unique_test_root("missing-dirs");
        let target_dir = root.join("nested").join("config");
        let target_file = target_dir.join("secrets.toml");

        let dest = canonical_destination(&target_file)
            .await
            .expect("should resolve a destination next to the target file");

        let canonical_target_dir = tokio::fs::canonicalize(&target_dir)
            .await
            .expect("parent directory should have been created");
        assert_eq!(dest.parent(), Some(canonical_target_dir.as_path()));
        assert_eq!(dest.file_name(), target_file.file_name());

        std::fs::remove_dir_all(root).expect("should remove the test directory");
    }

    // A symlinked file crossing a filesystem boundary is the case that motivated
    // canonicalizing: the destination must resolve to where the real file lives,
    // not the directory holding the symlink.
    #[cfg(unix)]
    #[tokio::test]
    async fn destination_follows_a_symlinked_file_to_its_real_location() {
        let root = unique_test_root("symlinked-file");
        let real_dir = root.join("real");
        let link_dir = root.join("links");
        tokio::fs::create_dir_all(&real_dir)
            .await
            .expect("should create the real directory");
        tokio::fs::create_dir_all(&link_dir)
            .await
            .expect("should create the links directory");

        let real_file = real_dir.join("secrets.toml");
        tokio::fs::write(&real_file, b"contents")
            .await
            .expect("should create the real file");

        let symlink = link_dir.join("secrets.toml");
        std::os::unix::fs::symlink(&real_file, &symlink).expect("should create the symlink");

        let dest = canonical_destination(&symlink)
            .await
            .expect("should resolve the symlink to its target");

        let canonical_real_dir = tokio::fs::canonicalize(&real_dir)
            .await
            .expect("real directory should exist");
        assert_eq!(dest.parent(), Some(canonical_real_dir.as_path()));

        std::fs::remove_dir_all(root).expect("should remove the test directory");
    }

    #[tokio::test]
    async fn atomic_write_creates_then_overwrites_the_file() {
        let root = unique_test_root("write");
        let target = root.join("nested").join("secrets.toml");

        atomic_write(&target, b"first")
            .await
            .expect("should create the file");
        assert_eq!(
            tokio::fs::read(&target).await.expect("file should exist"),
            b"first"
        );

        atomic_write(&target, b"second")
            .await
            .expect("should overwrite the file");
        assert_eq!(
            tokio::fs::read(&target).await.expect("file should exist"),
            b"second"
        );

        std::fs::remove_dir_all(root).expect("should remove the test directory");
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn atomic_write_creates_new_files_with_owner_only_permissions() {
        use std::os::unix::fs::PermissionsExt as _;

        let root = unique_test_root("perms");
        let target = root.join("secrets.toml");

        atomic_write(&target, b"secret")
            .await
            .expect("should create the file");

        let mode = tokio::fs::metadata(&target)
            .await
            .expect("file should exist")
            .permissions()
            .mode();
        assert_eq!(mode & 0o777, 0o600);

        std::fs::remove_dir_all(root).expect("should remove the test directory");
    }
}
