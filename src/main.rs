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
    api::user_account_control::{
        create_chatroom, fetch_known_chatrooms, fetch_login, fetch_session_token,
        fetch_unknown_chatroom, handle_logout_request, register_user,
    },
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    // Establish connection with the database
    let servere_state = establish_state()?;

    // Start up the webserver
    let router = Router::new()
        .route("/api/register", post(register_user))
        .route("/api/login", post(fetch_login))
        .route("/api/session", post(fetch_session_token))
        .route("/api/logout", post(handle_logout_request))
        .route(
            "/api/request_unknown_chatroom",
            post(fetch_unknown_chatroom),
        )
        .route("/api/request_known_chatroom", post(fetch_known_chatrooms))
        .route("/api/chatroom_new", post(create_chatroom))
        .route("/api/chatroom_send_message", post(create_chatroom))
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
