FROM rust:latest as builder
WORKDIR /usr/src/wisdom_client
COPY . .
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder /usr/src/wisdom_client/target/release/wisdom_client /usr/local/bin/wisdom_client
CMD ["wisdom_client"]
