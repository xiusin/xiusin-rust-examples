use async_std::task;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::{
    sync::{Mutex, Once},
    time::Duration,
};

const DB_URL: &str = "postgres://postgres:yuAU702G!@124.222.103.232/postgres";

struct Singleton {
    db: DatabaseConnection,
}

lazy_static::lazy_static! {
    static ref SINGLETON: Mutex<Singleton> = {
        // 阻塞获取
         let db = task::block_on(init_db()).await;
         Mutex::new(Singleton { db: db })
    };
    static ref INIT: Once = Once::new();
}

pub async fn init_db() -> Singleton {
    let mut opt = ConnectOptions::new(DB_URL);

    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .set_schema_search_path("public"); // 配置默认的schema

    let db = Database::connect(opt).await.unwrap();

    return db;
}
