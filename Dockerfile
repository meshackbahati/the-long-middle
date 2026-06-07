FROM rust:1.96-bookworm

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libsqlite3-dev \
    curl \
    && rm -rf /var/lib/apt/lists/*

ENV CARGO_NET_RETRY=10
ENV CARGO_REGISTRY_SPARSE=false

# Create structure to match "../../rust-oxidite" relative path
WORKDIR /usr/src
COPY rust-oxidite ./rust-oxidite
COPY web ./the-long-middle/web

WORKDIR /usr/src/the-long-middle/web

RUN sed -i 's/host = "127.0.0.1"/host = "0.0.0.0"/g' oxidite.toml

RUN cargo build --release

EXPOSE 8080

ENV OXIDITE_ENV=production
ENV SERVER_HOST=0.0.0.0
ENV SERVER_PORT=8080

CMD ["./target/release/the-long-middle"]
