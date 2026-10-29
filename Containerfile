# =============================================================================
# ARTEFACTO: sos-structural-engine (OCI Container Image)
# VERSIÓN: 1.2.0
# ORGANIZACIÓN: guadalupelopezjose96-org (ID: 320717657398)
# BASELINE: Scratch minimalista inmutable (Zero-Trust Attack Surface)
# =============================================================================

# --- Stage 1: Build & Verify ---
FROM rust:1.80-alpine AS builder

RUN apk add --no-cache musl-dev perl make

WORKDIR /usr/src/sos-structural-engine
COPY . .

# Compilación estática pura sin glibc
RUN cargo build --release --locked

# --- Stage 2: Final Immutable Runtime ---
FROM scratch

LABEL org.opencontainers.image.title="sos-structural-engine" \
      org.opencontainers.image.version="1.2.0" \
      org.opencontainers.image.vendor="guadalupelopezjose96-org" \
      org.opencontainers.image.licenses="Apache-2.0" \
      org.opencontainers.image.description="Deterministic VPP dispatch and fiscal ledger engine"

# Copiar únicamente el binario compilado de forma estática
COPY --from=builder /usr/src/sos-structural-engine/target/release/sos-structural-engine /sos-structural-engine
COPY --from=builder /usr/src/sos-structural-engine/STRUCTURAL_COMPLIANCE_MANIFEST.sha256 /STRUCTURAL_COMPLIANCE_MANIFEST.sha256

USER 65534:65534
ENTRYPOINT ["/sos-structural-engine"]
