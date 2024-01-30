use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use super::types::*;
use crate::auth::types::Claims;
use crate::state::MyState;

#[cfg(debug_assertions)]
pub async fn push(
    State(state): State<MyState>,
    Json(data): Json<ProgramNew>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    match sqlx::query_as::<_, Program>(
        "INSERT INTO programs (program_name, doctype, url) \
            VALUES ($1, $2, $3) \
            RETURNING id, program_name, doctype, url",
    )
    .bind(&data.program_name)
    .bind(&data.doctype)
    .bind(&data.url)
    .fetch_one(&state.pool)
    .await
    {
        Ok(program) => Ok((StatusCode::CREATED, Json(program))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

pub async fn enqueue(
    State(state): State<MyState>,
    Json(data): Json<QueueNew>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    match sqlx::query_as::<_, QueueNew>(
        "INSERT INTO queue (program_name, doctype, url, request_type) \
            VALUES ($1, $2, $3, $4) \
            RETURNING id, program_name, doctype, url, request_type",
    )
    .bind(&data.program_name)
    .bind(&data.doctype)
    .bind(&data.url)
    .bind(&data.request_type)
    .fetch_one(&state.pool)
    .await
    {
        Ok(program) => Ok((StatusCode::CREATED, Json(program))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

pub async fn auth_push(
    State(state): State<MyState>,
    data: Claims,
) -> Result<impl IntoResponse, impl IntoResponse> {
    println!("in test");
    match sqlx::query_as::<_, Program>(
        "INSERT INTO programs (program_name, doctype, url) \
            VALUES ($1, $2, $3) \
            RETURNING id, program_name, doctype, url",
    )
    .bind(&data.payload.program_name)
    .bind(&data.payload.doctype)
    .bind(&data.payload.url)
    .fetch_one(&state.pool)
    .await
    {
        Ok(_) => (),
        Err(e) => return Err((StatusCode::BAD_REQUEST, e.to_string())),
    };

    match sqlx::query_as::<_, Program>("DELETE FROM queue WHERE id=$1")
        .bind(data.payload.id)
        .fetch_one(&state.pool)
        .await
    {
        Ok(program) => Ok(Json(program)),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}
