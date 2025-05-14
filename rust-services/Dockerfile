FROM rust:latest AS builder 

WORKDIR /usr/src/rust-services

COPY . .

RUN cargo build --release

CMD ["./target/release/rust-services"]

# Health check for Docker
HEALTHCHECK CMD curl --fail http://localhost:4000/health || exit 1
