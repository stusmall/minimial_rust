# syntax = docker/dockerfile:1.4
FROM rust:1.71.1-alpine as build
COPY ["Cargo.toml", "Cargo.lock", "/app/"]
COPY ["src", "/app/src"]
WORKDIR /app/
RUN apk add musl-dev
RUN --mount=type=cache,target=/usr/local/cargo/registry  \
    --mount=type=cache,target=/app/target cargo build --release  \
    && cp /app/target/release/server /server


FROM scratch as prod
COPY --from=build /server /server
EXPOSE 8080
ENTRYPOINT ["/server"]