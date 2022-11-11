# rust-grpc-smartcore

A Rust service to run `https://github.com/smartcorelib/smartcore` via RPC APIs.

* `sudo apt install protobuf-compiler` (PROTOC)
* `cargo build && cargo run --bin grpc-smartcore-server`
* in another terminal: `cargo run --bin grpc-smartcore-client`
* A Linear Regression using QR should be computed and printed back to the client