FROM rust:1.82.0 AS builder

WORKDIR /usr/src/myapp

COPY Cargo.toml Cargo.lock ./

COPY ./src ./src

RUN cargo build --release

#########################################################

FROM debian:bookworm-slim

RUN apt update -y && apt install nodejs -y

COPY ./function ./function

COPY --from=builder /usr/src/myapp/target/release/invoker_dp .

CMD ./invoker_dp