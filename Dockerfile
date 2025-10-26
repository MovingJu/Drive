FROM rust:1.80 AS compiletime

WORKDIR /app

COPY ./Cargo.lock ./Cargo.toml /app/
COPY ./src /app/src

RUN cargo build --release -j 4

FROM debian:bookworm-slim

WORKDIR /app

COPY --from=compiletime /app/target/release/e /app/e

CMD ["/app/e"]