use sea_orm::DatabaseConnection;

use crate::features::{ingest::storage::repo::ScanStateRepo, mappers::storage::repo::MapperRepo};

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub mappers_repo: MapperRepo,
    pub scan_state_repo: ScanStateRepo,
}

impl AppState {
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            mappers_repo: MapperRepo::new(db.clone()),
            scan_state_repo: ScanStateRepo::new(db.clone()),
            db,
        }
    }
}
