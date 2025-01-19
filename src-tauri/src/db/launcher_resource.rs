use anyhow::Result;
use sqlx::{Executor, Sqlite};

/// 使用FromRow宏把数据库中读取出来的数据转换成LauncherResource结构
#[allow(dead_code)]
#[derive(sqlx::FromRow, Debug)]
pub struct LauncherResource {
    pub id: i64,
    pub launcher_id: i64,
    pub name: String,
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
            name        VARCHAR             NOT NULL,
            path        VARCHAR             NOT NULL);
            CREATE INDEX IF NOT EXISTS idx_launcherresource_launcherid ON launcher_resource(launcher_id);"#,
    )
    .execute(executor)
    .await?;
    Ok(())
}

// 新增
pub async fn create<'a, E>(executor: E, launcher_id: i64, name: &str, path: &str) -> Result<i64>
where
    E: Executor<'a, Database = Sqlite>,
{
    let id = sqlx::query("INSERT INTO launcher_resource (launcher_id,name,path) VALUES (?,?,?)")
        .bind(launcher_id)
        .bind(name)
        .bind(path)
        .execute(executor)
        .await?
        .last_insert_rowid();
    Ok(id)
}

// 修改名称
pub async fn modify_name<'a, E>(executor: E, resource_id: i64, name: &str) -> Result<()>
where
    E: Executor<'a, Database = Sqlite>,
{
    sqlx::query("UPDATE launcher_resource SET name = ? WHERE id = ?")
        .bind(name)
        .bind(resource_id)
        .execute(executor)
        .await?;
    Ok(())
}

// 按launcher_id删除
pub async fn delete_by_launcher<'a, E>(executor: E, launcher_id: i64) -> Result<()>
where
    E: Executor<'a, Database = Sqlite>,
{
    sqlx::query("DELETE FROM launcher_resource WHERE launcher_id = ?")
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
    let resources = sqlx::query_as(
        "SELECT id,launcher_id,name,path FROM launcher_resource WHERE launcher_id=?",
    )
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
        sqlx::query_as("SELECT id,launcher_id,name,path FROM launcher_resource ORDER BY id DESC")
            .fetch_all(executor)
            .await?;
    Ok(resources)
}
