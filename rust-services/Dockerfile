# ขั้นตอน Build
FROM rust:1.88-slim AS builder

# ติดตั้ง dependencies ที่จำเป็นสำหรับ build
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    build-essential \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# ตั้ง working directory
WORKDIR /usr/src/app

# Copy ทุกอย่างจาก rust-services directory
# Railway build context เป็น root ของ repo ดังนั้นไฟล์อยู่ใน rust-services/
COPY rust-services/ .

# Debug: ตรวจสอบว่าไฟล์ถูก copy หรือไม่ - แสดงทุกอย่าง
RUN echo "=== Debug: Current directory ===" && \
    pwd && \
    echo "=== Debug: Listing ALL files ===" && \
    ls -laR . 2>&1 | head -50 && \
    echo "=== Debug: Checking for Cargo.toml ===" && \
    find . -name "Cargo.toml" -type f 2>/dev/null || echo "Cargo.toml NOT FOUND" && \
    echo "=== Debug: Checking for src directory ===" && \
    find . -type d -name "src" 2>/dev/null || echo "src directory NOT FOUND" && \
    echo "=== Debug: All .toml files ===" && \
    find . -name "*.toml" -type f 2>/dev/null || echo "No .toml files found"

# Build สำหรับ release พร้อม optimizations
RUN cargo build --release && \
    strip target/release/rust-services

# ขั้นตอน Runtime - image ขนาดเล็ก
FROM debian:bookworm-slim

# ติดตั้งเฉพาะ runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get clean

# สร้าง non-root user เพื่อความปลอดภัย
RUN groupadd -r appuser && \
    useradd -r -g appuser appuser

# Copy binary ที่ build แล้วจาก builder stage
COPY --from=builder /usr/src/app/target/release/rust-services /usr/local/bin/rust-services

# ตั้ง ownership และ permissions
RUN chown appuser:appuser /usr/local/bin/rust-services && \
    chmod +x /usr/local/bin/rust-services

# เปลี่ยนเป็น non-root user
USER appuser

# ตั้ง working directory (optional - เพื่อความสอดคล้อง)
WORKDIR /home/appuser

# ตั้งคำสั่งเริ่มต้น
CMD ["rust-services"]
