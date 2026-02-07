mod app;
mod features;
mod infra;
mod shared;

#[tokio::main]
async fn main() -> Result<(), shared::errors::WorkerError> {
    app::run::run().await
}
