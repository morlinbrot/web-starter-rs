use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::EnvFilter;

use web_starter_rs::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let cfg = config();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(1))
        .connect(&cfg.DATABASE_URL)
        .await
        .expect("Fatal: Failed to connect to database. Is the database running?");

    let app = configure_app(pool).await?;

    let addr = format!("{}:{}", cfg.APP_HOST, cfg.APP_PORT);
    let listener = TcpListener::bind(&addr).await.unwrap();
    info!("{:<12} - {addr}\n", "listening on");
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
