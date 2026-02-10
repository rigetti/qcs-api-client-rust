//! This binary is used to generate Python stub files (type hints).
//! For more information on why this exists as a separate binary rather than a build script,
//! see the [`pyo3-stub-gen`][] documentation.
//!
//! [`pyo3-stub-gen`]: https://github.com/Jij-Inc/pyo3-stub-gen

#[cfg(not(feature = "stubs"))]
fn main() {
    eprintln!("Executing this binary only makes sense with the --stubs feature enabled.");
}

#[cfg(feature = "stubs")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stub = qcs_api_client_common::stub_info()?;
    rigetti_pyo3::stubs::sort(&mut stub);
    stub.generate()?;
    Ok(())
}
