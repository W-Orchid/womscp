FROM rust:latest

WORKDIR /server

COPY . /server

EXPOSE 3000

RUN cargo build --release
