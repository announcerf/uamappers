FROM rust:1.93.0-slim-bookworm AS chef
WORKDIR /app
RUN apt-get update \
  && apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
  && rm -rf /var/lib/apt/lists/*
RUN cargo install cargo-chef --version 0.1.71

FROM chef AS planner
COPY Cargo.toml /app/Cargo.toml
COPY Cargo.lock /app/Cargo.lock
COPY apps/api/Cargo.toml /app/apps/api/Cargo.toml
COPY apps/worker/Cargo.toml /app/apps/worker/Cargo.toml
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json /app/recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY Cargo.toml /app/Cargo.toml
COPY Cargo.lock /app/Cargo.lock
COPY apps/api/Cargo.toml /app/apps/api/Cargo.toml
COPY apps/api/src /app/apps/api/src
COPY apps/api/migrations /app/apps/api/migrations
COPY apps/worker/Cargo.toml /app/apps/worker/Cargo.toml
COPY apps/worker/src /app/apps/worker/src

RUN cargo build -p uamappers-api --release

FROM debian:bookworm-slim
RUN apt-get update \
  && apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
  && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/uamappers-api /usr/local/bin/uamappers-api
COPY --from=builder /app/target/release/uamappers-migrator /usr/local/bin/uamappers-migrator

EXPOSE 8080
CMD ["/usr/local/bin/uamappers-api"]
