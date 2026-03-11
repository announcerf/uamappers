FROM rust:1.93.0-slim-bookworm AS chef
WORKDIR /app
# hadolint ignore=DL3008
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
COPY tests/Cargo.toml /app/tests/Cargo.toml
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
COPY tests /app/tests

RUN cargo build -p uamappers-worker --release

FROM debian:bookworm-slim
# hadolint ignore=DL3008
RUN apt-get update \
  && apt-get install -y --no-install-recommends \
  ca-certificates \
  && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/uamappers-worker /usr/local/bin/uamappers-worker

CMD ["/usr/local/bin/uamappers-worker"]
