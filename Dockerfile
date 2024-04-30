FROM rust:latest AS builder

WORKDIR /usr/videos_processor

COPY . .

RUN apt update
RUN apt install -y protobuf-compiler

RUN cargo install --path .

FROM ubuntu:latest

COPY --from=builder /usr/local/cargo/bin/videos_processor /usr/local/bin/videos_processor

CMD ["videos_processor"]
