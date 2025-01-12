use anyhow::{Ok, Result};

use crate::{
    db::{launcher, launcher_resource},
    DatabaseManager,
};

/// 复制启动器,包含启动器的关联资源.新的启动器将会获得一个基于源启动器名称拼接随机字符串的新名称
pub async fn copy_launcher(db: &DatabaseManager, launcher_id: i64) -> Result<i64> {
    // 复制启动器,包含启动器关联的资源数据

    let mut tx = db.pool.begin().await?;

    // 1. 复制启动器
    let launcher = launcher::find(&mut tx, launcher_id).await?;

    let launcher_resoures = launcher_resource::query_by_launcher_id(&mut tx, launcher_id).await?;

    // 2. 复制资源
    let new_launcher_id = launcher::create(&mut tx, &launcher.name).await?;

    for res in launcher_resoures.iter() {
        launcher_resource::create(&mut tx, new_launcher_id, &res.path).await?;
    }

    tx.commit().await?;

    Ok(new_launcher_id)
}
