FROM rust:1.93.0 as builder
ARG URBAN_DICTIONARY_KEY
ENV URBAN_DICTIONARY_KEY=$URBAN_DICTIONARY_KEY
WORKDIR /usr/src/wordbot
COPY . .
RUN cargo install --path .

FROM rust:1.93.0-slim
COPY --from=builder /usr/local/cargo/bin/wordbot /usr/local/bin/wordbot
CMD ["wordbot"]
