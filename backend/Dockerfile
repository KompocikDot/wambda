FROM rust:1-slim-trixie AS builder

RUN apt-get update && apt-get install -y --no-install-recommends pkg-config libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /build

# Build dependencies first so they are cached until Cargo.toml/Cargo.lock change.
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs \
    && cargo build --release \
    && rm -rf src target/release/deps/wambda* target/release/wambda*

COPY src ./src
RUN cargo build --release

# cc (not static): the binary links glibc, libgcc_s and libssl (native-tls)
FROM gcr.io/distroless/cc-debian13:nonroot

COPY --from=builder /build/target/release/wambda /usr/local/bin/wambda
# static busybox only for `wget` in the compose healthcheck (distroless has no shell/curl)
COPY --from=busybox:stable-musl /bin/busybox /busybox

ENV BACKEND_ADDR=0.0.0.0:8000
EXPOSE 8000

ENTRYPOINT ["/usr/local/bin/wambda"]
