FROM rust:1.81.0 AS builder

WORKDIR /

COPY ./src /src
COPY ./Cargo.toml /Cargo.toml
COPY ./Cargo.lock /Cargo.lock

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12 AS prod

USER nonroot

COPY --from=builder /target/release/promtail_cleaner /promtail_cleaner

ENTRYPOINT ["/promtail_cleaner"]
