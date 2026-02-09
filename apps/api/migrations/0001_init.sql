CREATE TABLE IF NOT EXISTS scan_state (
    name TEXT PRIMARY KEY,
    cursor TEXT NULL,
    last_success_at TIMESTAMPTZ NULL,
    last_error_at TIMESTAMPTZ NULL,
    retry_count INT NOT NULL DEFAULT 0,
    next_retry_at TIMESTAMPTZ NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS ua_mappers (
    osu_user_id BIGINT PRIMARY KEY,
    username TEXT NOT NULL,
    country_code CHAR(2) NOT NULL,
    first_seen_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_seen_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_ua_mappers_country_code ON ua_mappers (country_code);
CREATE INDEX IF NOT EXISTS idx_ua_mappers_updated_at ON ua_mappers (updated_at);
CREATE INDEX IF NOT EXISTS idx_ua_mappers_username_lower ON ua_mappers (LOWER(username));

CREATE TABLE IF NOT EXISTS osu_users (
    osu_user_id BIGINT PRIMARY KEY,
    raw JSONB NOT NULL DEFAULT '{}'::jsonb,
    fetched_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_osu_users_updated_at ON osu_users (updated_at);

CREATE TABLE IF NOT EXISTS beatmapsets (
    osu_beatmapset_id BIGINT PRIMARY KEY,
    last_updated TIMESTAMPTZ NOT NULL,
    raw JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_beatmapsets_last_updated
    ON beatmapsets (last_updated);

CREATE TABLE IF NOT EXISTS osu_user_beatmapsets (
    osu_user_id BIGINT NOT NULL,
    kind TEXT NOT NULL,
    osu_beatmapset_id BIGINT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (osu_user_id, kind, osu_beatmapset_id)
);

CREATE INDEX IF NOT EXISTS idx_osu_user_beatmapsets_user_kind
    ON osu_user_beatmapsets (osu_user_id, kind);
