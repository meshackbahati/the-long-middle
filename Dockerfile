# Use the official Rust image
FROM rust:1.75-slim-bookworm

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libsqlite3-dev \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Install oxidite-cli version 2.3.1
# Note: This might take a few minutes as it compiles the CLI
RUN cargo install oxidite-cli --version 2.3.1

WORKDIR /usr/src/app

# Copy the web source
COPY web/ .

# Ensure the configuration is set to listen on 0.0.0.0 for Docker
RUN sed -i 's/host = "127.0.0.1"/host = "0.0.0.0"/g' oxidite.toml

# Expose the application port
EXPOSE 8080

# The user specified using 'oxidite serve' which builds a release and runs it.
# This command handles both the compilation and the execution of the server.
CMD ["oxidite", "serve", "--host", "0.0.0.0"]
