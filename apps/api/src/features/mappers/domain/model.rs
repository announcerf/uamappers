#[derive(Clone, Debug)]
pub struct MapperStats {
    pub osu_user_id: i64,
    pub username: String,
    pub country_code: String,
    pub count_graveyard: i32,
    pub count_pending: i32,
    pub count_wip: i32,
    pub count_loved: i32,
    pub count_ranked: i32,
    pub count_approved: i32,
    pub count_total: i32,
    pub is_bn: bool,
    pub nominated_count: Option<i32>,
}
