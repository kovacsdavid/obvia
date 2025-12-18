FROM --platform=$BUILDPLATFORM rust:1.92.0 AS builder
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
    cp target/aarch64-unknown-linux-gnu/release/obvia_backend /opt; \
    else \
    cargo build --release; \
    cp target/release/obvia_backend /opt; \
    fi

FROM ubuntu:24.04
WORKDIR /opt
EXPOSE 3000
COPY --from=builder /opt/obvia_backend /opt/obvia_backend
RUN groupadd -r obvia && useradd -r -g obvia obvia
USER obvia
ENTRYPOINT ["/opt/obvia_backend"]
