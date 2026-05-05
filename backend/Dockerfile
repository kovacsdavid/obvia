#
# This file is part of the Obvia ERP.
#
# Copyright (C) 2025 Kovács Dávid <kapcsolat@kovacsdavid.dev>
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU Affero General Public License as published
# by the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU Affero General Public License for more details.
#
# You should have received a copy of the GNU Affero General Public License
# along with this program.  If not, see <https://www.gnu.org/licenses/>.
#

FROM ubuntu:24.04 AS typst
ARG TARGETPLATFORM
SHELL ["/bin/bash", "-euxo", "pipefail", "-c"]

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates curl tar coreutils \
  && rm -rf /var/lib/apt/lists/*

ARG TYPST_VERSION=0.14.2

RUN case "${TARGETPLATFORM}" in \
      "linux/amd64") \
        TYPST_ARCH="x86_64-unknown-linux-musl"; \
        TYPST_SHA256="a6044cbad2a954deb921167e257e120ac0a16b20339ec01121194ff9d394996d"; \
        ;; \
      "linux/arm64") \
        TYPST_ARCH="aarch64-unknown-linux-musl"; \
        TYPST_SHA256="491b101aa40a3a7ea82a3f8a6232cabb4e6a7e233810082e5ac812d43fdcd47a"; \
        ;; \
      *) echo "Unsupported TARGETPLATFORM: ${TARGETPLATFORM}" && exit 1 ;; \
    esac \
 && TYPST_URL="https://github.com/typst/typst/releases/download/v${TYPST_VERSION}/typst-${TYPST_ARCH}.tar.xz" \
 && curl -fsSL -o /tmp/typst.tar.xz "${TYPST_URL}" \
 && echo "${TYPST_SHA256}  /tmp/typst.tar.xz" | sha256sum -c - \
 && mkdir -p /opt/typst \
 && tar -xJf /tmp/typst.tar.xz -C /opt/typst --strip-components=1 \
 && install -m 0755 /opt/typst/typst /usr/local/bin/typst \
 && typst --version

FROM --platform=$BUILDPLATFORM rust:1.95.0 AS builder
ARG TARGETPLATFORM
ARG BUILDPLATFORM
SHELL ["/bin/bash", "-euxo", "pipefail", "-c"]
RUN if [ "$TARGETPLATFORM" = "linux/arm64" ] && [ "$BUILDPLATFORM" != "$TARGETPLATFORM" ]; then \
    apt update && \
    apt install -y gcc-aarch64-linux-gnu && \
    rustup target add aarch64-unknown-linux-gnu && \
    rm -rf /var/lib/apt/lists/*; \
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
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    fontconfig \
    fonts-dejavu-core \
  && rm -rf /var/lib/apt/lists/*
COPY --from=builder /opt/obvia_backend /opt/obvia_backend
COPY --from=builder /opt/templates /opt/templates
COPY --from=typst /usr/local/bin/typst /usr/local/bin/typst
RUN groupadd -r obvia && useradd -r -g obvia obvia
RUN typst --version
USER obvia
ENTRYPOINT ["/opt/obvia_backend"]
