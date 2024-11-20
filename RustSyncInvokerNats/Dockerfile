FROM rust:1.82 as builder
WORKDIR /usr/src/myapp
COPY . .
RUN cargo install --path .
#RUN cargo build --release

FROM debian:bookworm-slim
#RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/invoker /usr/local/bin/invoker

ENV COMMAND="echo hello world"
CMD ["invoker"]