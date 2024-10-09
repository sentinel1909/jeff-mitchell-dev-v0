// src/lib/utilities.rs

// dependencies
use crate::domain::app_data::{BodyContent, FrontMatter};
use crate::errors::AppError;
use gray_matter::engine::YAML;
use gray_matter::Matter;
use pulldown_cmark::{Parser, Options};
use regex::Regex;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

// utility function to grab the front matter from each markdown article
pub fn get_frontmatters() -> Result<Vec<FrontMatter>, AppError> {
    let folder_path = Path::new("./content");
    let mut front_matters = Vec::new();
    for entry in WalkDir::new(folder_path).min_depth(1).into_iter() {
        let entry = entry
            .map_err(|err: walkdir::Error| AppError::DirectoryRead(err.into_io_error().unwrap()))?;
        let path = entry.path();
        if path.is_file() {
            let content = fs::read_to_string(path).map_err(AppError::FileRead)?;
            let matter = Matter::<YAML>::new().parse(&content);
            let front_matter: FrontMatter = matter
                .data
                .as_ref()
                .map(|data| data.deserialize())
                .transpose()
                .map_err(AppError::DeserializeFrontMatter)?
                .unwrap_or_default();
            front_matters.push(front_matter);
        }
    }
    Ok(front_matters)
}

// utility function to grab the body content from each markdown article
pub fn get_bodies() -> Result<Vec<BodyContent>, AppError> {
    let folder_path = Path::new("./content");
    let mut bodies = Vec::new();
    for entry in WalkDir::new(folder_path).min_depth(1).into_iter() {
        let entry = entry
            .map_err(|err: walkdir::Error| AppError::DirectoryRead(err.into_io_error().unwrap()))?;
        let path = entry.path();
        if path.is_file() {
            let content = fs::read_to_string(path).map_err(AppError::FileRead)?;
            let frontmatter_regex =
                Regex::new(r"---\s*\n(?s:.+?)\n---\s*\n").map_err(AppError::Regex)?;
            let body: BodyContent = BodyContent {
                content: frontmatter_regex.replace(&content, "").to_string(),
            };
            bodies.push(body);
        }
    }
    Ok(bodies)
}

// utility function to convert markdown content into HTML
pub fn convert_markdown(markdown_content: String) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    let parser = Parser::new_ext(&markdown_content, options);
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);

    html_output
}
