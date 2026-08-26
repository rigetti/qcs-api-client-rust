//! Support for credentials whose access tokens are produced by an external program.

use std::{path::PathBuf, process::Stdio, time::Duration};

use serde::{Deserialize, Serialize};
use tokio::io::AsyncReadExt as _;

use crate::configuration::tokens::ExternallyManaged;

use super::secret_string::SecretAccessToken;

/// The most stdout (tokens) or stderr (error messages) retained.
const MAX_PIPE_BYTES: usize = 4 * 1024;

/// How long the command is given to produce an access token when `timeout_seconds` is unset.
const DEFAULT_TIMEOUT_SECONDS: u64 = 30;

const fn default_timeout_seconds() -> u64 {
    DEFAULT_TIMEOUT_SECONDS
}

#[allow(clippy::trivially_copy_pass_by_ref, reason = "serde needs a reference")]
fn is_default_timeout_seconds(timeout_seconds: &u64) -> bool {
    timeout_seconds == &DEFAULT_TIMEOUT_SECONDS
}

/// A credential whose access tokens come from running an external program, rather than from
/// tokens stored in `secrets.toml`.
///
/// This is how you integrate QCS clients with a credential helper in e.g. hosted environments.
/// Valid tokens are cached in-process so the subcommand is only called when a new token is needed.
///
/// With this method, tokens are never written back to `secrets.toml`.
///
/// # Schema
///
/// ```toml
/// [credentials.coder.externally_managed]
/// # Required.
/// command = "/usr/bin/coder"
/// # Optional. Passed to the program verbatim; no shell is involved, so no quoting or
/// # escaping is applied or required.
/// args = ["external-auth", "access-token", "qcs"]
/// # Optional, defaults to 30. How long the program may run before it is killed.
/// timeout_seconds = 30
/// ```
///
/// All [`super::settings::Profile`]s still reference an `auth_server`, although this credential
/// method does not use the auth server when fetching credential.
///
/// # Security
///
/// This turns `secrets.toml` into a file that causes code to run: anyone who can write to it, or
/// to the program it names, can run arbitrary code as you. The program runs with this process's
/// environment, so it sees the same `PATH` and variables the client does.
///
/// The usual precautions apply if that matters for your environment: keep `secrets.toml` writable
/// only by your own user, and give `command` an absolute path so it can't be resolved through an
/// attacker-controlled `PATH`.
///
/// The subcommand's stderr is included in error messages, so avoid emitting sensitive values there.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ExternallyManagedCredential {
    /// Path to the program that produces an access token on stdout.
    pub command: PathBuf,
    /// Arguments passed to the program verbatim, without shell interpretation.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub args: Vec<String>,
    /// How long the program may run before it is killed. Defaults to 30 seconds.
    #[serde(
        default = "default_timeout_seconds",
        skip_serializing_if = "is_default_timeout_seconds"
    )]
    pub timeout_seconds: u64,
}

impl From<ExternallyManagedCredential> for ExternallyManaged {
    fn from(credential: ExternallyManagedCredential) -> Self {
        Self::from_async(move |_auth_server| {
            let credential = credential.clone();
            async move {
                credential
                    .request_access_token()
                    .await
                    .map(|token| token.secret().to_string())
                    .map_err(Into::into)
            }
        })
    }
}

impl ExternallyManagedCredential {
    /// Run the program and return the access token it prints to stdout.
    ///
    /// # Errors
    ///
    /// See [`ExternalCommandError`].
    pub async fn request_access_token(&self) -> Result<SecretAccessToken, ExternalCommandError> {
        // Boxed because `run`'s state holds a `Command`, which is large enough that leaving it
        // inline bloats the future of every caller that awaits a token.
        let output = Box::pin(self.run()).await?;

        let token = String::from_utf8(output)
            .map_err(|_| ExternalCommandError::InvalidUtf8 {
                program: self.command.clone(),
            })?
            .trim()
            .to_string();

        if token.is_empty() {
            return Err(ExternalCommandError::EmptyOutput {
                program: self.command.clone(),
            });
        }

        Ok(SecretAccessToken::from(token))
    }

