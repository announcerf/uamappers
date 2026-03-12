#![recursion_limit = "256"]

use chrono::{TimeZone, Utc};
use rosu_v2::model::beatmap::BeatmapsetExtended;
use serde_json::json;
use uamappers_api::features::mappers::storage::mapper_stats_current_repo::NewMapperStatsCurrentRow;
use uamappers_worker::features::ingest::worker::jobs::mapper_enrich::snapshot::{
    mapper_stats_row_to_snapshot_row, mapset_to_snapshot_row, snapshot_week,
};

#[test]
fn snapshot_week_rounds_to_monday_midnight_utc() {
    let dt = Utc.with_ymd_and_hms(2026, 3, 11, 15, 45, 0).unwrap();

    let week = snapshot_week(dt);

    assert_eq!(week, Utc.with_ymd_and_hms(2026, 3, 9, 0, 0, 0).unwrap());
}

#[test]
fn beatmapset_snapshot_uses_current_mapset_counters() {
    let mapset: BeatmapsetExtended = serde_json::from_value(json!({
        "id": 1000,
        "artist": "Artist",
        "availability": { "download_disabled": false, "more_information": null },
        "bpm": 180.0,
        "can_be_hyped": false,
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
        "is_scoreable": true,
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
                "version": "Insane"
            }
        ],
        "nominations_summary": {
            "current": 1,
            "eligible_main_rulesets": [0],
            "required_meta": { "main_ruleset": 0, "non_main_ruleset": 1 }
        },
        "nsfw": false,
        "offset": 0,
        "play_count": 5000,
        "preview_url": "https://preview.mp3",
        "rating": 8.7,
        "source": "Game",
        "spotlight": false,
        "status": 1,
        "storyboard": true,
        "tags": "tag1 tag2",
        "title": "Title",
        "video": true
    }))
    .expect("mapset json should deserialize");

    let row = mapset_to_snapshot_row(&mapset, Utc.with_ymd_and_hms(2026, 3, 9, 0, 0, 0).unwrap());

    assert_eq!(row.osu_beatmapset_id, 1000);
    assert_eq!(row.playcount, 5000);
    assert_eq!(row.favourite_count, 300);
    assert_eq!(row.status, "ranked");
    assert!(row.avg_pass_rate > 0.0);
}

#[test]
fn mapper_stats_snapshot_keeps_weekly_rollup_values() {
    let stats = NewMapperStatsCurrentRow {
        osu_user_id: 42,
        total_mapsets: 10,
        ranked_mapsets: 4,
        loved_mapsets: 2,
        guest_mapsets: 3,
        nominated_mapsets: 1,
        graveyard_mapsets: 1,
        pending_mapsets: 0,
        total_playcount: 10000,
        avg_rating: 8.5,
        weighted_rating: 8.6,
        avg_stars: 5.0,
        min_stars: 2.0,
        max_stars: 7.0,
        avg_bpm: 180.0,
        avg_length_seconds: 120.0,
        avg_ar: 9.0,
        avg_cs: 4.0,
        avg_od: 8.0,
        avg_hp: 6.0,
        first_submitted_date: None,
        first_ranked_date: None,
        last_mapset_updated_at: None,
        main_mode: "osu".to_string(),
        mapping_followers: 10,
        kudosu_available: 12,
        kudosu_total: 20,
        has_ranked: true,
        has_loved: true,
        has_guest: true,
        has_nominated: true,
    };

    let row = mapper_stats_row_to_snapshot_row(
        &stats,
        Utc.with_ymd_and_hms(2026, 3, 9, 0, 0, 0).unwrap(),
    );

    assert_eq!(row.osu_user_id, 42);
    assert_eq!(row.total_mapsets, 10);
    assert_eq!(row.ranked_mapsets, 4);
    assert_eq!(row.main_mode, "osu");
}
