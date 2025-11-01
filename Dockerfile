# Multi-stage Dockerfile for Rust Coins Application
# Stage 1: Builder - Compile the Rust application
FROM rust:1.83 AS builder

# Create app directory
WORKDIR /usr/src/coins

# Copy manifests
COPY Cargo.toml ./

# Copy source code
COPY src ./src

# Build the application in release mode
RUN cargo build --release

# Run tests to ensure everything works
RUN cargo test --release

# Stage 2: Runtime - Create minimal runtime image
FROM debian:bookworm-slim

# Install minimal runtime dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Create a non-root user
RUN useradd -m -u 1000 coins && \
    mkdir -p /home/coins/app && \
    chown -R coins:coins /home/coins

# Set working directory
WORKDIR /home/coins/app

# Copy the binary from builder
COPY --from=builder /usr/src/coins/target/release/coins .

# Change ownership
RUN chown coins:coins coins

# Switch to non-root user
USER coins

# Set the entrypoint
ENTRYPOINT ["./coins"]