    /// Spawn the program and collect its (capped) stdout, enforcing [`Self::timeout_seconds`].
    async fn run(&self) -> Result<Vec<u8>, ExternalCommandError> {
        let mut command = tokio::process::Command::new(&self.command);

        command
            .args(&self.args)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true);

        let mut child = command
            .spawn()
            .map_err(|source| ExternalCommandError::Spawn {
                program: self.command.clone(),
                source,
            })?;

        let stdout = child.stdout.take().expect("stdout is piped");
        let stderr = child.stderr.take().expect("stderr is piped");

        let timeout = Duration::from_secs(self.timeout_seconds);
        let (status, stdout_buf, stderr_buf, stdout_truncated) =
            tokio::time::timeout(timeout, async {
                let (stdout_result, stderr_result) = futures::future::join(
                    read_capped(stdout, MAX_PIPE_BYTES),
                    read_capped(stderr, MAX_PIPE_BYTES),
                )
                .await;

                let (stdout_buf, stdout_truncated) =
                    stdout_result.map_err(|source| ExternalCommandError::Read {
                        program: self.command.clone(),
                        source,
                    })?;

                // Don't fail just because we couldn't capture the error message
                let (stderr_buf, _) = stderr_result.unwrap_or_default();

                let status = child
                    .wait()
                    .await
                    .map_err(|source| ExternalCommandError::Read {
                        program: self.command.clone(),
                        source,
                    })?;

                Ok::<_, ExternalCommandError>((status, stdout_buf, stderr_buf, stdout_truncated))
            })
            .await
            .map_err(|_| ExternalCommandError::Timeout {
                program: self.command.clone(),
                timeout,
            })??;

        if !status.success() {
            return Err(ExternalCommandError::ExitStatus {
                program: self.command.clone(),
                status: status.to_string(),
                stderr: String::from_utf8_lossy(&stderr_buf).trim().to_string(),
            });
        }

        if stdout_truncated {
            return Err(ExternalCommandError::OutputTooLarge {
                program: self.command.clone(),
                limit: MAX_PIPE_BYTES,
            });
        }

        Ok(stdout_buf)
    }
}

/// Returns at most `cap` bytes and whether anything was discarded.
async fn read_capped(
    mut reader: impl tokio::io::AsyncRead + Unpin,
    cap: usize,
) -> std::io::Result<(Vec<u8>, bool)> {
    let mut retained = Vec::new();

    // Heap-allocated rather than a stack array: this buffer lives across an await point, so an
    // inline one would be carried in the future of everything that requests a token.
    let mut chunk = vec![0_u8; 1024];
    let mut truncated = false;

    loop {
        let read = reader.read(&mut chunk).await?;
        if read == 0 {
            return Ok((retained, truncated));
        }

        let room = cap - retained.len();
        if read > room {
            truncated = true;
        }

        retained.extend_from_slice(&chunk[..read.min(room)]);
    }
}

/// Errors that can occur while getting an access token from an [`ExternallyManagedCredential`].
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum ExternalCommandError {
    /// The program could not be started.
    #[error("failed to run {program:?}: {source}")]
    Spawn {
        /// The program that could not be started.
        program: PathBuf,
        /// The underlying error.
        source: std::io::Error,
    },
    /// The program's output could not be read.
    #[error("failed to read the output of {program:?}: {source}")]
    Read {
        /// The program whose output could not be read.
        program: PathBuf,
        /// The underlying error.
        source: std::io::Error,
    },
    /// The program did not finish within its timeout and was killed.
    #[error("{program:?} did not produce an access token within {timeout:?}")]
    Timeout {
        /// The program that timed out.
        program: PathBuf,
        /// The timeout that elapsed.
        timeout: Duration,
    },
    /// The program exited unsuccessfully.
    #[error("{program:?} failed with {status}: {stderr}")]
    ExitStatus {
        /// The program that failed.
        program: PathBuf,
        /// The exit status it reported.
        status: String,
        /// What it wrote to stderr, truncated.
        stderr: String,
    },
    /// The program wrote more to stdout than an access token could plausibly need.
    #[error("{program:?} wrote more than {limit} bytes to stdout")]
    OutputTooLarge {
        /// The program that wrote too much.
        program: PathBuf,
        /// The limit it exceeded.
        limit: usize,
    },
    /// The program's output was not valid UTF-8.
    #[error("{program:?} did not write a valid UTF-8 access token to stdout")]
    InvalidUtf8 {
        /// The program with invalid output.
        program: PathBuf,
    },
    /// The program wrote nothing to stdout.
    #[error("{program:?} did not write an access token to stdout")]
    EmptyOutput {
        /// The program that wrote nothing.
        program: PathBuf,
    },
}

