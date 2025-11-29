# Build stage
FROM rust:1.88-slim AS builder

# Install required build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    build-essential \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /usr/src/app

# Copy dependency files first for better layer caching
# This allows Docker to cache the dependency compilation layer
COPY Cargo.toml Cargo.lock rust-toolchain.toml ./

# Create a dummy main.rs to build dependencies
# This step will be cached unless Cargo.toml changes
RUN mkdir -p src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy source code
# This layer will only rebuild when source code changes
COPY src ./src

# Build final binary with optimizations
# Dependencies are already compiled, so this is faster
RUN cargo build --release && \
    strip target/release/rust-services

# Runtime stage - minimal image
FROM debian:bookworm-slim

# Install only runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get clean

# Create non-root user for security best practices
RUN groupadd -r appuser && \
    useradd -r -g appuser -u 1000 appuser

# Copy the build artifact from the builder stage
COPY --from=builder /usr/src/app/target/release/rust-services /usr/local/bin/rust-services

# Set proper ownership and permissions
RUN chown appuser:appuser /usr/local/bin/rust-services && \
    chmod +x /usr/local/bin/rust-services

# Switch to non-root user
USER appuser

# Set working directory (optional, for consistency)
WORKDIR /home/appuser

# Health check (optional - uncomment if needed)
# HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
#   CMD pgrep rust-services || exit 1

# Set the startup command
CMD ["rust-services"]
