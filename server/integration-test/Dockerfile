# syntax=docker/dockerfile:1.4

FROM golang:1.21.0 AS build-stage
WORKDIR /
COPY test.go .
RUN CGO_ENABLED=0 GOOS=linux GOARCH=amd64 go build test.go


FROM scratch AS prod
WORKDIR /
COPY --from=build-stage /test /test
ENTRYPOINT ["/test"]
