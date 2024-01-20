FROM rust:1.75.0 as builder
WORKDIR /usr/src/wordbot
COPY . .
RUN cargo install --path .

FROM rust:1.75.0-slim
COPY --from=builder /usr/local/cargo/bin/wordbot /usr/local/bin/wordbot
CMD ["wordbot"]
