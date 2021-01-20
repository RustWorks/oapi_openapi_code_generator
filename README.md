# oapi_generator

Generate code from OpenAPI specifications.

Forked from the original with the idea to make it into simple build-script crate as opposed to being a binary


## To use it as a binary:

```sh
cargo run -- path-to-specification path-to-output
```

## To use it inside a build script:

```rust
oapi_generator::generate_oapi_server_stubs(speification, destination)
```

The hosting crate should have the following crates inside its Cargo.toml:

```toml
reqwest = { version = "0.10.6", features = [ "json" ] } 
async-std = "1.6.0"
serde = { version = "1.0.111", features = [ "derive" ] }
serde_json = "1.0.53"
serde_urlencoded = "0.6.1"
actix-multipart = "0.2.0"
async-trait = "0.1.33"
url = "2.1.1"
thiserror = "1.0.19"
displaydoc = "0.1.6"
regex = "1.4.2"
lazy_static = "1.4.0"
```
