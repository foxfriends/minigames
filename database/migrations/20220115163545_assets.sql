CREATE TABLE assets (
    id  UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    ext VARCHAR(4) NOT NULL
);

ALTER TABLE game_servers ADD COLUMN asset_id UUID REFERENCES assets (id) ON DELETE SET NULL;
