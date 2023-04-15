# syntax = docker/dockerfile:1.4
FROM rust:1.68.2-alpine as build
COPY . /app
WORKDIR /app/server
RUN apk add musl-dev
RUN --mount=type=cache,target=/usr/local/cargo/registry --mount=type=cache,target=/app/server/target cargo build --release
RUN ls -alh  /app/server/target
RUN ls -alh  /app/server/target/release


FROM scratch as prod
COPY --from=build /app/server/target/release/server /server
EXPOSE 8080
ENTRYPOINT ["/server"]