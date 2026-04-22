use actix_web::{get, post, web, HttpResponse, Responder};
use serde::Deserialize;
use uuid::Uuid;
use crate::app::AppState;
use crate::services::api_key_service;

#[derive(Deserialize)]
pub struct CreateApiKeyRequest {
    pub user_id: Uuid,
    pub tier: String,
}

#[post("/api-keys")]
pub async fn create(state: web::Data<AppState>, payload: web::Json<CreateApiKeyRequest>) -> impl Responder {
    match api_key_service::generate_api_key(&state.db, payload.user_id, payload.tier.clone()).await {
        Ok(api_key) => HttpResponse::Created().json(api_key),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/api-keys")]
pub async fn list(state: web::Data<AppState>) -> impl Responder {
    match api_key_service::get_all_api_keys(&state.db).await {
        Ok(keys) => HttpResponse::Ok().json(keys),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
