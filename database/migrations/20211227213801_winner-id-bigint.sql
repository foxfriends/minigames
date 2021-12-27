ALTER TABLE game_complete_votes ALTER COLUMN winner_id TYPE BIGINT USING null;
ALTER TABLE game_complete_votes ADD FOREIGN KEY (winner_id) REFERENCES users (id) ON DELETE CASCADE;
