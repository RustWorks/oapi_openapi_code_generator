# oapi_generator

Generate code from OpenAPI specifications.

Forked from the original with the idea to make it into simple build-script crate as opposed to being a binary


To use it as a binary:
cargo run -- path-to-specification path-to-output

To use it inside a build script:
oapi_generator::generate_oapi_server_stubs(speification, destination)

The hosting crate should have the following crates inside its Cargo.toml:
TODO