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
    if data.payload.request_type == "create" {
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
            Ok(program) => match sqlx::query("DELETE FROM queue WHERE id=$1")
                .bind(data.payload.id)
                .execute(&state.pool)
                .await
            {
                Ok(_) => Ok(Json(data.payload)),
                Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
            },
            Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
        }
    } else if data.payload.request_type == "update" {
        match sqlx::query(
            "UPDATE programs \
            SET doctype = $1, url = $2 \
            WHERE program_name = $3", // TODO needs updating when move to multi-entry
        )
        .bind(&data.payload.doctype)
        .bind(&data.payload.url)
        .bind(&data.payload.program_name)
        .execute(&state.pool)
        .await
        {
            Ok(_) => match sqlx::query("DELETE FROM queue WHERE id=$1")
                .bind(data.payload.id)
                .execute(&state.pool)
                .await
            {
                Ok(_) => Ok(Json(data.payload)),
                Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
            },
            Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
        }
    } else {
        Err((StatusCode::BAD_REQUEST, "haha".to_string()))
    }
}
