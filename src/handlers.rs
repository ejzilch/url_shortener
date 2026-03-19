use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::Redirect;
use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};

use crate::error::AppError;
use crate::store::Db;

#[derive(Deserialize)]
pub struct ShortenRequest {
    url: String,
}
#[derive(Serialize)]
pub struct ShortenResponse {
    short_url: String,
}

pub async fn shorten_url(
    State(db): State<Db>,
    Json(payload): Json<ShortenRequest>,
) -> Result<(StatusCode, Json<ShortenResponse>), AppError> {
    let code = "abc123".to_string();

    let mut db_inner = db
        .write()
        .map_err(|_| anyhow::anyhow!("無法取得 db write lock"))?;

    db_inner.insert(code.clone(), payload.url);

    Ok((
        StatusCode::CREATED,
        Json(ShortenResponse {
            short_url: format!("http://localhost:3000/{}", code),
        }),
    ))
}

pub async fn redirect_url(
    Path(code): Path<String>,
    State(db): State<Db>,
) -> Result<Redirect, StatusCode> {
    let db_inner = db.read().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(url) = db_inner.get(&code) {
        Ok(Redirect::temporary(url))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
