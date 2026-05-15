use sqlx::SqlitePool;

pub async fn run_migrations(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS meliinda_sequences (
            id         TEXT PRIMARY KEY,
            label      TEXT,
            events     TEXT NOT NULL,   -- JSON blob : Vec<KeyEvent>
            created_at TEXT NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await?;
    Ok(())
}
