ALTER TABLE api_keys
    ALTER COLUMN public_key SET DATA TYPE CHAR(64) USING LEFT(public_key, 64),
    ALTER COLUMN secret_key SET DATA TYPE CHAR(64) USING LEFT(secret_key, 64);
