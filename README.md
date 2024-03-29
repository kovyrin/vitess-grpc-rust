# gRPC Client for Vitess

[![Crates.io](https://img.shields.io/crates/v/vitess-grpc)](https://crates.io/crates/vitess-grpc)
[![Docs.rs](https://docs.rs/vitess-grpc/badge.svg)](https://docs.rs/vitess-grpc)
[![Tests](https://github.com/kovyrin/vitess-grpc-rust/actions/workflows/test.yml/badge.svg)](https://github.com/kovyrin/vitess-grpc-rust/actions/workflows/test.yml)
[![Checks](https://github.com/kovyrin/vitess-grpc-rust/actions/workflows/check.yml/badge.svg)](https://github.com/kovyrin/vitess-grpc-rust/actions/workflows/check.yml)

This is a Rust gRPC client for Vitess. It is generated from the Vitess proto files using
[tonic](https://github.com/hyperium/tonic).

## Usage

To use this crate, add the following to your Cargo.toml:

```toml
[dependencies]
vitess-grpc = "0.3"
tokio = { version = "1.0", features = ["full"] }
```

## Example

You can find an example of how to use this crate in the [examples](vitess-grpc/examples) directory.

Examples can be run with:

```shell
cargo run --example <example_name>
```

Available examples are:
* [vstream-consumer](vitess-grpc/examples/vstream.rs): A simple vstream consumer, which prints the events it receives.


Before running the examples, you will need to start Vitess locally. An easy way to do this is to use the `scripts/start-vttestserver` script, which will start a local Vitess cluster in Docker (or Podman) and configure a simple schema inside you can use for testing.

## Versioning

The version of the crate is independent of the version of Vitess, but contains a build metadata portion (see [SemVer documentation](https://semver.org/#spec-item-10) for details) that is set to the Vitess version the crate was generated from. For example, the version `0.3.0+vitess18.0.0` means that the crate was generated from the proto files included in Vitess v18.0.0.

## License

The files in the `proto` directory are copied from the [Vitess repository](https://github.com/vitessio/vitess/tree/main/proto) and are licensed under the Apache License, Version 2.0. See the
[Vitess LICENSE](https://github.com/vitessio/vitess/blob/main/LICENSE) file for more details.

The rest of the code in this repository is licensed under the MIT license. See the [LICENSE](LICENSE) file for more details.

## Vitess Update Process

When a new version of Vitess is released, the following steps should be taken to update this crate:

1. Update the `version` in vitess-grpc/Cargo.toml to the new Vitess version.
2. Run `script/update-protos` to update the proto files in the `vitess-grpc/proto` directory.
3. Run `cargo build` to make sure the crate builds.
4. Run `scripts/start-vttestserver` to start a local Vitess cluster used for testing.
5. Run `cargo test` to make sure the tests pass.
6. Stop the local Vitess cluster by running `scripts/stop-vttestserver`.
7. Commit the changes and push them to GitHub.
8. Change directory to `vitess-grpc`
9. Publish the crate to crates.io with `cargo publish`.
