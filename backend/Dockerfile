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
    ca-certificates curl tar coreutils xz-utils \
  && rm -rf /var/lib/apt/lists/*

ARG TYPST_VERSION=0.15.1

RUN case "${TARGETPLATFORM}" in \
      "linux/amd64") \
        TYPST_ARCH="x86_64-unknown-linux-musl"; \
        TYPST_SHA256="a6d077d0a95eed5a2eba715b2dae06be954f624ccbf85758a03f389ded33118c"; \
        ;; \
      "linux/arm64") \
        TYPST_ARCH="aarch64-unknown-linux-musl"; \
        TYPST_SHA256="5aa8d74a3d906e60ea12a66ac2f37f8eef1b14cbad7182a745e393a10c23dcee"; \
        ;; \
      *) echo "Unsupported TARGETPLATFORM: ${TARGETPLATFORM}" && exit 1 ;; \
    esac \
 && TYPST_URL="https://github.com/typst/typst/releases/download/v${TYPST_VERSION}/typst-${TYPST_ARCH}.tar.xz" \
 && curl -fsSL -o /tmp/typst.tar.xz "${TYPST_URL}" \
 && echo "${TYPST_SHA256} /tmp/typst.tar.xz" | sha256sum -c - \
 && mkdir -p /opt/typst \
 && tar -xJf /tmp/typst.tar.xz -C /opt/typst --strip-components=1 \
 && install -m 0755 /opt/typst/typst /usr/local/bin/typst \
 && typst --version

FROM --platform=$BUILDPLATFORM rust:1.97.1 AS builder
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
    cp target/aarch64-unknown-linux-gnu/release/obvia_server /opt; \
    else \
    cargo build --release; \
    cp target/release/obvia_server /opt; \
    fi

FROM ubuntu:24.04
WORKDIR /opt
EXPOSE 3000
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    fontconfig \
    fonts-dejavu-core \
  && rm -rf /var/lib/apt/lists/*
COPY --from=builder /opt/obvia_server /opt/obvia_server
COPY --from=builder /opt/templates /opt/templates
COPY --from=typst /usr/local/bin/typst /usr/local/bin/typst
RUN groupadd -r obvia && useradd -r -g obvia obvia
RUN typst --version
USER obvia
ENTRYPOINT ["/opt/obvia_server"]
