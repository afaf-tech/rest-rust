# =============================================================================
# Stage 1: Development
# =============================================================================
FROM rust:1.75-bookworm AS development

WORKDIR /app

# Install development dependencies
RUN apt-get update && apt-get install -y \
    postgresql-client \
    && rm -rf /var/lib/apt/lists/*

# Install cargo-watch for hot reload and sqlx-cli for migrations
RUN cargo install cargo-watch sqlx-cli --no-default-features --features rustls,postgres

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Create dummy main.rs to cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies (this layer will be cached)
RUN cargo build && rm -rf src

# Copy source code
COPY . .

# Development command (can be overridden)
CMD ["cargo", "run", "--", "server"]

# =============================================================================
# Stage 2: Builder
# =============================================================================
FROM rust:1.75-bookworm AS builder

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Create dummy main.rs to cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies
RUN cargo build --release && rm -rf src

# Copy source code
COPY . .

# Build the application
RUN cargo build --release

# =============================================================================
# Stage 3: Runtime
# =============================================================================
FROM debian:bookworm-slim AS runtime

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user for security
RUN groupadd --gid 1000 appgroup && \
    useradd --uid 1000 --gid appgroup --shell /bin/bash --create-home appuser

# Create necessary directories
RUN mkdir -p /app/var/log && chown -R appuser:appgroup /app

# Copy the binary from builder stage
COPY --from=builder /app/target/release/afaf-rest-rust /app/afaf-rest-rust

# Copy migrations for runtime migration support
COPY --from=builder /app/migrations /app/migrations

# Switch to non-root user
USER appuser

# Expose the application port
EXPOSE 9000

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:9000/health || exit 1

# Run the application
CMD ["/app/afaf-rest-rust", "server"]
