FROM rust:alpine AS builder

RUN apk add --no-cache musl-dev

WORKDIR /build

COPY Cargo.toml Cargo.lock ./
COPY packages/backups packages/backups

RUN cargo build --release -p backups

FROM alpine:latest

COPY --from=builder /build/target/release/backups /usr/local/bin/backups

CMD ["/usr/local/bin/backups"]