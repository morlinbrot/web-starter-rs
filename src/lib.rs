use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, patch},
    Router,
};
use sqlx::PgPool;
use tower_cookies::CookieManagerLayer;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use uuid::Uuid;

pub mod prelude;

mod config;
mod error;
mod handlers;
mod models;

use prelude::*;

#[derive(Debug, Clone)]
struct InnerAppState {
    todos: HashMap<Uuid, Todo>,
    _pool: PgPool,
}

type AppState = Arc<RwLock<InnerAppState>>;

pub async fn configure_app(_pool: PgPool) -> Result<Router> {
    let state = Arc::new(RwLock::new(InnerAppState {
        todos: HashMap::new(),
        _pool,
    }));

    let app = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .route(
            "/todos",
            get(handlers::todos::todos_get).post(handlers::todos::todos_post),
        )
        .route(
            "/todos/{id}",
            patch(handlers::todos::todos_patch).delete(handlers::todos::todos_delete),
        )
        .layer(CookieManagerLayer::new())
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
        .fallback(handle_404);

    Ok(app)
}

async fn handle_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "404 - Nothing to see here.")
}
