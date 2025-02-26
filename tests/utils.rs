use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;

use web_starter_rs::prelude::*;

pub struct TestApp {
    pub address: String,
    pub _cfg: &'static Config,
}

pub async fn spawn_test_app() -> Result<TestApp> {
    let cfg = config();
    let pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(1))
        .connect(&cfg.DATABASE_URL)
        .await
        .expect("Fatal: Failed to connect to database. Is the database running?");

    let app = configure_app(pool).await?;

    let addr = format!("{}:{}", cfg.APP_HOST, 0);
    let listener = TcpListener::bind(addr).await.unwrap();
    let address = format!("http://{}", listener.local_addr().unwrap());

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    Ok(TestApp { _cfg: cfg, address })
}
