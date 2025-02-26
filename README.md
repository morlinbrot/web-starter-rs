# web-starter-rs

Starter template for a web application written in Rust, leveraging the "default" stack of [tokio](https://docs.rs/tokio), [axum](https://docs.rs/axum), and [sqlx](https://docs.rs/sqlx).

If you're looking to quickly spin up a solid, production-ready foundation of a scalable web application, this template is for you.

The template provides the necessary building blocks for a proper Rust web application, such as:

- Database connection pool provided via Axum state
- Database migrations using sqlx-cli
- Basic request tracing
- Crate set up as both lib and bin to facilitate e2e testing

## Usage

Make sure you have installed all the [prerequisites](#prerequisites).

Spin up the database:

```sh
just db
docker run -d --rm --name web-starter-db -p 5432:5432 --env-file .env postgres:17-alpine
```

Run the migrations:

```sh
just mig-up
sqlx migrate run
```

Run the server:

```sh
just watch
cargo watch -x run
```

Run tests:

```sh
just watch test
cargo watch -x test
```

## Prerequisites

- [just runner](https://github.com/casey/just) to run project-specific commands

```sh
cargo install just
```

- [cargo-watch](https://crates.io/crates/cargo-watch) to run the dev server in watch mode

```sh
cargo install cargo-watch --locked
```

- [sqlx-cli](https://crates.io/crates/sqlx-cli) to run migrations

```sh
cargo install sqlx-cli
```

- [docker](https://docs.docker.com/get-docker/) to run the database
