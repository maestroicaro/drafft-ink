# Build stage
FROM rust:latest-bookworm as builder

# Install dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    build-essential \
    cmake \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the project
COPY . .

# Build the server
RUN cargo build --release -p drafftink-server

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from builder
COPY --from=builder /app/target/release/drafftink-server /app/drafftink-server

# Expose websocket port
EXPOSE 3030

# Run the server
CMD ["/app/drafftink-server"]
