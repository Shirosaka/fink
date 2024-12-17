-- Your SQL goes here
CREATE TABLE starboard_entries (
    id SERIAL PRIMARY KEY,
    guild_id BIGINT NOT NULL,
    channel_id BIGINT NOT NULL,
    message_id BIGINT NOT NULL,
    reactions INT NOT NULL DEFAULT(0)
);