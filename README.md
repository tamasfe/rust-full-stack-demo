A full-stack Rust application example that uses gRPC for communication between the server and client. Most code is written in Rust and some glue code in protobuf files.

If I wanted to entirely use Rust for web development, I would do something like this setup. Which I would not do just yet due to the lacking front-end tooling.

To start the server, simply run:

```sh
cargo run -p api
```

To build and start start the client, I used [`trunk`](https://trunkrs.dev/):

```
cd crates/frontend && trunk serve
```
