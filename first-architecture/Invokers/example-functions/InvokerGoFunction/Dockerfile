FROM rust:1.82.0 AS builder

WORKDIR /usr/src/myapp

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && echo "fn main() {}" > src/main.rs

RUN cargo build --release

COPY ./src ./src

RUN cargo build --release

#########################################################

FROM debian:bookworm-slim

#RUN apt-get update && apt-get install -y \
#    wget \
#    tar \
#    && rm -rf /var/lib/apt/lists/*

#ENV GO_VERSION=1.22.4

#RUN wget https://go.dev/dl/go${GO_VERSION}.linux-amd64.tar.gz && \
#    tar -C /usr/local -xzf go${GO_VERSION}.linux-amd64.tar.gz && \
#    rm go${GO_VERSION}.linux-amd64.tar.gz

#ENV PATH="/usr/local/go/bin:${PATH}"

COPY ./go/first-arch-go/functions/acl/handler_acl ./go/first-arch-go/functions/acl/handler_acl

COPY ./go/first-arch-go/functions/log/handler_log ./go/first-arch-go/functions/log/handler_log

COPY ./src/log4rs.yml .

COPY --from=builder /usr/src/myapp/target/release/InvokerFunction .

CMD ./InvokerFunction