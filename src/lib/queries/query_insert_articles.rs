// src/lib/queries/query_insert_articles

// dependencies
use crate::domain::{BodyContent, FrontMatter};
use crate::errors::AppError;
use chrono::prelude::*;
use sqlx::PgPool;

// function to initialize the articles database by inserting the frontmatters and content
// pulled from the content markdown files
pub async fn query_insert_articles(
    pool: PgPool,
    front_matters: Vec<FrontMatter>,
    bodies: Vec<BodyContent>,
) -> Result<(), AppError> {
    for (front_matter, body) in front_matters.into_iter().zip(bodies.into_iter()) {
        if !front_matter.draft {
            let _insert_frontmatter = sqlx::query("INSERT INTO articles (article_title, article_date, article_edited, article_slug, article_category, article_tag, article_summary, article_content) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) ON CONFLICT (article_title) DO NOTHING")
            .bind(&front_matter.title)
            .bind(&front_matter.date)
            .bind(&front_matter.edited_at)
            .bind(&front_matter.slug)
            .bind(&front_matter.category)
            .bind(&front_matter.tag)
            .bind(&front_matter.summary)
            .bind(&body.content)
            .execute(&pool)
            .await?;
        }

        if front_matter.edited {
            let article_id: (i32,) =
                sqlx::query_as("SELECT article_id FROM articles WHERE article_slug = $1")
                    .bind(&front_matter.slug)
                    .fetch_one(&pool)
                    .await?;
            let _insert_frontmatter = sqlx::query("UPDATE articles SET article_title=$1, article_date=$2, article_edited=$3, article_slug=$4, article_category=$5, article_tag=$6, article_summary=$7, article_content=$8 WHERE article_id=$9 AND (article_title IS DISTINCT FROM $1 OR article_date IS DISTINCT FROM $2 OR article_edited IS DISTINCT FROM $3 OR article_slug IS DISTINCT FROM $4 OR article_category IS DISTINCT FROM $5 OR article_tag IS DISTINCT FROM $6 OR article_summary IS DISTINCT FROM $7 OR article_content IS DISTINCT FROM $8)")
            .bind(front_matter.title)
            .bind(front_matter.date)
            .bind(Local::now().format("%Y-%m-%d").to_string())
            .bind(front_matter.slug)
            .bind(front_matter.category)
            .bind(front_matter.tag)
            .bind(front_matter.summary)
            .bind(body.content)
            .bind(article_id.0)
            .execute(&pool)
            .await?;
        }
    }
    Ok(())
}
