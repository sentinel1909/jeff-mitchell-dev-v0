// src/lib/handlers/music.rs

// dependencies
use crate::templates::MusicTemplate;

// index handler function
pub async fn music() -> MusicTemplate {
    let url = "https://api.rivet-head.app/diary".to_string();
    MusicTemplate { api_url: url }
}
