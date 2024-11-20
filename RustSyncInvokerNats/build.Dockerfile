FROM rust:1.50 
WORKDIR /usr/src/myapp
RUN apt update && apt upgrade -y
COPY . .
RUN cargo install --path .
