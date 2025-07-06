use diesel::{r2d2::ConnectionManager, PgConnection};

pub mod models;
pub mod schema;
pub mod api;

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
    success: bool,
}

impl LoginResponse {
    pub fn new(success: bool) -> Self {
        Self { success }
    }
}

pub struct RegisterRequest {
    username: String,
    password: String,
    email: String,
    gender: bool,
}