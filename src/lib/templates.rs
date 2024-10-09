// src/lib/templates.rs

// dependencies
use crate::domain::Article;
use askama_axum::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {}

#[derive(Template)]
#[template(path = "about.html")]
pub struct AboutTemplate {}

#[derive(Template)]
#[template(path = "articles.html")]
pub struct ArticlesTemplate {
    pub articles: Vec<Article>,
}

#[derive(Template)]
#[template(path = "article.html")]
pub struct ArticleTemplate {
    pub page_title: String,
    pub article: String,
}

#[derive(Template)]
#[template(path = "music.html")]
pub struct MusicTemplate {
    pub api_url: String,
}

#[derive(Template)]
#[template(path = "games.html")]
pub struct GamesTemplate {}

#[derive(Template)]
#[template(path = "photography.html")]
pub struct PhotographyTemplate {}

#[derive(Template)]
#[template(path = "projects.html")]
pub struct ProjectsTemplate {}

#[derive(Template)]
#[template(path = "404.html")]
pub struct NotFoundTemplate {
    pub error: String,
}
