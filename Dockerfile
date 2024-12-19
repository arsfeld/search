FROM rust:1.82-slim as builder

WORKDIR /usr/src/

RUN apt-get update -y && \
  apt-get install -y pkg-config make g++ libssl-dev && \
  rustup target add x86_64-unknown-linux-gnu

COPY . .

RUN cargo build --release

# We do not need the Rust toolchain to run the binary!
FROM debian:bookworm-slim AS runtime

RUN set -xe && \
    apt-get update && \
    apt-get install -y --no-install-recommends openssl && \
    apt-get autoremove -y && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/* /usr/share/man/* /usr/share/doc/*

WORKDIR /usr/app

COPY --from=builder /usr/src/assets/static /usr/app/assets/static
COPY --from=builder /usr/src/assets/static/404.html /usr/app/assets/static/404.html
COPY --from=builder /usr/src/config /usr/app/config
COPY --from=builder /usr/src/target/release/search-cli /usr/app/search-cli

ENTRYPOINT ["/usr/app/search-cli" "start" "-e" "production"]
