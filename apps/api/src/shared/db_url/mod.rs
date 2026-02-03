pub fn build_database_url() -> String {
    let host = std::env::var("POSTGRES_HOST").unwrap_or_else(|_| "localhost".to_string());
    let port = std::env::var("POSTGRES_PORT").unwrap_or_else(|_| "5432".to_string());
    let user = std::env::var("POSTGRES_USER").unwrap_or_else(|_| "postgres".to_string());
    let password = std::env::var("POSTGRES_PASSWORD").unwrap_or_else(|_| "postgres".to_string());
    let db = std::env::var("POSTGRES_DB").unwrap_or_else(|_| "uamappers".to_string());

    format!("postgres://{}:{}@{}:{}/{}", user, password, host, port, db)
}
