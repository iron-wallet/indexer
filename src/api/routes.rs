use super::error::Result;
use crate::db::{Db, PgBackend};
use actix_web::{
    get, post,
    web::{self, Json},
    HttpResponse, Responder,
};
use serde::Deserialize;

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok()
}

#[derive(Debug, Deserialize)]
struct Register {
    address: alloy_primitives::Address,
}

#[post("/register")]
async fn register(
    db: web::Data<PgBackend>,
    Json(register): Json<Register>,
) -> Result<impl Responder> {
    db.register(register.address.into()).await?;

    Ok(HttpResponse::Ok())
}
