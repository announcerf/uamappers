ALTER TABLE mappers
    ADD COLUMN IF NOT EXISTS kudosu_available INT NULL,
    ADD COLUMN IF NOT EXISTS kudosu_total INT NULL;

CREATE TABLE IF NOT EXISTS beatmapsets (
    osu_beatmapset_id BIGINT PRIMARY KEY,
    creator_osu_user_id BIGINT NOT NULL,
    creator_username TEXT NOT NULL,
    status TEXT NOT NULL,
    artist TEXT NOT NULL,
    title TEXT NOT NULL,
    artist_unicode TEXT NULL,
    title_unicode TEXT NULL,
    submitted_date TIMESTAMPTZ NULL,
    ranked_date TIMESTAMPTZ NULL,
    last_updated TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_beatmapsets_creator_osu_user_id
    ON beatmapsets (creator_osu_user_id);
CREATE INDEX IF NOT EXISTS idx_beatmapsets_status
    ON beatmapsets (status);
CREATE INDEX IF NOT EXISTS idx_beatmapsets_last_updated
    ON beatmapsets (last_updated);

