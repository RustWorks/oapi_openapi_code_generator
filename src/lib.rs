mod helpers;
mod openapi_generator;

use anyhow::{Context, Result};
use std::path::Path;

use crate::openapi_generator::OpenApiGenerator;

pub fn generate_oapi_server_stubs(
    specification: impl AsRef<Path>,
    destination: impl AsRef<Path>,
) -> Result<()> {
    let mut openapi_generator = OpenApiGenerator::new(&specification).context(format!(
        "Cannot create OpenAPI generator with specifications at `{}`",
        specification.as_ref().to_string_lossy()
    ))?;
    openapi_generator.render(&destination).context(format!(
        "Cannot render to `{}`",
        destination.as_ref().to_string_lossy()
    ))?;

    log::info!("Running rustfmt on {}", &destination.as_ref().display());
    let fmt_result = std::process::Command::new("rustfmt")
        .arg("--emit")
        .arg("files")
        .arg("--edition")
        .arg("2018")
        .arg(destination.as_ref().to_str().unwrap())
        .output();

    if let Err(e) = fmt_result {
        log::error!("Failed running rustfmt on {}", e)
    }

    Ok(())
}
