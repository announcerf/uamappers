use chrono::Utc;
use serde_json::json;
use uamappers_api::features::beatmapsets::http::presenters::beatmapset_details_to_dto;

#[test]
fn beatmapset_details_to_dto_builds_headline_and_difficulty_sections() {
    let now = Utc::now();
    let details = uamappers_api::features::beatmapsets::usecases::BeatmapsetDetails {
        beatmapset: uamappers_api::entities::beatmapset_profile::Model {
            osu_beatmapset_id: 1000,
            creator_id: 42,
            creator_name: "Mapper".to_string(),
            artist: "Artist".to_string(),
            artist_unicode: None,
            title: "Title".to_string(),
            title_unicode: None,
            source: String::new(),
            tags: String::new(),
            genre: Some("anime".to_string()),
            language: Some("japanese".to_string()),
            status: "ranked".to_string(),
            submitted_date: Some(now),
            ranked_date: Some(now),
            last_updated: now,
            discussion_enabled: true,
            discussion_locked: false,
            can_be_hyped: false,
            is_scoreable: true,
            download_disabled: false,
            nsfw: false,
            video: true,
            storyboard: false,
            spotlight: false,
            playcount: 1000,
            favourite_count: 100,
            rating: 8.5,
            hype_current: 0,
            hype_required: 0,
            nominations_current: 1,
            cover_url: "cover".to_string(),
            card_url: "card".to_string(),
            preview_url: "preview".to_string(),
            bpm: 180.0,
            difficulty_count: 1,
            cached_at: now,
            created_at: now,
            updated_at: now,
        },
        beatmaps: vec![uamappers_api::entities::beatmap_profile::Model {
            osu_beatmap_id: 2000,
            osu_beatmapset_id: 1000,
            creator_id: 42,
            version: "Insane".to_string(),
            mode: "osu".to_string(),
            stars: 5.3,
            ar: 9.0,
            cs: 4.0,
            od: 8.5,
            hp: 6.0,
            bpm: 180.0,
            seconds_total: 120,
            seconds_drain: 100,
            max_combo: Some(1234),
            playcount: 1000,
            passcount: 500,
            count_circles: 100,
            count_sliders: 50,
            count_spinners: 2,
            owners_json: json!([{ "id": 42, "username": "Mapper" }]),
            status: "ranked".to_string(),
            is_scoreable: true,
            last_updated: now,
            cached_at: now,
            created_at: now,
            updated_at: now,
        }],
        charts: vec![],
    };

    let dto = beatmapset_details_to_dto(details);

    assert_eq!(dto.beatmapset.osu_beatmapset_id, 1000);
    assert_eq!(dto.headline_stats.difficulty_count, 1);
    assert_eq!(dto.difficulty_overview.len(), 1);
    assert_eq!(dto.difficulty_details.len(), 1);
    assert!(dto.headline_stats.avg_pass_rate > 0.0);
}
