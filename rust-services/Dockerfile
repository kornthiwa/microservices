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

# Copy everything at once (simple and reliable)
# This ensures all files are available
COPY . .

# Debug: Verify files are copied
RUN echo "=== Debug: Listing files ===" && \
    ls -la && \
    echo "=== Checking for Cargo.toml ===" && \
    test -f Cargo.toml && echo "✓ Cargo.toml found" || (echo "✗ Cargo.toml NOT FOUND" && ls -la *.toml 2>/dev/null || echo "No .toml files found") && \
    echo "=== Checking for src directory ===" && \
    test -d src && echo "✓ src directory found" || echo "✗ src directory NOT FOUND"

# Build for release with optimizations
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
    useradd -r -g appuser appuser

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