#[cfg(test)]
pub(super) use tests::shell;

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::{DEFAULT_TIMEOUT_SECONDS, ExternalCommandError, ExternallyManagedCredential};

    /// An [`ExternallyManagedCredential`] that runs `command` with no arguments and default
    /// settings.
    fn credential(command: impl Into<PathBuf>) -> ExternallyManagedCredential {
        ExternallyManagedCredential {
            command: command.into(),
            args: Vec::new(),
            timeout_seconds: DEFAULT_TIMEOUT_SECONDS,
        }
    }

    /// The absolute path of a program that can run a one-line script, plus the flag that
    /// introduces it. Tests avoid depending on any tool that isn't part of a base install.
    pub(in super::super) fn shell() -> (PathBuf, &'static str) {
        #[cfg(windows)]
        {
            let comspec = std::env::var_os("COMSPEC")
                .unwrap_or_else(|| r"C:\Windows\System32\cmd.exe".into());
            (PathBuf::from(comspec), "/C")
        }
        #[cfg(not(windows))]
        {
            (PathBuf::from("/bin/sh"), "-c")
        }
    }

    /// A credential whose program runs `script` through the platform shell.
    fn script_credential(script: &str) -> ExternallyManagedCredential {
        let (program, flag) = shell();
        ExternallyManagedCredential {
            args: vec![flag.to_string(), script.to_string()],
            ..credential(program)
        }
    }

    /// A credential that prints `token` and exits successfully.
    fn echo_credential(token: &str) -> ExternallyManagedCredential {
        #[cfg(windows)]
        let script = format!("echo {token}");
        #[cfg(not(windows))]
        let script = format!("printf '%s\\n' '{token}'");
        script_credential(&script)
    }

    #[tokio::test]
    async fn returns_trimmed_stdout_as_the_access_token() {
        let token = echo_credential("an-access-token")
            .request_access_token()
            .await
            .expect("the command should produce a token");

        assert_eq!(token.secret(), "an-access-token");
    }

    #[tokio::test]
    async fn reports_stderr_when_the_command_fails() {
        #[cfg(windows)]
        let script = "echo something went wrong 1>&2 && exit 3";
        #[cfg(not(windows))]
        let script = "echo 'something went wrong' >&2; exit 3";

        let error = script_credential(script)
            .request_access_token()
            .await
            .expect_err("a failing command should be an error");

        let message = error.to_string();
        assert!(
            message.contains("something went wrong"),
            "stderr should be reported: {message}"
        );
    }

    #[tokio::test]
    async fn times_out_a_command_that_hangs() {
        #[cfg(windows)]
        let script = "ping -n 30 127.0.0.1 > nul";
        #[cfg(not(windows))]
        let script = "sleep 30";

        let error = ExternallyManagedCredential {
            timeout_seconds: 1,
            ..script_credential(script)
        }
        .request_access_token()
        .await
        .expect_err("a hanging command should time out");

        assert!(
            matches!(error, ExternalCommandError::Timeout { .. }),
            "unexpected error: {error}"
        );
    }

    #[tokio::test]
    async fn rejects_empty_output() {
        let error = script_credential("exit 0")
            .request_access_token()
            .await
            .expect_err("a command that prints nothing should be an error");

        assert!(
            matches!(error, ExternalCommandError::EmptyOutput { .. }),
            "unexpected error: {error}"
        );
    }

    #[tokio::test]
    async fn rejects_output_that_is_too_large() {
        #[cfg(windows)]
        let script = "for /L %i in (1,1,20000) do @echo aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        #[cfg(not(windows))]
        let script = "yes aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa | head -c 200000";

        let error = script_credential(script)
            .request_access_token()
            .await
            .expect_err("an oversized output should be an error");

        assert!(
            matches!(error, ExternalCommandError::OutputTooLarge { .. }),
            "unexpected error: {error}"
        );
    }
}
