FROM rust:1.82-slim as builder

WORKDIR /usr/src/

RUN apt-get update -y && \
  apt-get install -y pkg-config make g++ libssl-dev && \
  rustup target add x86_64-unknown-linux-gnu

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /usr/app

COPY --from=builder /usr/src/assets/static /usr/app/assets/static
COPY --from=builder /usr/src/assets/static/404.html /usr/app/assets/static/404.html
COPY --from=builder /usr/src/config /usr/app/config
COPY --from=builder /usr/src/target/release/search-cli /usr/app/search-cli

ENTRYPOINT ["/usr/app/search-cli"]