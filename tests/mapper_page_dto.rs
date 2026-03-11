use chrono::Utc;
use uamappers_api::entities::ua_mapper;
use uamappers_api::features::mappers::http::presenters::mapper_page_to_dto;
use uamappers_api::features::mappers::usecases::MapperPage;

#[test]
fn mapper_page_to_dto_uses_next_cursor() {
    let now = Utc::now();
    let page = MapperPage {
        items: vec![ua_mapper::Model {
            osu_user_id: 42,
            username: "Mapper".to_string(),
            country_code: "UA".to_string(),
            first_seen_at: now,
            last_seen_at: now,
            created_at: now,
            updated_at: now,
        }],
        next_cursor: Some(42),
        total: 4000,
    };

    let dto = mapper_page_to_dto(page);

    assert_eq!(dto.next_cursor, Some(42));
    assert_eq!(dto.total, 4000);
    assert_eq!(dto.items.len(), 1);
    assert_eq!(dto.items[0].osu_user_id, 42);
}
