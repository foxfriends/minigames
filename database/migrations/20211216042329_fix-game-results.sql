CREATE OR REPLACE FUNCTION clear_votes_for_game() RETURNS TRIGGER AS
    $$
    BEGIN
        DELETE FROM game_complete_votes WHERE game_id = NEW.id;
        RETURN NEW;
    END;
    $$
    LANGUAGE plpgsql;
