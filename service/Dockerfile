# Define the Rust version as an argument
ARG RUST_VERSION=1.73

# Use the official Rust image as a builder
FROM rust:${RUST_VERSION} as builder

# Create a new empty shell project for caching dependencies
WORKDIR /usr/src/adinmo
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

RUN USER=root cargo new --bin server
RUN USER=root cargo new --bin service
RUN USER=root cargo new --bin database

COPY ./server/Cargo.toml ./server/Cargo.toml
COPY ./service/Cargo.toml ./service/Cargo.toml
COPY ./database/Cargo.toml ./database/Cargo.toml

RUN cargo build --release
# Copy the actual source tree
COPY . .

# Build for release
RUN cargo build --release --bin service

# Final stage
FROM ubuntu:jammy

ENV DATABASE_URL=mariadb://httpbin-user:aabb@mariadb:3306/httpbin-db
ENV APP_LOG_LEVEL=debug
ENV APP_LOG_STYLE=auto

# Update package lists and install ca-certificates
RUN apt-get update \
    && apt-get install -y ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the build artifact from the builder stage
COPY --from=builder /usr/src/adinmo/target/release/service /usr/local/bin/service

# Set the startup command to run your binary
CMD ["service"]
