# Build stage
FROM rust:1.88-slim AS builder

# Install required dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    build-essential \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /usr/src/app

# Copy all files
COPY . .

# Build for release with optimizations
RUN cargo build --release && \
    strip target/release/rust-services

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies only
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get clean

# Create non-root user for security
RUN groupadd -r appuser && useradd -r -g appuser appuser

# Copy the build artifact from the builder stage
COPY --from=builder /usr/src/app/target/release/rust-services /usr/local/bin/rust-services

# Set ownership and permissions
RUN chown appuser:appuser /usr/local/bin/rust-services && \
    chmod +x /usr/local/bin/rust-services

# Switch to non-root user
USER appuser

# Set the startup command
CMD ["rust-services"]
