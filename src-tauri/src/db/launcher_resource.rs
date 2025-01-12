use anyhow::Result;
use sqlx::{Executor, Sqlite, SqlitePool};

/// 使用FromRow宏把数据库中读取出来的数据转换成LauncherResource结构
#[allow(dead_code)]
#[derive(sqlx::FromRow, Debug)]
pub struct LauncherResource {
    pub id: i64,
    pub launcher_id: i64,
    pub path: String,
}

pub async fn initialize<'a, E>(executor: E) -> Result<()>
where
    E: Executor<'a, Database = Sqlite>,
{
    sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS launcher_resource(
            id          INTEGER PRIMARY KEY NOT NULL,
            launcher_id INTEGER             NOT NULL,
            path        VARCHAR             NOT NULL);
            CREATE INDEX IF NOT EXISTS idx_launcherresource_launcherid ON launcher_resource(launcher_id);"#,
    )
    .execute(executor)
    .await?;
    Ok(())
}

// 新增
pub async fn create<'a, E>(executor: E, launcher_id: i64, path: &str) -> Result<i64>
where
    E: Executor<'a, Database = Sqlite>,
{
    let id = sqlx::query("INSERT INTO launcher_resource (launcher_id,path) VALUES (?,?)")
        .bind(launcher_id)
        .bind(path)
        .execute(executor)
        .await?
        .last_insert_rowid();
    Ok(id)
}

// 按launcher_id删除
pub async fn delete_by_launcher<'a, E>(executor: E, launcher_id: i64) -> Result<()>
where
    E: Executor<'a, Database = Sqlite>,
{
    sqlx::query("DELETE FROM launcher_resource WHERE launcher_id=?")
        .bind(launcher_id)
        .execute(executor)
        .await?;
    Ok(())
}

// 按id删除
pub async fn delete_by_id<'a, E>(executor: E, id: i64) -> Result<()>
where
    E: Executor<'a, Database = Sqlite>,
{
    sqlx::query("DELETE FROM launcher_resource WHERE id = ?")
        .bind(id)
        .execute(executor)
        .await?;
    Ok(())
}

// 按launcher_id查询
pub async fn query_by_launcher_id<'a, E>(
    executor: E,
    launcher_id: i64,
) -> Result<Vec<LauncherResource>>
where
    E: Executor<'a, Database = Sqlite>,
{
    let resources =
        sqlx::query_as("SELECT id,launcher_id,path FROM launcher_resource WHERE launcher_id=?")
            .bind(launcher_id)
            .fetch_all(executor)
            .await?;
    Ok(resources)
}

// 查询全部
pub async fn query_all<'a, E>(executor: E) -> Result<Vec<LauncherResource>>
where
    E: Executor<'a, Database = Sqlite>,
{
    let resources =
        sqlx::query_as("SELECT id,launcher_id,path FROM launcher_resource ORDER BY id DESC")
            .fetch_all(executor)
            .await?;
    Ok(resources)
}

/// 重新创建 launcher_resource 表
pub async fn recreate_table(pool: &SqlitePool) -> anyhow::Result<()> {
    sqlx::query("DROP TABLE IF EXISTS launcher_resource")
        .execute(pool)
        .await?;
    sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS launcher_resource(
                id          INTEGER PRIMARY KEY NOT NULL,
                launcher_id INTEGER             NOT NULL,
                path        VARCHAR             NOT NULL)"#,
    )
    .execute(pool)
    .await?;
    Ok(())
}
