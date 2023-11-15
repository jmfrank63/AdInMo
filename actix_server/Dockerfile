# Define the Rust version as an argument
ARG RUST_VERSION=1.73

# Use the official Rust image as a builder
FROM rust:${RUST_VERSION} as builder

# Set the environment variable for the server address and port
ENV SERVER_ADDR_PORT="0.0.0.0:3300"

# Create a new empty shell project
RUN USER=root cargo new --bin actix_server
WORKDIR /actix_server

# Copy over your manifests
COPY ./Cargo.toml ./Cargo.toml

# Cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# Copy your source tree
COPY ./src ./src

# Build for release
RUN rm ./target/release/deps/actix_server*
RUN cargo build --release

# Final stage
FROM ubuntu:jammy

# Copy the build artifact from the builder stage
COPY --from=builder /actix_server/target/release/actix_server .

# Set the startup command to run your binary
CMD ["./actix_server"]
