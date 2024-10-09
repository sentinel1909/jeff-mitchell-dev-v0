// src/lib/startup.rs

// dependencies
use crate::domain::SharedState;
use crate::errors::AppError;
use crate::handlers::{
    about, article, articles, games, get_articles_by_category, get_articles_by_tag, handler_404,
    health_check, index, music, photography, projects,
};
use crate::queries::query_insert_articles;
use crate::utilities::{get_bodies, get_frontmatters};
use axum::{routing::get, Router};
use sqlx::Executor;
use tower_http::{
    cors::CorsLayer,
    services::ServeDir,
    trace::{self, TraceLayer},
};
use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub async fn build_content(app_state: &SharedState) -> Result<(), AppError> {
    // add the articles to the database
    let pool = app_state
        .read()
        .map_err(|_| AppError::LockPoisoned)?
        .pool
        .clone();
    query_insert_articles(pool, get_frontmatters()?, get_bodies()?).await?;

    Ok(())
}

// configure router with handlers, file serving, and state
pub fn build_router(app_state: SharedState) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/about", get(about))
        .route("/articles", get(articles))
        .route("/articles_category", get(get_articles_by_category))
        .route("/articles_tag", get(get_articles_by_tag))
        .route("/article/:date/:slug", get(article))
        .route("/music", get(music))
        .route("/games", get(games))
        .route("/projects", get(projects))
        .route("/photography", get(photography))
        .route("/health_check", get(health_check))
        .layer(CorsLayer::permissive())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_request(trace::DefaultOnRequest::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        .nest_service("/assets", ServeDir::new("assets"))
        .fallback(handler_404)
        .with_state(app_state)
}

// database migrations
pub async fn database_migrations(app_state: &SharedState) -> Result<(), AppError> {
    let pool = app_state
        .read()
        .map_err(|_| AppError::LockPoisoned)?
        .pool
        .clone();
    pool.execute(include_str!(
        "../../migrations/20240331195133_articles.up.sql"
    ))
    .await
    .map_err(AppError::Database)?;

    Ok(())
}

// initialize tracing
pub fn initialize_tracing() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();
}
