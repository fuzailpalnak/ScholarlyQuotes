# Start with the Rust slim image for building
FROM rust:1.81.0-slim AS builder

# Set the working directory
WORKDIR /app

# Install required dependencies
RUN apt-get update && apt-get install -y libsqlite3-dev  && apt-get install -y pkg-config && apt install -y libssl-dev

# Copy Cargo manifest files and fetch dependencies
COPY Cargo.toml ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo fetch

# Copy the actual source code and build the application
COPY src ./src

RUN cargo build --release

# Use the same base image for runtime
FROM rust:1.81.0-slim AS runtime

# Set the working directory
WORKDIR /app

COPY .env /app/.env
ENV ENV_PATH=/app/.env

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/scholarly_quotes ./scholarly_quotes

EXPOSE 8080:8080

# Command to run the application
CMD ["./scholarly_quotes"]
