# Use a more recent and full Rust image for better compatibility and toolchain stability
FROM rust:1.80-bookworm

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libsqlite3-dev \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Set Cargo environment variables to improve reliability in Docker builds
ENV CARGO_NET_RETRY=10
ENV CARGO_REGISTRY_SPARSE=false

# Install oxidite-cli version 2.3.1
# Using --locked can help if the crate has a checked-in Cargo.lock
RUN cargo install oxidite-cli --version 2.3.1 || \
    (cargo install oxidite-cli --version 2.3.1 --registry crates-io)

WORKDIR /usr/src/app

# Copy the web source
COPY . .

# Ensure the configuration is set to listen on 0.0.0.0 for Docker
RUN sed -i 's/host = "127.0.0.1"/host = "0.0.0.0"/g' oxidite.toml

# Expose the application port
EXPOSE 8080

# Environment variables for Oxidite
ENV OXIDITE_ENV=production
ENV SERVER_HOST=0.0.0.0
ENV SERVER_PORT=8080

# The user specified using 'oxidite serve' which builds a release and runs it.
CMD ["oxidite", "serve", "--host", "0.0.0.0", "--port", "8080"]
