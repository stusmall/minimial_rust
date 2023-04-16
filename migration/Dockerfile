# syntax = docker/dockerfile:1.4
FROM rust:1.68.2-alpine as build
COPY . /app
WORKDIR /app/
RUN apk add musl-dev
RUN --mount=type=cache,target=/usr/local/cargo/registry  \
    --mount=type=cache,target=/app/target cargo build --release  \
    && cp /app/target/release/migration /migration


FROM scratch as prod
COPY --from=build /migration /migration
EXPOSE 8080
ENTRYPOINT ["/migration"]