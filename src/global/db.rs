use async_std::task;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::{
    sync::{Arc, OnceLock},
    time::Duration,
};

pub async fn db() -> &'static DatabaseConnection {
    println!("db exec 1");
    static DB: OnceLock<Arc<DatabaseConnection>> = OnceLock::new();
    // println!("db exec 2");
    DB.get_or_init(|| task::block_on(init_db()))
}

async fn init_db() -> Arc<DatabaseConnection> {
    let mut opt = ConnectOptions::new("postgres://postgres:yuAU702G!@124.222.103.232/postgres");
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .set_schema_search_path("public"); // 配置默认的schema

    let db = Database::connect(opt).await.unwrap();

    Arc::new(db)
}
