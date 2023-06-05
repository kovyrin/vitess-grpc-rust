# gRPC Client for Vitess

This is a Rust gRPC client for Vitess. It is generated from the Vitess proto files using
[tonic](https://github.com/hyperium/tonic).

## Usage

To use this crate, add the following to your Cargo.toml:

```toml
[dependencies]
vitess-grpc = "0.1"
tokio = { version = "1.0", features = ["full"] }
```

## Example

You can find an example of how to use this crate in the [examples](examples) directory.

Examples can be run with:

```shell
cargo run --example <example_name>
```

Available examples are:
* [vstream-consumer](examples/vstream.rs): A simple vstream consumer, which prints the events it receives.

## License

The files in the `proto` directory are copied from the [Vitess repository](https://github.com/vitessio/vitess/tree/main/proto) and are licensed under the Apache License, Version 2.0. See the
[Vitess LICENSE](https://github.com/vitessio/vitess/blob/main/LICENSE) file for more details.

The rest of the code in this repository is licensed under the MIT license. See the [LICENSE](LICENSE) file for more details.
