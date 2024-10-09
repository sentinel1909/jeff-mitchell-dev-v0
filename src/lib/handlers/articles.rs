// src/lib/handlers/blog.rs

// dependencies
use crate::domain::SharedState;
use crate::errors::AppError;
use crate::queries::query_articles;
use crate::templates::{ArticlesTemplate, NotFoundTemplate};
use askama_axum::Template;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use axum_macros::debug_handler;

// enum to represent the response from the blog handler function, wraps the template types
enum BlogHandlerResponse {
    Articles(ArticlesTemplate),
    NotFound(NotFoundTemplate),
}

// implement the IntoResponse template for the blog handler response type
impl IntoResponse for BlogHandlerResponse {
    fn into_response(self) -> Response {
        match self {
            BlogHandlerResponse::Articles(template) => match template.render() {
                Ok(content) => Html(content).into_response(),
                Err(_) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
                }
            },
            BlogHandlerResponse::NotFound(template) => match template.render() {
                Ok(content) => Html(content).into_response(),
                Err(_) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
                }
            },
        }
    }
}

// blog handler function
#[debug_handler]
pub async fn articles(State(app_state): State<SharedState>) -> Result<impl IntoResponse, AppError> {
    let pool = app_state
        .read()
        .map_err(|_| AppError::LockPoisoned)?
        .pool
        .clone();
    match query_articles(pool).await {
        Ok(articles) => Ok(BlogHandlerResponse::Articles(ArticlesTemplate { articles })),
        Err(e) => Ok(BlogHandlerResponse::NotFound(NotFoundTemplate {
            error: e.to_string(),
        })),
    }
}
