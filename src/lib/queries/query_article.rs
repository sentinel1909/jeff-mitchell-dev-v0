// src/lib/queries/query_article.rs

// dependencies
use crate::domain::{Article, Segments};
use crate::errors::AppError;
use sqlx::PgPool;

// function to get all articles from the database
pub async fn query_article(segments: Segments, pool: PgPool) -> Result<Article, AppError> {
    let article = sqlx::query_as::<_, Article>(
        "SELECT * FROM articles WHERE article_date = $1 AND article_slug = $2 LIMIT 1",
    )
    .bind(segments.date)
    .bind(segments.slug)
    .fetch_one(&pool)
    .await?;
    Ok(article)
}
