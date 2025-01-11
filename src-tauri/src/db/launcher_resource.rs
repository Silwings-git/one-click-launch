use sqlx::SqlitePool;

/// launcher_resource表对应的数据结构,处理增删改查
pub struct LauncherResourceDb {
    pool: SqlitePool,
}

/// 使用FromRow宏把数据库中读取出来的数据转换成LauncherResource结构
#[allow(dead_code)]
#[derive(sqlx::FromRow, Debug)]
pub struct LauncherResource {
    id: i64,
    launcher_id: i64,
    path: String,
}

impl LauncherResourceDb {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    // 新增
    // 按launcher_id删除
    // 按id删除
    // 按launcher_id查询
    // 查询全部
    //
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
