CREATE TABLE IF NOT EXISTS mappers (
    osu_user_id BIGINT PRIMARY KEY,
    username TEXT NOT NULL,
    country_code CHAR(2) NOT NULL,
    count_graveyard INT NOT NULL DEFAULT 0,
    count_pending INT NOT NULL DEFAULT 0,
    count_wip INT NOT NULL DEFAULT 0,
    count_loved INT NOT NULL DEFAULT 0,
    count_ranked INT NOT NULL DEFAULT 0,
    count_approved INT NOT NULL DEFAULT 0,
    count_total INT NOT NULL DEFAULT 0,
    is_bn BOOLEAN NOT NULL DEFAULT FALSE,
    nominated_count INT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_mappers_country_code ON mappers (country_code);
CREATE INDEX IF NOT EXISTS idx_mappers_updated_at ON mappers (updated_at);

CREATE TABLE IF NOT EXISTS scan_state (
    name TEXT PRIMARY KEY,
    cursor TEXT NULL,
    last_success_at TIMESTAMPTZ NULL,
    last_error_at TIMESTAMPTZ NULL,
    retry_count INT NOT NULL DEFAULT 0,
    next_retry_at TIMESTAMPTZ NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
