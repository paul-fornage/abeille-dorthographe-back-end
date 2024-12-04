FROM rust:1.82.0 AS builder
LABEL authors="pfornage"

WORKDIR /usr/src/abeille-dorthographe

# Copy the Cargo.toml and Cargo.lock files to the container
COPY Cargo.toml Cargo.lock ./

# Copy the source tree as a dummy to trigger a cargo fetch
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Pre-fetch the dependencies
RUN cargo fetch

# Copy the source tree
COPY . .

# Build the application in release mode
RUN cargo build --release


FROM debian:latest

WORKDIR /usr/src/abeille-dorthographe

RUN apt-get update && apt-get install -y \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Copy executable from the builder image
COPY --from=builder /usr/src/abeille-dorthographe/target/release/abeille-dorthographe .
COPY Rocket.toml .
COPY .env .
COPY languages/ ./languages/

# Set the entry point
EXPOSE 8000
CMD ["./abeille-dorthographe"]
