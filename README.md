# Rust gRPC Framework

A gRPC framework built upon Tonic using the Rust language.

## Motivation

This project aims to create ergonomic abstractions comparable to frameworks in
other languages while attempting to maintain the performance benefits of Rust.

## Features

- [X] Tonic Server
- [ ] Multi-Database Support (CockroachDB, Postgres, MySQL, Sqlite)
- [ ] JWT Support
- [ ] Async Caching Layer with a Simple API
- [X] .env for Local Development
- [X] Integrated Application State with a Simple API
- [X] Lazy Static Config struct
- [X] Built-in Healthcheck
- [ ] Listeners configured for TDD
- [X] Custom Errors
- [ ] Secure Argon2i Password Hashing
- [ ] Unit and Integration Tests
- [X] Test Coverage Reports
- [X] Dockerfile for Running the Server in a Container
- [ ] TravisCI Integration

## Featured Packages

- `Argon2i`: Argon2i Password Hasning
- `dotenv`: Configuration Loader (.env)
- `envy`: Deserializes Environment Variables into a Config Struct
- `kcov`: Coverage Analysis

## Quick Installation

You can skip the first portion and jump ahead to the `TBD` section of this setup by copying the skeleton code in the `/examples` folder.

## Installation

First, create a new project:

```shell
cargo new grpc_server --bin
```

Next, cd into the `grpc_server` folder and add the following to Cargo.toml:

```toml
[package]
name = "grpc_server"
version = "0.1.0"
authors = ["YOUR NAME <yourname@yourdomain.com>"]
edition = "2018"

[dependencies]
grpc-framework = { path = "../" }
dotenv = "0.14"
listenfd = "0.3"
log = "0.4"
pretty_env_logger = "0.4.0"
tokio = { version = "1.0", features = ["macros", "rt-multi-thread"] }
```

With that setup in place, you can add in the server code in `/src/main.rs`:

```rust
use grpc_framework::error::Result;
use grpc_framework::server::serve;
use grpc_framework::state::State;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    // we're storing String values in state, but it can be anything that
    // fits in a hashmap
    let state = State::<String>::new().await?;
    serve(state).await?;

    Ok(())
}
```

Create an .env file at the root of your project:

```shell
touch .env
```

Now add environment values for local development:

```ini
SERVER=[::1]:50051
```

**IMPORTANT:** Change .env values for your setup.

## Running the Server

To startup the server:

```shell
RUST_LOG=info cargo run
```
