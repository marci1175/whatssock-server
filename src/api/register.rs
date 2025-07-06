use axum::{extract::State, http::StatusCode, Json};

use crate::{LoginRequest, LoginResponse, ServerState};

#[axum::debug_handler]
pub async fn fetch_login(State(state): State<ServerState>, Json(information): Json<LoginRequest>) -> Result<Json<LoginResponse>, StatusCode> {
    dbg!(information);

    Ok(Json(LoginResponse::new(false)))
}

pub async fn register_user(Json(information): Json<LoginRequest>) -> String {
    dbg!(information);

    "fasz".to_string()
}