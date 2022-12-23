FROM ghcr.io/evanrichter/cargo-fuzz as builder

ADD . /json_typegen
WORKDIR /json_typegen/fuzz
RUN cargo +nightly fuzz build 

FROM debian:bookworm
COPY --from=builder /json_typegen/fuzz/target/x86_64-unknown-linux-gnu/release/json-parser-fuzz /
COPY --from=builder /json_typegen/fuzz/target/x86_64-unknown-linux-gnu/release/json-codegen-fuzz /
