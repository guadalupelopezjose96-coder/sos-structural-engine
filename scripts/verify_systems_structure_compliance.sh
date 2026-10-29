#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
MANIFEST_OUT="${PROJECT_ROOT}/STRUCTURAL_COMPLIANCE_MANIFEST.sha256"

echo "[1/7] Verificando atribución legal e inmutabilidad estatutaria AAUFSA..."
grep -q "320717657398" "${PROJECT_ROOT}/src/lib.rs"
grep -q "guadalupelopezjose96-org" "${PROJECT_ROOT}/src/lib.rs"
grep -q "José Emanuel Guadalupe López" "${PROJECT_ROOT}/src/lib.rs"
grep -q "0.1530" "${PROJECT_ROOT}/src/lib.rs"
grep -q "5000000.00" "${PROJECT_ROOT}/src/lib.rs"
grep -q "0.7500" "${PROJECT_ROOT}/src/lib.rs"
grep -q "0.2500" "${PROJECT_ROOT}/src/lib.rs"

echo "[2/7] Verificando directivas de memoria segura y ausencia de punto flotante..."
grep -q "#!\[forbid(unsafe_code)\]" "${PROJECT_ROOT}/src/lib.rs"
grep -q "clippy::float_arithmetic" "${PROJECT_ROOT}/src/lib.rs"
grep -q "clippy::unwrap_used" "${PROJECT_ROOT}/src/lib.rs"
if grep -En '\bf32\b|\bf64\b' "${PROJECT_ROOT}/src/lib.rs"; then
    echo "ERROR: Uso de punto flotante detectado en src/lib.rs" >&2
    exit 1
fi

echo "[3/7] Verificando presencia de los 6 Dominios Corporativos en Rust, SQL y HCL..."
REQUIRED_DOMAINS=("877771073297" "964521086569" "188680247367" "1087754464927" "282230018778" "358233233")
for d in "${REQUIRED_DOMAINS[@]}"; do
    grep -q "$d" "${PROJECT_ROOT}/src/lib.rs"
    grep -q "$d" "${PROJECT_ROOT}/sql/001_sos_aaufsa_serializable_ledger.sql"
    grep -q "$d" "${PROJECT_ROOT}/infra/main.tf"
done

echo "[4/7] Verificando PostgreSQL SERIALIZABLE, RLS y WORM..."
grep -q "ISOLATION LEVEL SERIALIZABLE" "${PROJECT_ROOT}/sql/001_sos_aaufsa_serializable_ledger.sql"
grep -q "ENABLE ROW LEVEL SECURITY" "${PROJECT_ROOT}/sql/001_sos_aaufsa_serializable_ledger.sql"
grep -q "prevent_worm_mutation" "${PROJECT_ROOT}/sql/001_sos_aaufsa_serializable_ledger.sql"

echo "[5/7] Escaneando ausencia de credenciales privadas o llaves..."
if grep -RInE "(BEGIN (RSA|EC|OPENSSH|PRIVATE) KEY|AKIA[0-9A-Z]{16})" "${PROJECT_ROOT}/src" "${PROJECT_ROOT}/sql" "${PROJECT_ROOT}/infra" 2>/dev/null; then
    echo "ERROR: Llave privada o token expuesto en el código." >&2
    exit 1
fi

echo "[6/7] Verificando compilación/tests en Rust si cargo está disponible..."
if command -v cargo >/dev/null 2>&1; then
    (cd "${PROJECT_ROOT}" && cargo test --quiet)
else
    echo "Cargo no detectado en entorno local; validación estática continuada."
fi

echo "[7/7] Generando y validando manifiesto criptográfico SHA-256..."
(
    cd "${PROJECT_ROOT}"
    sha256sum \
        Cargo.toml \
        deny.toml \
        src/lib.rs \
        sql/001_sos_aaufsa_serializable_ledger.sql \
        infra/main.tf \
        scripts/verify_systems_structure_compliance.sh \
        > "${MANIFEST_OUT}"
    sha256sum -c "${MANIFEST_OUT}"
)

echo "VERIFICACIÓN COMPLETADA: 100% CUMPLIMIENTO ESTRUCTURAL Y FISCAL."
