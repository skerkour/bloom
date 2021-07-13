####################################################################################################
## Build server
####################################################################################################
FROM rust:latest AS builder_rust

RUN rustup update
RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev libssl-dev
RUN update-ca-certificates

# Create appuser
ENV USER=bloom
ENV UID=10001

# See https://stackoverflow.com/a/55757473/12429735RUN
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"


WORKDIR /bloom

COPY ./ .
WORKDIR /bloom/bloom

RUN make build_static
# RUN make build

####################################################################################################
## Build webapp, chatbox, bloom.js
####################################################################################################
FROM node:lts-alpine AS builder_node

RUN apk update && apk add --no-cache git make

# Build webapp
WORKDIR /bloom/webapp
COPY ./webapp .

RUN make install
RUN make build

# Build chatbox.js
WORKDIR /bloom/chatbox.js
COPY ./chatbox.js .

RUN make install
RUN make build

# Build bloom.js
WORKDIR /bloom/bloom.js
COPY ./bloom.js .

RUN make install
RUN make build


####################################################################################################
## Final image
####################################################################################################
# FROM scratch
FROM alpine:latest
# FROM debian:buster-slim

RUN apk update && apk add --no-cache ca-certificates
RUN update-ca-certificates

# Import from builder.
COPY --from=builder_rust /etc/passwd /etc/passwd
COPY --from=builder_rust /etc/group /etc/group

WORKDIR /bloom

# Copy our builds
COPY --from=builder_rust /bloom/bloom/dist/ ./
COPY --from=builder_node /bloom/webapp/dist/ ./public/
COPY --from=builder_node /bloom/chatbox.js/dist/chatbox.js ./public/libs/
COPY --from=builder_node /bloom/bloom.js/dist/bloom.js ./public/libs/


# Use an unprivileged user.
USER bloom:bloom

EXPOSE 8080 8443
CMD ["/bloom/bloom", "server", "--worker", "--scheduler"]

LABEL maintainer="Bloom <https://bloom.sh>"
LABEL homepage=https://bloom.sh
LABEL org.opencontainers.image.name=bloom
LABEL repository=https://github.com/skerkour/bloom
LABEL org.opencontainers.image.source = "https://github.com/skerkour/bloom"

# If some crashes or slowness are noticed when running the static rust binary with musl and Jemalloc
# see here: https://andygrove.io/2020/05/why-musl-extremely-slow/
