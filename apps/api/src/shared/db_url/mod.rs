fn required(name: &str) -> Result<String, String> {
    std::env::var(name).map_err(|_| format!("{name} is required"))
}

pub fn build_database_url() -> Result<String, String> {
    let host = required("POSTGRES_HOST")?;
    let port = required("POSTGRES_PORT")?;
    let user = required("POSTGRES_USER")?;
    let password = required("POSTGRES_PASSWORD")?;
    let db = required("POSTGRES_DB")?;

    Ok(format!(
        "postgres://{}:{}@{}:{}/{}",
        user, password, host, port, db
    ))
}
