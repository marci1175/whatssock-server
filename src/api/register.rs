use axum::{Json, extract::State, http::StatusCode};
use diesel::dsl::count_star;
use diesel::query_dsl::methods::{FilterDsl, SelectDsl};
use diesel::{ExpressionMethods, OptionalExtension, RunQueryDsl, SelectableHelper};
use log::error;

use crate::api::register::users::dsl::users;
use crate::models::{AccountRegister, UserAccount};
use crate::schema::users::{passw, username};
use crate::{LoginRequest, LoginResponse, RegisterRequest, ServerState, schema::*};

#[axum::debug_handler]
pub async fn fetch_login(
    State(state): State<ServerState>,
    Json(information): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    let mut pg_connection = state.pg_pool.get().map_err(|err| {
        error!(
            "An error occured while fetching login information from db: {}",
            err.to_string()
        );

        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let user_query = users
        .filter(username.eq(information.username.clone()))
        .filter(passw.eq(information.password))
        .select(UserAccount::as_select())
        .get_result(&mut pg_connection)
        .map_err(|err| {
            error!(
                "An error occured while searching for the user's account: {}",
                err.to_string()
            );

            StatusCode::NOT_FOUND
        })?;

    dbg!(user_query);

    Ok(Json(LoginResponse::new(information.username)))
}

pub async fn register_user(
    State(state): State<ServerState>,
    Json(information): Json<RegisterRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    let mut pg_connection = state.pg_pool.get().map_err(|err| {
        error!(
            "An error occured while fetching login information from db: {}",
            err.to_string()
        );

        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    if let Some(_count) = users
        .filter(username.eq(information.username.clone()))
        .select(count_star())
        .first::<i64>(&mut pg_connection)
        .optional()
        .map_err(|err| {
            error!(
                "An error occured while fetching login information from db: {}",
                err.to_string()
            );
            StatusCode::REQUEST_TIMEOUT
        })?
    {
        return Err(StatusCode::FOUND);
    }

    diesel::insert_into(users)
        .values(&AccountRegister {
            username: information.username.clone(),
            passw: information.password,
            email: information.email,
            gender: information.gender,
        })
        .load::<UserAccount>(&mut pg_connection).map_err(|err| {
            error!(
                "An error occured while fetching login information from db: {}",
                err.to_string()
            );
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(LoginResponse {
        username: information.username,
    }))
}
