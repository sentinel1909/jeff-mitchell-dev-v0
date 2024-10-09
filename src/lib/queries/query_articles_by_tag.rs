// src/lib/queries/query_articles_by_tag.rs

// dependencies
use crate::domain::Article;
use crate::errors::AppError;
use sqlx::PgPool;

// function to get all articles by category from the database
pub async fn query_articles_by_tag(tag: String, pool: PgPool) -> Result<Vec<Article>, AppError> {
    let articles = sqlx::query_as::<_, Article>(
        "SELECT * FROM articles WHERE article_tag = $1 ORDER BY article_date DESC",
    )
    .bind(&tag)
    .fetch_all(&pool)
    .await?;
    Ok(articles)
}
