Code that is shared between the client and the server using gRPC+protobuf.

The WASM client won't compile with some tonic features enabled (such as the default transport which expects sockets to be available), so these are toggled with the `server` feature. The code generation is done twice likewise so that both the client and the server can run at the same time, and the generated code won't overwrite each other. This could probably be solved in a nicer way.
