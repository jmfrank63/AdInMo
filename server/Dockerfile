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
RUN cargo build --release --bin server

# Final stage
FROM ubuntu:jammy

# Set the environment variable for the server address and port
ENV SERVER_ADDR_PORT="0.0.0.0:3300"
ENV DATABASE_URL=mariadb://httpbin-user:aabb@mariadb:3306/httpbin-db
ENV ROOT_DATABASE_URL=mariadb://root:uuvv@mariadb:3306/mysql
ENV APP_LOG_LEVEL=debug
ENV APP_LOG_STYLE=auto

# Copy the build artifact from the builder stage
COPY --from=builder /usr/src/adinmo/target/release/server /usr/local/bin/server

# Set the startup command to run your binary
CMD ["server"]
