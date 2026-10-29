-- 1. Crear usuario aplicativo sin permisos de superusuario
DO $$
BEGIN
    IF NOT EXISTS (SELECT FROM pg_roles WHERE rolname = 'aaufsa_app_user') THEN
        CREATE ROLE aaufsa_app_user WITH LOGIN;
    END IF;
END
$$;

-- 2. Conceder permisos DML sobre el esquema y las tablas
GRANT USAGE ON SCHEMA aaufsa_ledger TO aaufsa_app_user;
GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA aaufsa_ledger TO aaufsa_app_user;

-- 3. Probar como usuario aplicativo bajo Dominio AuraTetrax (877771073297)
SET ROLE aaufsa_app_user;
SET app.current_domain_id = '877771073297';

SELECT count(*) AS rls_filas_visibles_otro_dominio FROM aaufsa_ledger.journal_headers;

-- 4. Probar como usuario aplicativo bajo el dominio propietario AuraIntelCore (188680247367)
SET app.current_domain_id = '188680247367';

SELECT count(*) AS rls_filas_visibles_propio_dominio FROM aaufsa_ledger.journal_headers;

RESET ROLE;
