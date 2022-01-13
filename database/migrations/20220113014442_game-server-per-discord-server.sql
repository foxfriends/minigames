CREATE TABLE game_server_guilds (
    game_server_name VARCHAR(32) NOT NULL REFERENCES game_servers (name) ON DELETE CASCADE ON UPDATE CASCADE,
    guild_id         BIGINT      NOT NULL REFERENCES guilds (id) ON DELETE CASCADE,
    PRIMARY KEY (game_server_name, guild_id)
);
