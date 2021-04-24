FROM rust:1.51 AS builder

COPY ./ /app

WORKDIR /app

RUN cargo build --release

FROM rust:1.51-slim-buster AS buster-builder

COPY ./ /app

WORKDIR /app

RUN cargo build --release

FROM rust:1.51-alpine AS alpine-builder

COPY ./ /app

WORKDIR /app

RUN cargo build --release

# Size 117MB
FROM debian AS app

COPY --from=builder /app/target/release/cedict-parser /

ENTRYPOINT [ "./cedict-parser" ]

# Size 72.6MB
FROM debian:buster-slim AS app-buster-slim

COPY --from=buster-builder /app/target/release/cedict-parser /

ENTRYPOINT [ "./cedict-parser" ]

# Size 24.6MB
FROM gcr.io/distroless/cc AS app-distroless

COPY --from=builder /app/target/release/cedict-parser /

ENTRYPOINT [ "./cedict-parser" ]

# Size 9.48MB
FROM alpine AS app-alpine

COPY --from=alpine-builder /app/target/release/cedict-parser /

ENTRYPOINT [ "./cedict-parser" ]

# Size 3.37MB
FROM scratch AS app-scratch

COPY --from=alpine-builder /app/target/release/cedict-parser /

ENTRYPOINT [ "./cedict-parser" ]
