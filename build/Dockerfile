# Use a multi-stage build for efficiency
# Stage 1: Rust Build
FROM rust:1.72 AS rust-builder
WORKDIR /app
COPY src/core /app/src/core
RUN cargo build --release --manifest-path /app/src/core/Cargo.toml

# Stage 2: Node.js Build
FROM node:20 AS node-builder
WORKDIR /app
COPY src/api /app/src/api
RUN npm install --prefix /app/src/api && npm run build --prefix /app/src/api

# Stage 3: Final Image
FROM debian:bullseye-slim
WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy Rust core
COPY --from=rust-builder /app/src/core/target/release/libshellmorph_core.a /app/core/

# Copy API build
COPY --from=node-builder /app/src/api/build /app/api/

# Add CLI binary (optional step)
COPY src/cli/build/shellmorph-cli /usr/local/bin/

# Default command
CMD ["node", "/app/api/server.js"]
