FROM rust:1.64.0 as builder
WORKDIR /usr/src/wordbot
COPY . .
RUN cargo install --path .
FROM debian:buster-slim
RUN apt-get update && \
    apt-get install -y openssl ca-certificates libcrypto++-dev libcrypto++6 libssl-dev libssl-doc && \
    rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/wordbot /usr/local/bin/wordbot
CMD ["wordbot"]
