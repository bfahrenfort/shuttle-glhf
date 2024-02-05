pub mod types;

use axum::{extract::State, response::IntoResponse, Json};
use jsonwebtoken::{encode, Header};
use once_cell::sync::Lazy;
use rand::rngs::OsRng;
use rand::RngCore;
use std::time::SystemTime;

use crate::db::types::{Admin, Queue};
use crate::state::MyState;
use types::*;

pub static KEYS: Lazy<Keys> = Lazy::new(|| {
    let mut key = [0u8; 64];
    OsRng.fill_bytes(&mut key);
    Keys::new(&key)
});
static QUEUE_FETCH_ONCE: u32 = 5;

//TODO ensure server is not leaking info that could be used to build an attack surface
pub async fn login(
    State(state): State<MyState>,
    Json(payload): Json<AuthPayload>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    if payload.client_id.is_empty() || payload.client_secret.is_empty() {
        return Err(AuthError::MissingCredentials);
    }

    let tokens: Vec<String> = sqlx::query_as::<_, Admin>("SELECT * FROM admins")
        .fetch_all(&state.pool)
        .await
        .map_err(|_| AuthError::WrongCredentials)?
        .into_iter()
        .map(|e| e.token)
        .collect();
    // TODO fix, change to use the migration 3 db
    if !tokens.contains(&payload.client_secret)
        && state.secrets.get("ROOT_ADMIN").unwrap() != payload.client_secret
    {
        return Err(AuthError::WrongCredentials);
    }
    // add 5 minutes to current unix epoch time as expiry date/time
    let exp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + 300;
    let exp = usize::try_from(exp).unwrap();

    match payload.queue_id {
        Some(id) => {
            // If they already know what id to use from queue_peek, then just push it
            let pusher = sqlx::query_as::<_, Queue>("SELECT * FROM queue WHERE id=$1")
                .bind(id)
                .fetch_one(&state.pool)
                .await
                .map_err(|_| AuthError::BadQueue)?;

            let record = Claims {
                payload: pusher,
                exp,
            };

            let token = encode(&Header::default(), &record, &KEYS.encoding)
                .map_err(|_| AuthError::TokenCreation)?;

            Ok(Json(AuthBody::new(token)))
        }
        None => {
            let queue_peek = sqlx::query_as::<_, Queue>("SELECT * FROM queue")
                .fetch_all(&state.pool)
                .await
                .map_err(|_| AuthError::BadQueue)?;

            // TODO display web interface to pick queue items?
            let record = Claims {
                payload: queue_peek[0].clone(),
                exp,
            };

            let token = encode(&Header::default(), &record, &KEYS.encoding)
                .map_err(|_| AuthError::TokenCreation)?;

            Ok(Json(AuthBody::new(token)))
        }
    }
}
