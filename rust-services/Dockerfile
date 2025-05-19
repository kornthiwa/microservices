# Stage 1: Build
FROM rust:1.82.0 AS builder

WORKDIR /usr/src/rust-services
COPY . .
RUN cargo build --release

# Stage 2: Runtime
FROM debian:bullseye-slim

WORKDIR /app

# คัดลอก binary จาก builder stage
COPY --from=builder /usr/src/rust-services/target/release/rust-services .

# ติดตั้ง curl สำหรับ healthcheck
RUN apt-get update && apt-get install -y curl && apt-get clean

# ระบุพอร์ตให้ Railway รู้
ENV PORT=4000
EXPOSE 4000

# Run binary
CMD ["./rust-services"]

# Health check
HEALTHCHECK CMD curl --fail http://localhost:4000/health || exit 1
