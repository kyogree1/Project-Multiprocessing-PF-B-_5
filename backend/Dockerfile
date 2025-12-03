FROM rust:1.75 as builder

WORKDIR /app

# Copy Cargo files
COPY Cargo.toml ./

# Copy source code
COPY src ./src

# Build the application
RUN cargo build --release --bin main_api

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary from builder
COPY --from=builder /app/target/release/main_api /app/main_api

# Create data directories
RUN mkdir -p /app/data/uploads /app/data/compressed

EXPOSE 3000

CMD ["/app/main_api"]
