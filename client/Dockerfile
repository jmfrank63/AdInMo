# Define the Rust version as an argument
ARG RUST_VERSION=1.73

# Use the official Rust image as a builder
FROM rust:${RUST_VERSION} as builder

# Create a new empty shell project
RUN USER=root cargo new --bin client
WORKDIR /client

# Copy over your manifests
COPY ./Cargo.toml ./Cargo.toml

# Cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# Copy your source tree
COPY ./src ./src

# Build for release
RUN rm ./target/release/deps/client*
RUN cargo build --release

# Final stage
FROM ubuntu:jammy

# Set the environment variable for the server address and port
ENV SERVER_ADDR_PORT="server:3300"

# Update package lists and install ca-certificates
RUN apt-get update \
    && apt-get install -y curl \
    && rm -rf /var/lib/apt/lists/*

# Copy the build artifact from the builder stage
COPY --from=builder /client/target/release/client .

# Set the startup command to run your binary
CMD ["./client"]
