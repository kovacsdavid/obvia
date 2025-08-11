FROM --platform=$BUILDPLATFORM rust:1.89.0 AS builder
ARG TARGETPLATFORM
ARG BUILDPLATFORM
RUN if [ "$TARGETPLATFORM" = "linux/arm64" ] && [ "$BUILDPLATFORM" != "$TARGETPLATFORM" ]; then \
    apt update && \
    apt install -y gcc-aarch64-linux-gnu && \
    rustup target add aarch64-unknown-linux-gnu; \
    fi
WORKDIR /opt
COPY . .
RUN if [ "$TARGETPLATFORM" = "linux/arm64" ] && [ "$BUILDPLATFORM" != "$TARGETPLATFORM" ]; then \
    cargo build --release --target aarch64-unknown-linux-gnu --config target.aarch64-unknown-linux-gnu.linker=\"aarch64-linux-gnu-gcc\"; \
    cp target/aarch64-unknown-linux-gnu/release/backend_bin /opt; \
    else \
    cargo build --release; \
    cp target/release/backend_bin /opt; \
    fi

FROM debian:bookworm-slim
WORKDIR /opt
EXPOSE 3000
COPY --from=builder /opt/migrations /opt/migrations
COPY --from=builder /opt/backend_bin /opt/backend_bin
ENTRYPOINT ["/opt/backend_bin"]
