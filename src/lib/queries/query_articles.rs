// src/lib/queries/query_articles.rs

// dependencies
use crate::domain::Article;
use crate::errors::AppError;
use sqlx::PgPool;

// function to get all articles from the database
pub async fn query_articles(pool: PgPool) -> Result<Vec<Article>, AppError> {
    let articles =
        sqlx::query_as::<_, Article>("SELECT * FROM articles ORDER BY article_date DESC")
            .fetch_all(&pool)
            .await?;
    Ok(articles)
}
