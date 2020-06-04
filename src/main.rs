mod helpers;
mod openapi_generator;

use crate::openapi_generator::OpenApiGenerator;
use anyhow::{Context, Result};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "openapi_generator",
    about = "Generate code from OpenAPI specifications"
)]
struct Cli {
    /// Path of the OpenAPI specification file to use for generation
    openapi: PathBuf,
    #[structopt(short = "d", long = "dest", default_value = "output")]
    /// Destination of the generated code
    destination: PathBuf,
}

fn main() -> Result<()> {
    pretty_env_logger::init();
    let args = Cli::from_args();
    let mut openapi_generator =
        OpenApiGenerator::new(&args.openapi).context(format!(
            "Cannot create OpenAPI generator with specifications at `{}`",
            args.openapi.to_string_lossy()
        ))?;
    openapi_generator
        .render(&args.destination.clone())
        .context(format!(
            "Cannot render to `{}`",
            args.destination.to_string_lossy()
        ))?;
    Ok(())
}
