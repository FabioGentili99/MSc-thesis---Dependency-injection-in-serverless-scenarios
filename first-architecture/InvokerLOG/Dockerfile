FROM rust:1.82.0 AS builder

WORKDIR /usr/src/myapp

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && echo "fn main() {}" > src/main.rs

RUN cargo build --release

COPY ./src ./src

RUN cargo build --release

#########################################################

FROM debian:bookworm-slim

RUN apt update -y && apt install nodejs -y

COPY ./function ./function

COPY --from=builder /usr/src/myapp/target/release/invoker_log .

CMD ./invoker_log 