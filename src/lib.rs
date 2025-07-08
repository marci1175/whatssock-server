use diesel::{PgConnection, r2d2::ConnectionManager};

pub mod api;
pub mod models;
pub mod schema;

pub type PgPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Debug, Clone)]
pub struct ServerState {
    pub pg_pool: PgPool,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct LoginResponse {
    username: String,
}

impl LoginResponse {
    pub fn new(username: String) -> Self {
        Self { username }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct RegisterRequest {
    username: String,
    password: String,
    email: String,
}
