#![cfg(feature = "server")]

use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;
use std::sync::OnceLock;

// 静态全局数据库连接池
pub static POOL: OnceLock<PgPool> = OnceLock::new();
//pub static DB: &OnceLock<PgPool> = &POOL;
pub async fn create_pool() {
    dotenvy::dotenv().ok();

    let database_url  = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://dbuser:dbpass@localhost:5432/dioxusdb".to_string());
    // ⚡ 使用 PgPoolOptions 进行精细化配置
    let pool = PgPoolOptions::new()
        // 1. 设置最大连接数（根据业务并发量调整，默认是 10）
        .max_connections(20)
        // 2. 设置最小保持连接数（避免冷启动延迟）
        .min_connections(5) 
        // 3. 建立连接的超时时间（防止数据库挂掉时后端无限卡死）
        .acquire_timeout(Duration::from_secs(10))
        // 4. 空闲连接回收时间（释放长期不用的连接，节约省数据库资源）
        .idle_timeout(Duration::from_secs(600)) 
        .connect(&database_url )
        .await
        .expect("Failed to create configured PostgreSQL pool");

    // (可选) 自动执行数据库迁移
    // sqlx::migrate!("./migrations").run(&pool).await.expect("Migration failed");


        POOL.set(pool).expect("Failed to set global DB pool");
}

pub fn pool() -> &'static PgPool {
        POOL.get().expect("Database pool is not initialized")
}
