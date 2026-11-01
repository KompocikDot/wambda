FROM rust:alpine AS build
RUN apk add --no-cache musl-dev
WORKDIR /src
COPY Cargo.toml ./
COPY core core
COPY native native
# guest and attacks are not needed for the container image
RUN sed -i -e '/default-members/d' -e 's/, "guest", "attacks"//' Cargo.toml && cargo build --release -p native

FROM alpine:3.22
COPY --from=build /src/target/release/native /app
EXPOSE 8080
USER 65534
ENTRYPOINT ["/app"]
