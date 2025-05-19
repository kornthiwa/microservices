# ===== STAGE 1: Build Rust binary =====
FROM rust:1.82.0 as builder

WORKDIR /app
COPY rust-services ./rust-services
WORKDIR /app/rust-services
RUN cargo build --release

# ===== STAGE 2: Minimal image to run =====
FROM debian:buster-slim

# นำ binary มาใส่ใน image สุดท้าย
COPY --from=builder /app/rust-services/target/release/rust-services /usr/local/bin/rust-services

# รัน binary เมื่อ container start
CMD ["rust-services"]
