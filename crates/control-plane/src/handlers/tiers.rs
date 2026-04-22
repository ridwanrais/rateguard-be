use actix_web::{get, post, web, HttpResponse, Responder};
use rateguard_core::models::tier::Tier;
use crate::app::AppState;
use crate::services::tier_service;

#[post("/tiers")]
pub async fn create(state: web::Data<AppState>, payload: web::Json<Tier>) -> impl Responder {
    match tier_service::create_tier(&state.db, payload.into_inner()).await {
        Ok(_) => HttpResponse::Created().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/tiers")]
pub async fn list(state: web::Data<AppState>) -> impl Responder {
    match tier_service::get_all_tiers(&state.db).await {
        Ok(tiers) => HttpResponse::Ok().json(tiers),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
