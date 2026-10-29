-- =============================================================================
-- ORGANIZACIÓN: guadalupelopezjose96-org (320717657398 — Tech Arquitecture Pro-Elite)
-- MOTOR CONTABLE Y ESTRUCTURAL: PostgreSQL 16+ ACID SERIALIZABLE + RLS + WORM
-- MARCO LEGAL: AAUFSA (IRC Chapter 56: Secs. 5000F, 5000G, 5000H & Sec. 4)
-- =============================================================================

BEGIN TRANSACTION ISOLATION LEVEL SERIALIZABLE;

CREATE SCHEMA IF NOT EXISTS aaufsa_ledger;

-- 1. Registro Maestro de los 6 Dominios Corporativos Autorizados
CREATE TABLE IF NOT EXISTS aaufsa_ledger.corporate_domains (
    domain_id VARCHAR(32) PRIMARY KEY,
    domain_fqdn VARCHAR(128) NOT NULL UNIQUE,
    org_root_id VARCHAR(32) NOT NULL DEFAULT '320717657398',
    created_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
    CONSTRAINT chk_org_root_id CHECK (org_root_id = '320717657398'),
    CONSTRAINT chk_valid_six_domains CHECK (
        domain_id IN (
            '877771073297',  -- AuraTetrax-com
            '964521086569',  -- Fourphonix-com
            '188680247367',  -- AuraIntelCore-com
            '1087754464927', -- AuraIntel-Global-com
            '282230018778',  -- Standard1979-com
            '358233233'      -- Prophonix-com
        )
    )
);

INSERT INTO aaufsa_ledger.corporate_domains (domain_id, domain_fqdn) VALUES
    ('877771073297',  'AuraTetrax-com'),
    ('964521086569',  'Fourphonix-com'),
    ('188680247367',  'AuraIntelCore-com'),
    ('1087754464927', 'AuraIntel-Global-com'),
    ('282230018778',  'Standard1979-com'),
    ('358233233',     'Prophonix-com')
ON CONFLICT (domain_id) DO NOTHING;

-- 2. Tabla de Agregación Societaria IRC Sec. 414 y 1563 (AAUFSA Sec. 5000G(b))
CREATE TABLE IF NOT EXISTS aaufsa_ledger.controlled_group_assessments (
    assessment_id UUID PRIMARY KEY,
    domain_id VARCHAR(32) NOT NULL REFERENCES aaufsa_ledger.corporate_domains(domain_id),
    tax_year INTEGER NOT NULL CHECK (tax_year >= 2026),
    consolidated_gross_receipts_usd NUMERIC(20, 2) NOT NULL CHECK (consolidated_gross_receipts_usd >= 0.00),
    imputed_displacement_cost_usd NUMERIC(20, 2) NOT NULL CHECK (imputed_displacement_cost_usd >= 0.00),
    is_saas_vendor_exempt_sec_5000h_d BOOLEAN NOT NULL DEFAULT FALSE,
    safe_harbor_36m_applied_sec_5000g_a BOOLEAN NOT NULL DEFAULT FALSE,
    clawback_triggered_sec_5000g_c BOOLEAN NOT NULL DEFAULT FALSE,
    excise_tax_15_3_pct_usd NUMERIC(20, 2) NOT NULL CHECK (excise_tax_15_3_pct_usd >= 0.00),
    clawback_interest_usd NUMERIC(20, 2) NOT NULL DEFAULT 0.00 CHECK (clawback_interest_usd >= 0.00),
    total_chapter_56_liability_usd NUMERIC(20, 2) NOT NULL CHECK (total_chapter_56_liability_usd >= 0.00),
    oasdi_trust_75_pct_usd NUMERIC(20, 2) NOT NULL CHECK (oasdi_trust_75_pct_usd >= 0.00),
    reskilling_trust_25_pct_usd NUMERIC(20, 2) NOT NULL CHECK (reskilling_trust_25_pct_usd >= 0.00),
    recorded_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
    CONSTRAINT chk_exact_trust_fund_split CHECK (
        oasdi_trust_75_pct_usd + reskilling_trust_25_pct_usd = total_chapter_56_liability_usd
    ),
    CONSTRAINT chk_exact_liability_sum CHECK (
        excise_tax_15_3_pct_usd + clawback_interest_usd = total_chapter_56_liability_usd
    )
);

-- 3. Libro Mayor Transaccional de Partida Doble con Encadenamiento Merkle WORM
CREATE TABLE IF NOT EXISTS aaufsa_ledger.journal_headers (
    journal_id UUID PRIMARY KEY,
    idempotency_key VARCHAR(128) NOT NULL UNIQUE,
    domain_id VARCHAR(32) NOT NULL REFERENCES aaufsa_ledger.corporate_domains(domain_id),
    source_plane VARCHAR(32) NOT NULL CHECK (source_plane = 'FISCAL_PLANE_AAUFSA'),
    previous_merkle_hash CHAR(64) NOT NULL,
    current_merkle_hash CHAR(64) NOT NULL UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp()
);

