FROM rust:latest AS builder
WORKDIR /usr/src/rust-log-app
COPY Cargo.toml ./
RUN cargo fetch
COPY src ./src
RUN rustup target add x86_64-unknown-linux-musl && \
    cargo build --release --target x86_64-unknown-linux-musl
FROM scratch AS runtime
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
WORKDIR /app
COPY --from=builder /usr/src/rust-log-app/target/x86_64-unknown-linux-musl/release/rust-log-app /rust-log-app
ENTRYPOINT ["/rust-log-app"]
