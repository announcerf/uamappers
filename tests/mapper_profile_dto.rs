use chrono::Utc;
use serde_json::json;
use uamappers_api::features::mappers::http::presenters::mapper_profile_to_dto;
use uamappers_api::features::mappers::usecases::{
    MapperChartPoint, MapperCurrentStats, MapperKudosu, MapperLeaderboardPosition,
};

#[test]
fn mapper_profile_to_dto_includes_stats_positions_and_charts() {
    let now = Utc::now();
    let dto = mapper_profile_to_dto(uamappers_api::features::mappers::usecases::MapperProfile {
        mapper: uamappers_api::entities::ua_mapper::Model {
            osu_user_id: 42,
            username: "mapper".to_string(),
            country_code: "UA".to_string(),
            first_seen_at: now,
            last_seen_at: now,
            created_at: now,
            updated_at: now,
        },
        mapper_fingerprint: Some(json!({
            "username": "mapper",
            "country": "Ukraine",
            "countryCode": "UA",
            "avatarUrl": "https://avatar",
            "cover": { "url": "https://cover", "customUrl": null },
            "primaryMode": "osu",
            "mappingFollowers": 10,
            "kudosu": { "available": 2, "total": 50 },
            "badges": [],
            "groups": [],
            "isBng": true,
            "isNat": false,
            "isGmt": false,
            "isProbationBn": false,
            "isFullBn": true,
            "cachedAt": now
        })),
        mapper_stats: Some(MapperCurrentStats {
            total_mapsets: 5,
            ranked_mapsets: 3,
            loved_mapsets: 1,
            guest_mapsets: 2,
            nominated_mapsets: 1,
            graveyard_mapsets: 1,
            pending_mapsets: 0,
            total_playcount: 1000,
            avg_rating: 8.2,
            weighted_rating: 8.4,
            avg_stars: 5.1,
            min_stars: 3.2,
            max_stars: 6.7,
            avg_bpm: 180.0,
            avg_length_seconds: 120.0,
            avg_ar: 9.0,
            avg_cs: 4.0,
            avg_od: 8.5,
            avg_hp: 6.5,
            first_submitted_date: Some(now),
            first_ranked_date: Some(now),
            last_mapset_updated_at: Some(now),
            main_mode: "osu".to_string(),
            mapping_followers: 10,
            kudosu: MapperKudosu {
                total: 50,
                available: 2,
            },
            updated_at: now,
        }),
        leaderboard_positions: vec![MapperLeaderboardPosition {
            leaderboard_key: "ranked".to_string(),
            current_rank: 1,
            previous_rank: Some(3),
            rank_delta: 2,
            measured_at: now,
        }],
        charts: vec![MapperChartPoint {
            snapshot_week: now,
            total_mapsets: 5,
            ranked_mapsets: 3,
            loved_mapsets: 1,
            guest_mapsets: 2,
            nominated_mapsets: 1,
            total_playcount: 1000,
            avg_rating: 8.2,
            avg_stars: 5.1,
            avg_bpm: 180.0,
            avg_length_seconds: 120.0,
            main_mode: "osu".to_string(),
        }],
    });

    assert!(dto.stats.is_some());
    assert_eq!(dto.leaderboard_positions.len(), 1);
    assert_eq!(dto.charts.osu_user_id, 42);
    assert_eq!(dto.charts.points.len(), 1);
    assert_eq!(dto.mapper.bio.username, "mapper");
    assert_eq!(dto.mapper.bio.country_code, "UA");
    assert_eq!(dto.mapper.tracking.cached_at, now);
    assert_eq!(dto.mapper.tracking.created_at, now);
    assert_eq!(dto.stats.expect("stats").kudosu.available, 2);
}
