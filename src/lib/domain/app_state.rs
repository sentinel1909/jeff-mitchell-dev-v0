// src/lib/domain/app_state.rs

// dependencies
use sqlx::PgPool;
use std::sync::{Arc, RwLock};

// custom type for a shared state
pub type SharedState = Arc<RwLock<AppState>>;

// struct to represent the app state; has one field for the database pool
#[derive(Debug)]
pub struct AppState {
    pub pool: PgPool,
}

impl AppState {
    pub fn initialize(pool: PgPool) -> Self {
        Self { pool }
    }

    pub fn initialize_shared_state(self) -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(self))
    }
}
