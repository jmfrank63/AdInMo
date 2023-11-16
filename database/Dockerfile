# Define the Rust version as an argument
ARG RUST_VERSION=1.73

# Use the official Rust image as a builder
FROM rust:${RUST_VERSION} as builder

# Create a new empty shell project
RUN USER=root cargo new --bin database
WORKDIR /database

# Copy over your manifests
COPY ./Cargo.toml ./Cargo.toml

# Cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# Copy your source tree
COPY ./src ./src

# Build for release
RUN cargo build --release

# Final stage
FROM ubuntu:jammy

# Copy the build artifact from the builder stage
COPY --from=builder /database/target/release/database .

# Set the startup command to run your binary
CMD ["./database"]
