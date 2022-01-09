CREATE TABLE game_servers (
    name VARCHAR(32) PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    public_url TEXT NOT NULL
);

-- There should be no API keys yet, and if someone manually inserted one it is
-- invalid anyway, so let's get rid of it.
TRUNCATE TABLE api_keys;
ALTER TABLE api_keys
    DROP COLUMN user_id,
    ADD COLUMN game_server_name VARCHAR(32) NOT NULL REFERENCES game_servers (name) ON DELETE CASCADE ON UPDATE CASCADE;

-- We have no choice but to delete all existing games... This is all happening 
-- before anyone really uses this app anyway
TRUNCATE TABLE games CASCADE;
ALTER TABLE games ADD CONSTRAINT games_game_fkey FOREIGN KEY (game) REFERENCES game_servers (name) ON DELETE CASCADE ON UPDATE CASCADE;
