# Build stage
FROM rust:1.80-slim as builder

# Install required dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Create a new empty shell project
WORKDIR /usr/src/app

# Show what files are being copied
RUN echo "=== Copying source files ==="
COPY . .

# Show Rust and Cargo versions
RUN echo "=== Rust and Cargo versions ===" && \
    rustc --version && \
    cargo --version

# Show project structure
RUN echo "=== Project structure ===" && \
    ls -la && \
    echo "=== Cargo.toml content ===" && \
    cat Cargo.toml

# Build for release with verbose output
RUN echo "=== Starting build ===" && \
    cargo build --release --verbose

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Copy the build artifact from the builder stage
RUN echo "=== Copying build artifact ==="
COPY --from=builder /usr/src/app/target/release/rust-services /usr/local/bin/rust-services

# Verify the binary exists and show its info
RUN echo "=== Verifying binary ===" && \
    ls -lh /usr/local/bin/rust-services && \
    file /usr/local/bin/rust-services

# Set the startup command
CMD ["rust-services"]