CREATE TABLE IF NOT EXISTS aaufsa_ledger.journal_lines (
    line_id BIGSERIAL PRIMARY KEY,
    journal_id UUID NOT NULL REFERENCES aaufsa_ledger.journal_headers(journal_id) ON DELETE RESTRICT,
    domain_id VARCHAR(32) NOT NULL REFERENCES aaufsa_ledger.corporate_domains(domain_id),
    account_code VARCHAR(32) NOT NULL,
    entry_side VARCHAR(6) NOT NULL CHECK (entry_side IN ('DEBIT', 'CREDIT')),
    amount_usd NUMERIC(20, 2) NOT NULL CHECK (amount_usd >= 0.00)
);

-- 4. Verificador Diferido de Partida Doble Estricta al COMMIT (ACID Guarantee)
CREATE OR REPLACE FUNCTION aaufsa_ledger.enforce_double_entry_balance()
RETURNS TRIGGER
LANGUAGE plpgsql
SECURITY DEFINER
AS $$
DECLARE
    v_debits  NUMERIC(20, 2);
    v_credits NUMERIC(20, 2);
BEGIN
    SELECT
        COALESCE(SUM(CASE WHEN entry_side = 'DEBIT'  THEN amount_usd ELSE 0.00 END), 0.00),
        COALESCE(SUM(CASE WHEN entry_side = 'CREDIT' THEN amount_usd ELSE 0.00 END), 0.00)
    INTO v_debits, v_credits
    FROM aaufsa_ledger.journal_lines
    WHERE journal_id = NEW.journal_id;

    IF v_debits <> v_credits THEN
        RAISE EXCEPTION 'Violación ACID Partida Doble en journal_id %: Débitos (%) != Créditos (%)',
            NEW.journal_id, v_debits, v_credits;
    END IF;

    RETURN NEW;
END;
$$;

DROP TRIGGER IF EXISTS trg_verify_double_entry_balance ON aaufsa_ledger.journal_lines;
CREATE CONSTRAINT TRIGGER trg_verify_double_entry_balance
AFTER INSERT OR UPDATE ON aaufsa_ledger.journal_lines
DEFERRABLE INITIALLY DEFERRED
FOR EACH ROW
EXECUTE FUNCTION aaufsa_ledger.enforce_double_entry_balance();

-- 5. Protección WORM (Write-Once-Read-Many): Prohibición de UPDATE/DELETE en el Libro Mayor
CREATE OR REPLACE FUNCTION aaufsa_ledger.prevent_worm_mutation()
RETURNS TRIGGER
LANGUAGE plpgsql
AS $$
BEGIN
    RAISE EXCEPTION 'Violación Forense WORM: Los registros del Plano Fiscal AAUFSA son inmutables (Operación % rechazada)', TG_OP;
END;
$$;

DROP TRIGGER IF EXISTS trg_worm_headers ON aaufsa_ledger.journal_headers;
CREATE TRIGGER trg_worm_headers
BEFORE UPDATE OR DELETE ON aaufsa_ledger.journal_headers
FOR EACH ROW EXECUTE FUNCTION aaufsa_ledger.prevent_worm_mutation();

DROP TRIGGER IF EXISTS trg_worm_lines ON aaufsa_ledger.journal_lines;
CREATE TRIGGER trg_worm_lines
BEFORE UPDATE OR DELETE ON aaufsa_ledger.journal_lines
FOR EACH ROW EXECUTE FUNCTION aaufsa_ledger.prevent_worm_mutation();

-- 6. Aislamiento Row-Level Security (RLS) para los 6 Dominios Corporativos
ALTER TABLE aaufsa_ledger.controlled_group_assessments ENABLE ROW LEVEL SECURITY;
ALTER TABLE aaufsa_ledger.controlled_group_assessments FORCE ROW LEVEL SECURITY;

ALTER TABLE aaufsa_ledger.journal_headers ENABLE ROW LEVEL SECURITY;
ALTER TABLE aaufsa_ledger.journal_headers FORCE ROW LEVEL SECURITY;

ALTER TABLE aaufsa_ledger.journal_lines ENABLE ROW LEVEL SECURITY;
ALTER TABLE aaufsa_ledger.journal_lines FORCE ROW LEVEL SECURITY;

CREATE POLICY rls_domain_isolation_assessments ON aaufsa_ledger.controlled_group_assessments
    USING (domain_id = current_setting('app.current_domain_id', true))
    WITH CHECK (domain_id = current_setting('app.current_domain_id', true));

CREATE POLICY rls_domain_isolation_headers ON aaufsa_ledger.journal_headers
    USING (domain_id = current_setting('app.current_domain_id', true))
    WITH CHECK (domain_id = current_setting('app.current_domain_id', true));

CREATE POLICY rls_domain_isolation_lines ON aaufsa_ledger.journal_lines
    USING (domain_id = current_setting('app.current_domain_id', true))
    WITH CHECK (domain_id = current_setting('app.current_domain_id', true));

COMMIT;
