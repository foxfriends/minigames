CREATE TABLE api_keys (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id BIGINT NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    public_key CHAR(128) NOT NULL,
    secret_key CHAR(128) NOT NULL
);
