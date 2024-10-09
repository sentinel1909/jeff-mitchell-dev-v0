// src/lib/handlers/article.rs

// dependencies
use crate::domain::{Segments, SharedState};
use crate::errors::AppError;
use crate::queries::query_article;
use crate::templates::{ArticleTemplate, NotFoundTemplate};
use crate::utilities::convert_markdown;
use askama_axum::Template;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use axum_macros::debug_handler;

// enum to represent the response from the article handler function, wraps the template types
enum ArticleHandlerResponse {
    Article(ArticleTemplate),
    NotFound(NotFoundTemplate),
}

// implement the IntoResponse template for the blog handler response type
impl IntoResponse for ArticleHandlerResponse {
    fn into_response(self) -> Response {
        match self {
            ArticleHandlerResponse::Article(template) => match template.render() {
                Ok(content) => Html(content).into_response(),
                Err(_) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
                }
            },
            ArticleHandlerResponse::NotFound(template) => match template.render() {
                Ok(content) => Html(content).into_response(),
                Err(_) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
                }
            },
        }
    }
}

// articles handler function
#[debug_handler]
pub async fn article(
    State(app_state): State<SharedState>,
    Path(segments): Path<Segments>,
) -> Result<impl IntoResponse, AppError> {
    let pool = app_state
        .read()
        .map_err(|_| AppError::LockPoisoned)?
        .pool
        .clone();
    match query_article(segments, pool).await {
        Ok(article) => Ok(ArticleHandlerResponse::Article(ArticleTemplate {
            page_title: article.title,
            article: convert_markdown(article.content),
        })),
        Err(e) => Ok(ArticleHandlerResponse::NotFound(NotFoundTemplate {
            error: e.to_string(),
        })),
    }
}
