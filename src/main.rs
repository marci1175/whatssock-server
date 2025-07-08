use std::env;

use axum::{Router, routing::post, serve};
use diesel::{
    PgConnection,
    r2d2::{self, ConnectionManager},
};
use dotenvy::dotenv;
use tokio::net::TcpListener;
use whatssock_server::{
    ServerState,
    api::register::{fetch_login, register_user},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Establish connection with the database
    let servere_state = establish_state()?;

    // Start up the webserver
    let router = Router::new()
        .route("/api/login", post(fetch_login))
        .route("/api/register", post(register_user))
        .with_state(servere_state);

    let listener = TcpListener::bind("[::1]:3004").await?;

    serve(listener, router).await?;

    Ok(())
}

/// Establishes connection with the PostgreSQL database.
pub fn establish_state() -> anyhow::Result<ServerState> {
    // Read the database url from the .env
    dotenv().ok();

    // Fetch the DATABASE URL
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Establish connection with the database
    let pg_pool: r2d2::Pool<ConnectionManager<PgConnection>> =
        r2d2::Builder::new().build(ConnectionManager::new(database_url))?;

    Ok(ServerState { pg_pool })
}
