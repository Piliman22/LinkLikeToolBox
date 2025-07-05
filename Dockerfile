FROM rust:1.75-slim as builder

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY crates/ ./crates/
COPY src/ ./src/

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

RUN useradd -m -u 1000 linklike

WORKDIR /app

COPY --from=builder /app/target/release/link_like_tool_box /usr/local/bin/linklike

RUN mkdir -p /app/cache /app/masterdata /app/assets && \
    chown -R linklike:linklike /app

USER linklike

CMD ["linklike"]