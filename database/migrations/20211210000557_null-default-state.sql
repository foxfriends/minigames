ALTER TABLE games
    ALTER COLUMN state
    SET DEFAULT 'null'::jsonb;
