use super::error::Result;
use crate::db::Db;
use axum::extract::State;
use axum::routing::{get, post};
use axum::Router;
use axum::{response::IntoResponse, Json};
use serde::Deserialize;

pub fn router() -> Router<Db> {
    Router::new()
        .route("/health", get(health))
        .route("/register", post(register))
}

async fn health() -> impl IntoResponse {}

#[derive(Debug, Deserialize)]
struct Register {
    pub address: alloy_primitives::Address,
}

async fn register(
    State(db): State<Db>,
    Json(register): Json<Register>,
) -> Result<impl IntoResponse> {
    db.register(register.address.into()).await?;

    Ok(())
}
