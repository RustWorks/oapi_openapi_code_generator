use anyhow::Result;
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
    /// Destination of the generated code
    destination: PathBuf,
}

fn main() -> Result<()> {
    pretty_env_logger::init();
    let args = Cli::from_args();
    oapi_generator::generate_oapi_server_stubs(args.openapi, args.destination)
}
