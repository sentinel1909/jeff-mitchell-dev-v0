// src/bin/httpd.rs

// dependencies
use jeff_mitchell_dev::domain::{AppState, JeffMitchellDevService};
use jeff_mitchell_dev::startup::{
    build_content, build_router, database_migrations, initialize_tracing,
};
use shuttle_runtime::{CustomError, Error};
use sqlx::PgPool;

// the main function, builds and returns an AxumService
#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] pool: PgPool,
) -> Result<JeffMitchellDevService, Error> {
    // initialize tracing
    initialize_tracing();

    // initialize the application state to hold the database pool
    let app_state = AppState::initialize(pool);

    // initialize the shared state and make it safe to share across threads
    let shared_state = AppState::initialize_shared_state(app_state);

    // call database_migrations(), pass in a reference to shared state to get the database pool
    database_migrations(&shared_state).await.map_err(|err| {
        let error_msg = format!("Could not complete the database migrations: {}", err);
        CustomError::new(err).context(error_msg)
    })?;

    // call build_content(), load the article content into the database, pass in a reference to shared state to get the database pool
    build_content(&shared_state).await.map_err(|err| {
        let error_msg = format!("Could not build the content for the articles page: {}", err);
        CustomError::new(err).context(error_msg)
    })?;

    // call build_router(), passing in the database pool, to configure and build an Axum router (with tracing, CORS, handlers, and state)
    let app_router = build_router(shared_state);

    // convert the router into an AxumService in order to run it
    Ok(JeffMitchellDevService { app_router })
}
