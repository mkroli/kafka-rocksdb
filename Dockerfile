FROM rust:1.49-buster as builder
RUN apt-get update && apt-get install -y cmake clang && rm -rf /var/lib/apt/lists/*
WORKDIR /src
COPY . .
RUN cargo install --path .
RUN strip /usr/local/cargo/bin/kafka-rocksdb

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/kafka-rocksdb /usr/bin/kafka-rocksdb
ENTRYPOINT ["/usr/bin/kafka-rocksdb"]
