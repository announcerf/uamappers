use thiserror::Error;

#[derive(Debug, Error)]
pub enum WorkerError {
    #[error("configuration error: {0}")]
    Config(String),
    #[error("osu api error: {0}")]
    OsuApi(#[from] rosu_v2::error::OsuError),
    #[error("database error: {0}")]
    Database(#[from] sea_orm::DbErr),
}
