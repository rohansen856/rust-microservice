# Stage 1: Base for Development
FROM rust:latest AS dev

# Set the working directory
WORKDIR /usr/src/app

# Install build tools and dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Cache Cargo dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo fetch && cargo build

# Copy the source code
COPY . .

EXPOSE 5050

# Default command for development
CMD ["cargo", "run"]

# # Stage 2: Base for Production
# FROM rust:latest AS builder

# WORKDIR /usr/src/app

# # Copy dependencies and source code
# COPY Cargo.toml Cargo.lock ./
# RUN mkdir src && echo "fn main() {}" > src/main.rs
# RUN cargo fetch

# COPY . .

# # Build the application
# RUN cargo build --release

# # Stage 3: Runtime
# FROM debian:buster-slim AS runtime

# # Install only runtime dependencies
# RUN apt-get update && apt-get install -y libssl1.1 ca-certificates && rm -rf /var/lib/apt/lists/*

# # Set working directory
# WORKDIR /usr/src/app

# # Copy the binary from the builder stage
# COPY --from=builder /usr/src/app/target/release/my_app /usr/bin/my_app

# # Command to run the production binary
# CMD ["my_app"]
