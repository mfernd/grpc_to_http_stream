# gRPC to HTTP streaming examples

Examples to convert gRPC tonic stream to HTTP stream (SSE).

Snippets from frameworks like:

- [x] Axum: [`axum_server/src/handlers/sse.rs`](./src/axum_server/src/handlers/sse.rs)
- [ ] Actix?
- [ ] Rocket?
- [ ] Warp?
- [ ] Poem?

## How to run

### gRPC tonic server

```bash
cargo run --bin grpc_server
```

Will start the gRPC server on `localhost:50051` by default.

This can be changed with env variables `APP_HOST` and `APP_PORT`.

### Axum HTTP server

```bash
GRPC_URI="localhost:50051" cargo run --bin axum_server
```

Will start the gRPC server on `http://localhost:3000/` by default.

This can be changed with env variables `APP_HOST` and `APP_PORT`.
And you can change the gRPC server URI with the env variable `GRPC_URI`.

You can then test with curl:

```bash
curl http://localhost:3000/
```

## Notes

> [!NOTE]
> I didn't find solutions to handle errors when dealing with stream on the HTTP SSE side...
> If anyone has a solution for this, feel free to let me know.
