// src/lib/handlers/mod.rs

pub mod about;
pub mod article;
pub mod articles;
pub mod articles_category;

pub mod articles_tag;
pub mod games;
pub mod handler_404;
pub mod health_check;
pub mod index;
pub mod music;
pub mod photography;
pub mod projects;

pub use about::*;
pub use article::*;
pub use articles::*;
pub use articles_category::*;
pub use articles_tag::*;
pub use games::*;
pub use handler_404::*;
pub use health_check::*;
pub use index::*;
pub use music::*;
pub use photography::*;
pub use projects::*;
