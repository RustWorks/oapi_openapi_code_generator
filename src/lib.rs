mod helpers;
mod openapi_generator;

use anyhow::{Context, Result};
use std::path::Path;

use crate::openapi_generator::OpenApiGenerator;

pub fn generate_oapi_server_stubs(specification: impl AsRef<Path>,
    destination: impl AsRef<Path>) -> Result<()> {
    let mut openapi_generator =
        OpenApiGenerator::new(&specification).context(format!(
            "Cannot create OpenAPI generator with specifications at `{}`",
            specification.as_ref().to_string_lossy()
        ))?;
    openapi_generator
        .render(&destination)
        .context(format!(
            "Cannot render to `{}`",
            destination.as_ref().to_string_lossy()
        ))?;
    Ok(())
}