# uamappers

`uamappers` builds and maintains an up-to-date index of the Ukrainian osu! community, starting with Ukrainian mappers.

This repository currently contains the backend API and a crawler worker. A frontend and bot may be added later as separate components.

## Backend

- **API**: Rust HTTP API with OpenAPI/Swagger, backed by PostgreSQL.
- **Worker**: Rust crawler that discovers Ukrainian mappers and refreshes cached data.

### Local dev (docker)

1. Copy `.env.example` to `.env` and fill in the values (at minimum: `OSU_CLIENT_ID`, `OSU_CLIENT_SECRET`, `POSTGRES_PASSWORD`).
2. Start services:
   - `just up`
3. Apply DB schema:
   - `just migrate`
4. Run the worker manually (does not auto-start in docker):
   - `just worker`
   - `just worker-logs`
   - `just worker-stop`

### Database tables (high level)

- `scan_state`: resumable checkpoints for crawler jobs (cursor, retry/backoff, timestamps).
- `ua_mappers`: discovered Ukrainian mappers (osu user id, username, country code, first/last seen).
- `osu_users`: cached raw osu! user payloads keyed by `osu_user_id`.
- `beatmapsets`: cached raw osu! beatmapset payloads keyed by `osu_beatmapset_id`.
- `osu_user_beatmapsets`: join table mapping `(osu_user_id, kind) -> beatmapset ids` for fast listing.

## License

Apache License 2.0. See `LICENSE` and `NOTICE`.
