use std::env;

use axum::{routing::post, serve, Router};
use diesel::{r2d2::{self, ConnectionManager}, PgConnection};
use dotenvy::dotenv;
use diesel::prelude::*;
use tokio::net::TcpListener;
use whatssock_server::{api::register::{fetch_login, register_user}, ServerState};

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
    let pg_pool: r2d2::Pool<ConnectionManager<PgConnection>> = r2d2::Builder::new()
        .build(ConnectionManager::new(database_url))?;

    Ok(ServerState {
        pg_pool
    })
}

// /// example code right here
// pub fn create_post(conn: &mut PgConnection, title: &str, body: &str) -> Post {
//     use whatssock_server::schema::posts;

//     let new_post = NewPost { title, body };

//     diesel::insert_into(posts::table)
//         .values(&new_post)
//         .returning(Post::as_returning())
//         .get_result(conn)
//         .expect("Error saving new post")
// }