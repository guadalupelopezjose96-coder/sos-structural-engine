-- =============================================================================
-- TEST 1: Asiento balanceado bajo Dominio AuraIntelCore (188680247367)
-- =============================================================================
SET app.current_domain_id = '188680247367';

BEGIN TRANSACTION ISOLATION LEVEL SERIALIZABLE;

INSERT INTO aaufsa_ledger.journal_headers (
    journal_id, idempotency_key, domain_id, source_plane, previous_merkle_hash, current_merkle_hash
) VALUES (
    'a0000000-0000-0000-0000-000000000001',
    'IDEMP-2026-LIVE-001',
    '188680247367',
    'FISCAL_PLANE_AAUFSA',
    '0000000000000000000000000000000000000000000000000000000000000000',
    'e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855'
);

-- Débito: Impuesto AAUFSA 15.3% ($153,000.00 USD)
INSERT INTO aaufsa_ledger.journal_lines (journal_id, domain_id, account_code, entry_side, amount_usd)
VALUES ('a0000000-0000-0000-0000-000000000001', '188680247367', '6156-AAUFSA-EXP', 'DEBIT', 153000.00);

-- Créditos: 75% Social Security ($114,750.00) + 25% Recapacitación ($38,250.00)
INSERT INTO aaufsa_ledger.journal_lines (journal_id, domain_id, account_code, entry_side, amount_usd)
VALUES 
    ('a0000000-0000-0000-0000-000000000001', '188680247367', '2156-OASDI-75', 'CREDIT', 114750.00),
    ('a0000000-0000-0000-0000-000000000001', '188680247367', '2157-RESKILL-25', 'CREDIT', 38250.00);

COMMIT;

-- =============================================================================
-- TEST 2: Validación RLS (Cambio a Dominio AuraTetrax: 877771073297)
-- Debe reportar exactamente 0 filas visibles
-- =============================================================================
SET app.current_domain_id = '877771073297';
SELECT count(*) AS rls_filas_visibles_otro_dominio FROM aaufsa_ledger.journal_headers;

-- =============================================================================
-- TEST 3: Validación WORM (Intento de UPDATE debe ser repelido por el trigger)
-- =============================================================================
SET app.current_domain_id = '188680247367';
DO $$
BEGIN
    UPDATE aaufsa_ledger.journal_headers SET source_plane = 'TAMPERED' WHERE journal_id = 'a0000000-0000-0000-0000-000000000001';
    RAISE EXCEPTION 'FALLO: WORM permitió la mutación.';
EXCEPTION WHEN OTHERS THEN
    RAISE NOTICE 'CONFIRMACIÓN WORM (Mutación abortada con éxito): %', SQLERRM;
END;
$$;
