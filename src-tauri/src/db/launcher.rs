use anyhow::Result;
use sqlx::SqlitePool;
/// launcher表对应的数据结构,处理增删改查
pub struct LauncherDb {
    pool: SqlitePool,
}

/// 使用 FromRow 派生宏把从数据库中读取出来的数据转换成 Launcher 结构
#[allow(dead_code)]
#[derive(sqlx::FromRow, Debug)]
pub struct Launcher {
    id: i64,
    name: String,
    sort: i32,
}

impl LauncherDb {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// 存储一个新的启动器
    pub async fn create(&self, launcher_name: &str) -> Result<i64> {
        let id = sqlx::query("INSERT INTO launcher (name) VALUES (?)")
            .bind(launcher_name)
            .execute(&self.pool)
            .await?
            .last_insert_rowid();
        Ok(id)
    }

    /// 修改启动器名称
    pub async fn modify_launcher_name(&self, id: i64, name: &str) -> Result<()> {
        sqlx::query("UPDATE launcher SET name = ? WHERE id = ?")
            .bind(name)
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// 修改启动器顺序
    pub async fn modify_launcher_sort(&self, id: i64, sort: i32) -> Result<()> {
        sqlx::query("UPDATE launcher SET sort = ? WHERE id = ?")
            .bind(sort)
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// 删除启动器
    pub async fn delete(&self, id: i64) -> Result<()> {
        sqlx::query("DELETE FROM launcher WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// 查询启动器列表
    pub async fn query(&self) -> Result<Vec<Launcher>> {
        let launchers: Vec<Launcher> = sqlx::query_as("SELECT id,name,sort FROM launcher")
            .fetch_all(&self.pool)
            .await?;
        Ok(launchers)
    }
    
}

/// 重新创建 launcher 表
pub async fn recreate_table(pool: &SqlitePool) -> Result<()> {
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
