FROM rust:1.68-alpine3.17 AS builder

ARG TARGETARCH
ENV TARGETARCH=${TARGETARCH}
WORKDIR /build

COPY docker-target.sh .

ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse
ENV RUST_BACKTRACE=1

RUN apk add musl-dev git perl make gcc eudev-dev eudev pcsc-lite-dev pcsc-lite linux-headers
RUN ln -s "/usr/bin/$(sh ./docker-target.sh)-alpine-linux-musl-gcc" /usr/bin/musl-gcc
RUN rustup target add "$(sh ./docker-target.sh)-unknown-linux-musl"

COPY . .

ENV RUSTFLAGS=-Clinker=rust-lld
RUN cargo build --release --target "$(sh ./docker-target.sh)-unknown-linux-musl"

FROM alpine:3.17

WORKDIR /gfh
RUN apk add dumb-init

COPY --from=builder /build/target/*-unknown-linux-musl/release/gfh .
COPY --from=builder /build/target/*-unknown-linux-musl/release/gfh-keygen .

ENTRYPOINT [ "dumb-init", "--" ]
CMD [ "./gfh", "--version" ]