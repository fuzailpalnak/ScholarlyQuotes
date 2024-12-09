# Start with the Rust slim image for building
FROM rust:1.77.2-slim AS builder

# Set the working directory
WORKDIR /app

# Install required dependencies
RUN apt-get update && apt-get install -y libsqlite3-dev

# Copy Cargo manifest files and fetch dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo fetch

# Copy the actual source code and build the application
COPY src ./src
RUN cargo build --release

# Use the same base image for runtime
FROM rust:1.77.2-slim AS runtime

# Install runtime SQLite dependency
RUN apt-get update && apt-get install -y libsqlite3-0 && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/scholarly_quotes ./scholarly_quotes

# Copy any additional resources
# COPY ./data ./data

# Set appropriate permissions
RUN mkdir -p /app/data/db && chmod 755 /app/data/db
COPY ./target/scholarlyQuotes.db /app/data/db/scholarlyQuotes.db

COPY src/.env /app/data/.env
ENV ENV_PATH=/app/data/.env

EXPOSE 8080:8080

# Command to run the application
CMD ["./scholarly_quotes"]
