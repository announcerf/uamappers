use chrono::{TimeZone, Utc};
use serde_json::json;
use uamappers_api::entities::{beatmap_profile, beatmapset_profile, osu_user_beatmapset};
use uamappers_api::features::mappers::storage::{
    codes::{kind_code, mode_code, status_code},
    osu_user_fingerprint::{MapperFingerprint, MapperKudosu},
};
use uamappers_worker::features::ingest::worker::jobs::mapper_enrich::aggregate::build_mapper_stats_row;

#[test]
fn aggregate_computes_primary_metrics() {
    let mapper_profile = MapperFingerprint {
        username: "Mapper".to_string(),
        country: "Ukraine".to_string(),
        country_code: "UA".to_string(),
        avatar_url: "https://avatar".to_string(),
        cover: json!({"url": "https://cover"}),
        primary_mode: "osu".to_string(),
        mapping_followers: 15,
        kudosu: MapperKudosu {
            total: 7,
            available: 2,
        },
        badges: json!([]),
        groups: json!([]),
        is_bng: false,
        is_nat: false,
        is_gmt: false,
        is_probation_bn: false,
        is_full_bn: false,
        cached_at: Utc::now(),
    };

    let relations = vec![
        relation(42, "ranked", 100),
        relation(42, "guest", 101),
        relation(42, "nominated", 101),
    ];

    let beatmapsets = vec![
        mapset(100, "ranked", 1000, 50, 8.0),
        mapset(101, "loved", 500, 20, 9.0),
    ];

    let beatmaps = vec![
        beatmap(100, 5.0, "osu", 180.0, 120),
        beatmap(101, 6.0, "osu", 200.0, 150),
    ];

    let row = build_mapper_stats_row(42, Some(&mapper_profile), &relations, &beatmapsets, &beatmaps);

    assert_eq!(row.total_mapsets, 2);
    assert_eq!(row.ranked_mapsets, 1);
    assert_eq!(row.guest_mapsets, 1);
    assert_eq!(row.nominated_mapsets, 1);
    assert_eq!(row.loved_mapsets, 1);
    assert_eq!(row.total_playcount, 1500);
    assert_eq!(row.mapping_followers, 15);
    assert_eq!(row.kudosu_available, 2);
    assert_eq!(row.kudosu_total, 7);
    assert_eq!(row.main_mode, mode_code("osu"));
}

fn relation(osu_user_id: i64, kind: &str, osu_beatmapset_id: i64) -> osu_user_beatmapset::Model {
    osu_user_beatmapset::Model {
        osu_user_id,
        kind: kind_code(kind),
        osu_beatmapset_id,
        updated_at: Utc::now(),
    }
}

fn mapset(
    osu_beatmapset_id: i64,
    status: &str,
    playcount: i64,
    favourite_count: i64,
    rating: f32,
) -> beatmapset_profile::Model {
    beatmapset_profile::Model {
        osu_beatmapset_id,
        artist: "Artist".to_string(),
        artist_unicode: None,
        title: "Title".to_string(),
        title_unicode: None,
        source: String::new(),
        tags: String::new(),
        genre: None,
        language: None,
        status: status_code(status),
        submitted_date: Some(Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap()),
        ranked_date: Some(Utc.with_ymd_and_hms(2024, 2, 1, 0, 0, 0).unwrap()),
        last_updated: Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap(),
        nsfw: false,
        video: false,
        storyboard: false,
        spotlight: false,
        playcount,
        favourite_count,
        rating,
        cover_url: String::new(),
        card_url: String::new(),
        bpm: 180.0,
        cached_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

fn beatmap(
    osu_beatmapset_id: i64,
    stars: f32,
    mode: &str,
    bpm: f32,
    seconds_total: i32,
) -> beatmap_profile::Model {
    beatmap_profile::Model {
        osu_beatmap_id: osu_beatmapset_id * 10,
        osu_beatmapset_id,
        version: "Insane".to_string(),
        mode: mode_code(mode),
        stars,
        ar: 9.0,
        cs: 4.0,
        od: 8.0,
        hp: 6.0,
        bpm,
        seconds_total,
        seconds_drain: seconds_total - 10,
        max_combo: Some(1000),
        playcount: 100,
        passcount: 50,
        count_circles: 100,
        count_sliders: 50,
        count_spinners: 1,
        owners_json: json!([]),
        status: status_code("ranked"),
        last_updated: Utc::now(),
        cached_at: Utc::now(),
        updated_at: Utc::now(),
    }
}
