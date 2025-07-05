FROM rust:1.88.0 as builder
WORKDIR /opt
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /opt
EXPOSE 3000
COPY --from=builder /opt/target/release/backend_bin /opt/backend_bin
ENTRYPOINT ["/opt/backend_bin"]
