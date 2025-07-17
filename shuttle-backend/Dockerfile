# Dockerfile for DynaVest Backend
# Alternative deployment option to Shuttle.dev

FROM rust:1.82 as builder

WORKDIR /app

# Copy standalone manifests
COPY Cargo-standalone.toml Cargo.toml

# Create dummy Cargo.lock for caching
RUN cargo fetch

# Copy source code
COPY src ./src

# Build the standalone application
RUN cargo build --release --bin dynavest-backend

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder /app/target/release/dynavest-backend /app/dynavest-backend

# Expose port
EXPOSE 8000

# Set environment variables
ENV RUST_LOG=info
ENV PORT=8000

# Run the binary
CMD ["./dynavest-backend"]