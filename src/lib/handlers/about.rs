// src/lib/handlers/about.rs

// dependencies
use crate::templates::AboutTemplate;
use axum_macros::debug_handler;

// about handler function
#[debug_handler]
pub async fn about() -> AboutTemplate {
    AboutTemplate {}
}
