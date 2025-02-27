FROM rust:1.82.0 AS builder

WORKDIR /usr/src/myapp

COPY Cargo.toml Cargo.lock ./

COPY ./src ./src

RUN cargo build --release

#########################################################

FROM debian:bookworm-slim

COPY --from=builder /usr/src/myapp/target/release/http-trigger .

CMD ["./http-trigger"]