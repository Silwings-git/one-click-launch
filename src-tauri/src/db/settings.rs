use anyhow::Result;
use sqlx::Executor;
use sqlx::Sqlite;

#[derive(Debug, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct Settings {
    pub key: String,
    pub value: String,
}

pub async fn initialize<'a, E>(executor: E) -> Result<()>
where
    E: Executor<'a, Database = Sqlite>,
{
    sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS settings(
                key      String     PRIMARY KEY NOT NULL,
                value    String     NOT NULL);
                CREATE INDEX IF NOT EXISTS idx_settings_key ON settings(key);"#,
    )
    .execute(executor)
    .await?;

    Ok(())1
}

/// 存储一个设置
pub async fn save<'a, E>(executor: E, settings: &Settings) -> Result<()>
where
    E: Executor<'a, Database = Sqlite>,
{
    sqlx::query("INSERT OR REPLACE INTO settings (key, value) VALUES (?, ?);")
        .bind(&settings.key)
        .bind(&settings.value)
        .execute(executor)
        .await?;
    Ok(())
}

/// 读取一个设置
pub async fn read<'a, E>(executor: E, key: &str) -> Result<Option<Settings>>
where
    E: Executor<'a, Database = Sqlite>,
{
    let settings = sqlx::query_as("SELECT key,value FROM settings WHERE key = ?;")
        .bind(key)
        .fetch_optional(executor)
        .await?;
    Ok(settings)
}

/// 读取全部设置
pub async fn read_all<'a, E>(executor: E) -> Result<Vec<Settings>>
where
    E: Executor<'a, Database = Sqlite>,
{
    let settings = sqlx::query_as("SELECT key,value FROM settings;")
        .fetch_all(executor)
        .await?;
    Ok(settings)
}
