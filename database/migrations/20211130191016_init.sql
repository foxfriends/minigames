CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE guilds (id BIGINT PRIMARY KEY);

CREATE TABLE users (id BIGINT PRIMARY KEY);

CREATE TABLE games (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    guild_id BIGINT NOT NULL REFERENCES guilds (id),
    game VARCHAR(32) NOT NULL,
    state JSONB NOT NULL DEFAULT '{}'
);

CREATE TYPE enum_player_status AS ENUM(
    'Pending',
    'Declined',
    'Accepted'
);

CREATE TABLE game_participants (
    game_id UUID NOT NULL REFERENCES games (id),
    user_id BIGINT NOT NULL REFERENCES users (id),
    is_challenger BOOLEAN NOT NULL DEFAULT false,
    status enum_player_status NOT NULL DEFAULT 'Pending',
    score INT NOT NULL DEFAULT 0,
    PRIMARY KEY (game_id, user_id)
);
