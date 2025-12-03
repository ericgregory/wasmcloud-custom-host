# syntax=docker/dockerfile:1-labs

FROM cgr.dev/chainguard/rust:latest-dev AS builder
WORKDIR /src
ENV RUST_BACKTRACE=1

# tools
USER root
RUN apk --no-cache add protoc protobuf protobuf-dev
USER nonroot

# dependencies cache
COPY Cargo.toml Cargo.lock ./
RUN cargo fetch

# copy source code
COPY . .

# build static binary
RUN cargo build --release --bin wasmcloud-custom-host

# Release image
FROM cgr.dev/chainguard/wolfi-base
RUN apk add --no-cache git
COPY --from=builder /src/target/release/wasmcloud-custom-host /usr/local/bin/wasmcloud-custom-host
ENTRYPOINT ["/usr/local/bin/wasmcloud-custom-host"]