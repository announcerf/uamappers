ALTER TABLE beatmapsets
    ADD COLUMN IF NOT EXISTS raw JSONB NOT NULL DEFAULT '{}'::jsonb,
    ADD COLUMN IF NOT EXISTS play_count INT NOT NULL DEFAULT 0,
    ADD COLUMN IF NOT EXISTS favourite_count INT NOT NULL DEFAULT 0;

CREATE INDEX IF NOT EXISTS idx_beatmapsets_play_count
    ON beatmapsets (play_count);
CREATE INDEX IF NOT EXISTS idx_beatmapsets_favourite_count
    ON beatmapsets (favourite_count);

