FROM rust:1.82.0-slim-bullseye

ARG APP_NAME=better-battlebit-api

WORKDIR /app

COPY ./ ./

RUN apt-get update -y && apt-get upgrade -y && apt-get install -y pkg-config libssl-dev
RUN cargo build --release

ENTRYPOINT ["./target/release/better-battlebit-api"]