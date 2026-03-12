use chrono::Utc;
use serde_json::{Value, json};
use uamappers_api::features::beatmapsets::http::dto::{
    BeatmapDifficultyDetailDto, BeatmapDifficultyOverviewDto, BeatmapsetAnalyticsDto,
    BeatmapsetChartsPointDto, BeatmapsetChartsResponseDto, BeatmapsetDetailsDto,
    BeatmapsetHeaderDto, BeatmapsetHeadlineStatsDto,
};
use uamappers_api::features::leaderboards::http::dto::LeaderboardKeyDto;
use uamappers_api::features::mappers::http::dto::{
    MapperBioDto, MapperChartsPointDto, MapperChartsResponseDto, MapperDto, MapperKudosuDto,
    MapperLeaderboardPositionDto, MapperStatsCurrentDto, MapperTrackingDto, UaMapperProfileDto,
};

#[test]
fn mapper_profile_serializes_in_camel_case() {
    let now = Utc::now();
    let dto = UaMapperProfileDto {
        mapper: MapperDto {
            bio: MapperBioDto {
                osu_user_id: 1,
                username: "mapper".to_string(),
                country: "Ukraine".to_string(),
                country_code: "UA".to_string(),
                avatar_url: "a".to_string(),
                cover: json!({"url": "c", "customUrl": null}),
                badges: json!([]),
                groups: json!([]),
                primary_mode: "osu".to_string(),
                is_bng: false,
                is_nat: false,
                is_gmt: false,
                is_probation_bn: false,
                is_full_bn: true,
            },
            tracking: MapperTrackingDto {
                cached_at: now,
                first_seen_at: now,
                last_seen_at: now,
                created_at: now,
                updated_at: now,
            },
        },
        stats: Some(MapperStatsCurrentDto {
            total_mapsets: 1,
            ranked_mapsets: 1,
            loved_mapsets: 0,
            guest_mapsets: 0,
            nominated_mapsets: 0,
            graveyard_mapsets: 0,
            pending_mapsets: 0,
            total_playcount: 10,
            avg_rating: 8.0,
            weighted_rating: 8.0,
            avg_stars: 5.0,
            min_stars: 5.0,
            max_stars: 5.0,
            avg_bpm: 180.0,
            avg_length_seconds: 120.0,
            avg_ar: 9.0,
            avg_cs: 4.0,
            avg_od: 8.0,
            avg_hp: 6.0,
            first_submitted_date: Some(now),
            first_ranked_date: Some(now),
            last_mapset_updated_at: Some(now),
            main_mode: "osu".to_string(),
            mapping_followers: 1,
            kudosu: MapperKudosuDto {
                total: 3,
                available: 2,
            },
            has_ranked: true,
            has_loved: false,
            has_guest: false,
            has_nominated: false,
            updated_at: now,
        }),
        leaderboard_positions: vec![MapperLeaderboardPositionDto {
            leaderboard_key: "ranked".to_string(),
            current_rank: 1,
            previous_rank: Some(2),
            rank_delta: 1,
            measured_at: now,
        }],
        charts: MapperChartsResponseDto {
            osu_user_id: 1,
            points: vec![MapperChartsPointDto {
                snapshot_week: now,
                total_mapsets: 1,
                ranked_mapsets: 1,
                loved_mapsets: 0,
                guest_mapsets: 0,
                nominated_mapsets: 0,
                total_playcount: 10,
                avg_rating: 8.0,
                avg_stars: 5.0,
                avg_bpm: 180.0,
                avg_length_seconds: 120.0,
                main_mode: "osu".to_string(),
            }],
        },
    };

    let value = serde_json::to_value(dto).expect("serialize mapper profile");
    assert_camel_case_keys(&value);
    assert!(value.get("leaderboardPositions").is_some());
    assert!(value["mapper"].get("bio").is_some());
    assert!(value["mapper"].get("tracking").is_some());
    assert!(value.get("leaderboard_positions").is_none());
}

#[test]
fn beatmapset_details_and_leaderboard_key_serialize_in_camel_case() {
    let now = Utc::now();
    let dto = BeatmapsetDetailsDto {
        beatmapset: BeatmapsetHeaderDto {
            osu_beatmapset_id: 10,
            title: "Title".to_string(),
            title_unicode: None,
            artist: "Artist".to_string(),
            artist_unicode: None,
            creator_name: "Mapper".to_string(),
            status: "ranked".to_string(),
            genre: None,
            language: None,
            source: String::new(),
            tags: String::new(),
            cover_url: "cover".to_string(),
            card_url: "card".to_string(),
            preview_url: "preview".to_string(),
            submitted_date: Some(now),
            ranked_date: Some(now),
            last_updated: now,
            nsfw: false,
            video: true,
            storyboard: false,
            spotlight: false,
        },
        headline_stats: BeatmapsetHeadlineStatsDto {
            playcount: 10,
            favourite_count: 5,
            rating: 8.0,
            fav_play_ratio: 0.5,
            difficulty_count: 1,
            avg_pass_rate: 0.5,
            avg_stars: 5.0,
        },
        difficulty_overview: vec![BeatmapDifficultyOverviewDto {
            osu_beatmap_id: 20,
            version: "Insane".to_string(),
            mode: "osu".to_string(),
            stars: 5.0,
            ar: 9.0,
            cs: 4.0,
            od: 8.0,
            hp: 6.0,
            bpm: 180.0,
            seconds_total: 120,
            playcount: 10,
            passcount: 5,
            pass_rate: 0.5,
        }],
        difficulty_details: vec![BeatmapDifficultyDetailDto {
            osu_beatmap_id: 20,
            version: "Insane".to_string(),
            mode: "osu".to_string(),
            stars: 5.0,
            status: "ranked".to_string(),
            ar: 9.0,
            cs: 4.0,
            od: 8.0,
            hp: 6.0,
            bpm: 180.0,
            seconds_total: 120,
            seconds_drain: 100,
            max_combo: Some(1000),
            playcount: 10,
            passcount: 5,
            pass_rate: 0.5,
            objects: 100,
            owners: json!([]),
            count_circles: 50,
            count_sliders: 40,
            count_spinners: 10,
            last_updated: now,
        }],
        analytics: BeatmapsetAnalyticsDto {
            charts: BeatmapsetChartsResponseDto {
                osu_beatmapset_id: 10,
                points: vec![BeatmapsetChartsPointDto {
                    snapshot_week: now,
                    status: "ranked".to_string(),
                    playcount: 10,
                    favourite_count: 5,
                    rating: 8.0,
                    avg_passcount: 5.0,
                    avg_pass_rate: 0.5,
                    min_pass_rate: 0.5,
                    max_pass_rate: 0.5,
                    last_updated: now,
                }],
            },
        },
    };

    let value = serde_json::to_value(dto).expect("serialize beatmapset details");
    assert_camel_case_keys(&value);
    assert_eq!(
        serde_json::to_value(LeaderboardKeyDto::GuestDiff).expect("serialize leaderboard key"),
        Value::String("guestDiff".to_string())
    );
}

fn assert_camel_case_keys(value: &Value) {
    match value {
        Value::Object(map) => {
            for (key, nested) in map {
                assert!(
                    !key.contains('_'),
                    "key must be camelCase, found snake_case key: {key}"
                );
                assert_camel_case_keys(nested);
            }
        }
        Value::Array(items) => {
            for item in items {
                assert_camel_case_keys(item);
            }
        }
        _ => {}
    }
}
