// src/lib/queries/query_articles_by_category.rs

// dependencies
use crate::domain::Article;
use crate::errors::AppError;
use sqlx::PgPool;

// function to get all articles by category from the database
pub async fn query_articles_by_category(
    category: String,
    pool: PgPool,
) -> Result<Vec<Article>, AppError> {
    let articles = sqlx::query_as::<_, Article>(
        "SELECT * FROM articles WHERE article_category = $1 ORDER BY article_date DESC",
    )
    .bind(&category)
    .fetch_all(&pool)
    .await?;
    Ok(articles)
}
