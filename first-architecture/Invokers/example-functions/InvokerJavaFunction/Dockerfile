FROM rust:1.82.0 AS builder

WORKDIR /usr/src/myapp

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && echo "fn main() {}" > src/main.rs

RUN cargo build --release

COPY ./src ./src

RUN cargo build --release

#########################################################

FROM debian:bookworm-slim

RUN apt-get update && apt-get install default-jre -y

COPY ./java/first-arch-java/build/libs/handler_acl-1.0-SNAPSHOT.jar ./java/first-arch-java/build/libs/handler_acl-1.0-SNAPSHOT.jar

COPY ./java/first-arch-java/build/libs/handler_log-1.0-SNAPSHOT.jar ./java/first-arch-java/build/libs/handler_log-1.0-SNAPSHOT.jar

COPY ./java/first-arch-java/build/libs/handler_hello-1.0-SNAPSHOT.jar ./java/first-arch-java/build/libs/handler_hello-1.0-SNAPSHOT.jar

COPY ./src/log4rs.yml .

COPY --from=builder /usr/src/myapp/target/release/InvokerFunction .

CMD ./InvokerFunction
