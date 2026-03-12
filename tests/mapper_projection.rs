#![recursion_limit = "256"]

use chrono::Utc;
use rosu_v2::model::beatmap::BeatmapsetExtended;
use rosu_v2::model::user::UserExtended;
use serde_json::json;
use uamappers_worker::features::ingest::worker::jobs::mapper_enrich::projection::beatmapsets::{
    maps_to_profile_rows, mapset_to_profile_row,
};
use uamappers_worker::features::ingest::worker::jobs::mapper_enrich::projection::user_to_mapper_fingerprint;

#[test]
fn user_projection_keeps_only_required_fields() {
    let cached_at = Utc::now();
    let user: UserExtended = serde_json::from_value(json!({
        "id": 42,
        "username": "Mapper",
        "avatar_url": "https://a",
        "comments_count": 1,
        "country": "Ukraine",
        "country_code": "UA",
        "cover": { "url": "https://cover" },
        "default_group": "default",
        "has_supported": false,
        "is_active": true,
        "is_bot": false,
        "is_deleted": false,
        "is_online": false,
        "is_supporter": false,
        "join_date": "2024-01-01T00:00:00Z",
        "kudosu": { "available": 3, "total": 11 },
        "max_blocks": 100,
        "max_friends": 200,
        "playmode": 0,
        "pm_friends_only": false,
        "post_count": 0,
        "profile_order": [],
        "daily_challenge_user_stats": {
            "daily_streak_best": 0,
            "daily_streak_current": 0,
            "last_update": null,
            "last_weekly_streak": null,
            "playcount": 0,
            "top_10p_placements": 0,
            "top_50p_placements": 0,
            "user_id": 42,
            "weekly_streak_best": 0,
            "weekly_streak_current": 0
        },
        "mapping_follower_count": 99,
        "is_bng": true,
        "is_nat": false,
        "is_gmt": true,
        "is_limited_bn": false,
        "is_full_bn": true,
        "badges": [
            {
                "awarded_at": "2024-01-02T00:00:00Z",
                "description": "Badge",
                "image_url": "https://badge",
                "url": "https://badge-url"
            }
        ],
        "groups": [
            {
                "id": 1,
                "identifier": "nat",
                "name": "NAT",
                "short_name": "NAT",
                "has_playmodes": false,
                "is_probationary": false,
                "description": "desc",
                "colour": "#fff"
            }
        ]
    }))
    .expect("user json should deserialize");

    let row = user_to_mapper_fingerprint(&user, cached_at);

    assert_eq!(row.username, "Mapper");
    assert_eq!(row.country_code, "UA");
    assert_eq!(row.primary_mode, "osu");
    assert_eq!(row.mapping_followers, 99);
    assert_eq!(row.kudosu.total, 11);
    assert!(row.is_bng);
    assert!(row.is_gmt);
    assert!(row.is_full_bn);
    assert!(!row.is_probationary_bn);
}

#[test]
fn beatmapset_projection_flattens_mapset_and_nested_maps() {
    let cached_at = Utc::now();
    let mapset: BeatmapsetExtended = serde_json::from_value(json!({
        "id": 1000,
        "artist": "Artist",
        "artist_unicode": "Артист",
        "availability": {
            "download_disabled": false,
            "more_information": null
        },
        "bpm": 180.0,
        "can_be_hyped": true,
        "covers": {
            "cover": "https://cover",
            "cover@2x": "https://cover2x",
            "card": "https://card",
            "card@2x": "https://card2x",
            "list": "https://list",
            "list@2x": "https://list2x",
            "slimcover": "https://slim",
            "slimcover@2x": "https://slim2x"
        },
        "creator": "Mapper",
        "user_id": 42,
        "discussion_enabled": true,
        "discussion_locked": false,
        "favourite_count": 300,
        "genre": 3,
        "hype": { "current": 2, "required": 5 },
        "is_scoreable": true,
        "language": 3,
        "last_updated": "2025-01-01T00:00:00Z",
        "beatmaps": [
            {
                "ar": 9.0,
                "bpm": 180.0,
                "convert": false,
                "count_circles": 100,
                "count_sliders": 50,
                "count_spinners": 2,
                "user_id": 42,
                "cs": 4.0,
                "drain": 6.5,
                "is_scoreable": true,
                "last_updated": "2025-01-01T00:00:00Z",
                "id": 2001,
                "beatmapset_id": 1000,
                "max_combo": 1234,
                "mode": 0,
                "accuracy": 8.5,
                "passcount": 1000,
                "playcount": 5000,
                "hit_length": 90,
                "total_length": 120,
                "difficulty_rating": 5.2,
                "status": 1,
                "url": "https://osu.ppy.sh/beatmaps/2001",
                "version": "Insane",
                "owners": [{ "id": 42, "username": "Mapper" }]
            }
        ],
        "nominations_summary": {
            "current": 1,
            "eligible_main_rulesets": [0],
            "required_meta": {
                "main_ruleset": 0,
                "non_main_ruleset": 1
            }
        },
        "nsfw": false,
        "offset": 0,
        "play_count": 5000,
        "preview_url": "https://preview.mp3",
        "rating": 8.7,
        "ranked_date": "2025-01-15T00:00:00Z",
        "source": "Game",
        "spotlight": false,
        "status": 1,
        "storyboard": true,
        "submitted_date": "2024-12-01T00:00:00Z",
        "tags": "tag1 tag2",
        "title": "Title",
        "title_unicode": "Назва",
        "video": true
    }))
    .expect("mapset json should deserialize");

    let mapset_row = mapset_to_profile_row(&mapset, cached_at);
    let map_rows = maps_to_profile_rows(&mapset, cached_at);

    assert_eq!(mapset_row.osu_beatmapset_id, 1000);
    assert_eq!(mapset_row.creator_name, "Mapper");
    assert_eq!(mapset_row.genre.as_deref(), Some("anime"));
    assert_eq!(mapset_row.language.as_deref(), Some("japanese"));
    assert_eq!(mapset_row.status, "ranked");
    assert_eq!(mapset_row.difficulty_count, 1);
    assert_eq!(mapset_row.nominations_current, 1);

    assert_eq!(map_rows.len(), 1);
    assert_eq!(map_rows[0].osu_beatmap_id, 2001);
    assert_eq!(map_rows[0].mode, "osu");
    assert_eq!(map_rows[0].status, "ranked");
    assert_eq!(map_rows[0].count_circles, 100);
}
