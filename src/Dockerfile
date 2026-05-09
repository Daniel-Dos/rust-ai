# Build stage
FROM rust:1.95 as builder

WORKDIR /app

COPY Cargo.toml .
COPY Cargo.lock .
COPY src ./src

RUN apt-get update && apt-get install -y pkg-config libssl-dev

RUN cargo build --release --bin main
RUN cargo build --release --bin consumer

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/main /usr/local/bin/main
COPY --from=builder /app/target/release/consumer /usr/local/bin/consumer

EXPOSE 8080

CMD ["main"]