# need alpine 3.15 for libpq-dev
FROM alpine:3.15 AS base

FROM base AS builder
WORKDIR /usr/src/ticx

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH

RUN apk update && apk add --no-cache openssl-dev alpine-sdk perl libpq-dev curl && apk update
RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain stable -y
RUN cargo -V
RUN rustc --version
RUN rustup -V

COPY . .

RUN cargo install --path .

FROM base
COPY --from=builder /usr/local/cargo/bin/ticx .
WORKDIR /app
ENTRYPOINT ["/app/ticx"]