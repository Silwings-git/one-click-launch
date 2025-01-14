use anyhow::{Ok, Result};
use sqlx::{Executor, Sqlite, SqlitePool};

/// 使用 FromRow 派生宏把从数据库中读取出来的数据转换成 Launcher 结构
#[allow(dead_code)]
#[derive(sqlx::FromRow, Debug)]
pub struct Launcher {
    pub id: i64,
    pub name: String,
    pub sort: i32,
}

pub async fn initialize<'a, E>(executor: E) -> Result<()>
where
    E: Executor<'a, Database = Sqlite>,
{
    sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS launcher(
                id          INTEGER PRIMARY KEY NOT NULL,
                name        VARCHAR             NOT NULL,
                sort        INTEGER             NOT NULL DEFAULT 1)"#,
    )
    .execute(executor)
    .await?;
    Ok(())
}

/// 存储一个新的启动器
pub async fn create<'a, E>(executor: E, launcher_name: &str) -> Result<i64>
where
    E: Executor<'a, Database = Sqlite>,
{
    let id = sqlx::query("INSERT INTO launcher (name) VALUES (?)")
        .bind(launcher_name)
        .execute(executor)
        .await?
        .last_insert_rowid();
    Ok(id)
}

/// 修改启动器名称
pub async fn modify_launcher_name<'a, E>(executor: E, id: i64, name: &str) -> Result<()>
where
    E: Executor<'a, Database = Sqlite>,
{
    sqlx::query("UPDATE launcher SET name = ? WHERE id = ?")
        .bind(name)
        .bind(id)
        .execute(executor)
        .await?;
    Ok(())
}

/// 修改启动器顺序
pub async fn modify_launcher_sort<'a, E>(executor: E, id: i64, sort: i32) -> Result<()>
where
    E: Executor<'a, Database = Sqlite>,
{
    sqlx::query("UPDATE launcher SET sort = ? WHERE id = ?")
        .bind(sort)
        .bind(id)
        .execute(executor)
        .await?;
    Ok(())
}

/// 删除启动器
pub async fn delete_by_id<'a, E>(executor: E, id: i64) -> Result<()>
where
    E: Executor<'a, Database = Sqlite>,
{
    sqlx::query("DELETE FROM launcher WHERE id = ?")
        .bind(id)
        .execute(executor)
        .await?;
    Ok(())
}

/// 查询启动器列表
pub async fn query<'a, E>(executor: E) -> Result<Vec<Launcher>>
where
    E: Executor<'a, Database = Sqlite>,
{
    let launchers: Vec<Launcher> =
        sqlx::query_as("SELECT id,name,sort FROM launcher ORDER BY sort ASC, id DESC")
            .fetch_all(executor)
            .await?;
    Ok(launchers)
}

/// 查询单个启动器信息
pub async fn find_by_id<'a, E>(executor: E, id: i64) -> Result<Launcher>
where
    E: Executor<'a, Database = Sqlite>,
{
    let launcher = sqlx::query_as("SELECT id,name,sort FROM launcher WHERE id = ?")
        .bind(id)
        .fetch_one(executor)
        .await?;

    Ok(launcher)
}

/// 重新创建 launcher 表
pub async fn recreate_table(pool: &SqlitePool) -> Result<()> {
    // TODO
    sqlx::query("DROP TABLE IF EXISTS launcher")
        .execute(pool)
        .await?;
    sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS launcher(
                id          INTEGER PRIMARY KEY NOT NULL,
                name        VARCHAR             NOT NULL,
                sort        INTEGER             NOT NULL)"#,
    )
    .execute(pool)
    .await?;
    Ok(())
}
