FROM rust:1.70 as builder

WORKDIR /usr/src/metaverse
COPY . .

# Build with release optimizations
RUN cargo build --release

# Create minimal runtime image
FROM debian:bullseye-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the built binary
COPY --from=builder /usr/src/metaverse/target/release/metaverse /usr/local/bin/

# Create data directory
RUN mkdir -p /data/metaverse

# Set environment variables
ENV RUST_LOG=info
ENV METAVERSE_DATA=/data/metaverse

# Expose ports
EXPOSE 8545 30303

# Set entrypoint
ENTRYPOINT ["metaverse"]
