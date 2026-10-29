# =============================================================================
# ORGANIZACIÓN: guadalupelopezjose96-org (320717657398)
# ARTEFACTO: Imagen OCI de Producción - Compilación Determinista Estática (musl)
# =============================================================================

# Etapa 1: Compilación estática
FROM rust:1.75-alpine AS builder

RUN apk add --no-cache musl-dev

WORKDIR /build

COPY Cargo.toml Cargo.lock ./
COPY src ./src

ENV RUSTFLAGS="-C target-feature=+crt-static -C panic=abort -C link-arg=-s"

RUN cargo build --release --target x86_64-unknown-linux-musl

# Etapa 2: Imagen de ejecución minimalista de superficie cero
FROM scratch

LABEL org.opencontainers.image.title="sos-structural-engine" \
      org.opencontainers.image.description="System-of-Systems Topology & AAUFSA Ledger Engine" \
      org.opencontainers.image.vendor="guadalupelopezjose96-org" \
      org.opencontainers.image.authors="José Emanuel Guadalupe López" \
      org.opencontainers.image.version="1.0.0"

COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/sos-structural-engine /sos-structural-engine

USER 65534:65534

ENTRYPOINT ["/sos-structural-engine"]
