FROM lukemathwalker/cargo-chef:latest-rust-1.82-slim AS chef

RUN apt-get update -y && \
  apt-get install -y pkg-config make g++ libssl-dev && \
  rustup target add x86_64-unknown-linux-gnu

WORKDIR /usr/src/

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /usr/src/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin search-cli


# We do not need the Rust toolchain to run the binary!
FROM debian:bookworm-slim AS runtime

RUN set -xe && \
    apt-get update && \
    apt-get install -y --no-install-recommends openssl ca-certificates && \
    apt-get autoremove -y && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/* /usr/share/man/* /usr/share/doc/*

WORKDIR /usr/app

COPY --from=builder /usr/src/assets /usr/app/assets
COPY --from=builder /usr/src/config /usr/app/config
# Make sure we have seed data
COPY --from=builder /usr/src/src/fixtures /usr/app/src/fixtures
COPY --from=builder /usr/src/target/release/search-cli /usr/app/search-cli

VOLUME /usr/app/data

ENTRYPOINT ["/usr/app/search-cli", "start", "-e", "production", "-b", "0.0.0.0", "-s"]
