use sea_orm::DbErr;

use crate::{entities::scan_state, features::ingest::storage::repo::ScanStateRepo};

pub async fn list_scan_states(
    scan_state_repo: &ScanStateRepo,
) -> Result<Vec<scan_state::Model>, DbErr> {
    scan_state_repo.list_all().await
}
