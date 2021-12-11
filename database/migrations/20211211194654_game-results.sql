CREATE TABLE game_complete_votes (
    game_id UUID NOT NULL references games (id),
    user_id BIGINT NOT NULL REFERENCES users (id),
    winner_id UUID,
    PRIMARY KEY (game_id, user_id)
);

CREATE FUNCTION clear_votes_for_game() RETURNS TRIGGER AS
    $$
    BEGIN
        DELETE FROM game_complete_votes WHERE game_id = NEW.game_id;
        RETURN NEW;
    END;
    $$
    LANGUAGE plpgsql;

CREATE TRIGGER game_state_change_clears_votes
    AFTER UPDATE OF state ON games
    FOR EACH ROW
    EXECUTE PROCEDURE clear_votes_for_game();
