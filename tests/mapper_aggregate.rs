use chrono::{TimeZone, Utc};
use serde_json::json;
use uamappers_api::entities::{beatmap_profile, beatmapset_profile, mapper_profile, osu_user_beatmapset};
use uamappers_worker::features::ingest::worker::jobs::mapper_enrich::aggregate::build_mapper_stats_row;

#[test]
fn aggregate_computes_primary_metrics() {
    let mapper_profile = mapper_profile::Model {
        osu_user_id: 42,
        username: "Mapper".to_string(),
        avatar_url: "https://avatar".to_string(),
        country: "Ukraine".to_string(),
        country_code: "UA".to_string(),
        cover_url: "https://cover".to_string(),
        primary_mode: "osu".to_string(),
        join_date: Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap(),
        last_visit: None,
        mapping_followers: 15,
        kudosu_available: 2,
        kudosu_total: 7,
        badges_json: json!([]),
        groups_json: json!([]),
        is_bng: false,
        is_nat: false,
        is_gmt: false,
        is_limited_bn: false,
        is_full_bn: false,
        cached_at: Utc::now(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
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
    assert_eq!(row.kudosu_total, 7);
    assert_eq!(row.main_mode, "osu");
    assert!(row.has_ranked);
    assert!(row.has_guest);
    assert!(row.has_nominated);
}

fn relation(osu_user_id: i64, kind: &str, osu_beatmapset_id: i64) -> osu_user_beatmapset::Model {
    osu_user_beatmapset::Model {
        osu_user_id,
        kind: kind.to_string(),
        osu_beatmapset_id,
        created_at: Utc::now(),
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
        creator_id: 42,
        creator_name: "Mapper".to_string(),
        artist: "Artist".to_string(),
        artist_unicode: None,
        title: "Title".to_string(),
        title_unicode: None,
        source: String::new(),
        tags: String::new(),
        genre: None,
        language: None,
        status: status.to_string(),
        submitted_date: Some(Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap()),
        ranked_date: Some(Utc.with_ymd_and_hms(2024, 2, 1, 0, 0, 0).unwrap()),
        last_updated: Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap(),
        discussion_enabled: true,
        discussion_locked: false,
        can_be_hyped: false,
        is_scoreable: true,
        download_disabled: false,
        nsfw: false,
        video: false,
        storyboard: false,
        spotlight: false,
        playcount,
        favourite_count,
        rating,
        hype_current: 0,
        hype_required: 0,
        nominations_current: 0,
        cover_url: String::new(),
        card_url: String::new(),
        preview_url: String::new(),
        bpm: 180.0,
        difficulty_count: 1,
        cached_at: Utc::now(),
        created_at: Utc::now(),
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
        creator_id: 42,
        version: "Insane".to_string(),
        mode: mode.to_string(),
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
        status: "ranked".to_string(),
        is_scoreable: true,
        last_updated: Utc::now(),
        cached_at: Utc::now(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}
