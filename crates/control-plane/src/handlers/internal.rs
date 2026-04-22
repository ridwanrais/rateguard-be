use actix_web::{get, web, HttpResponse, Responder};
use serde::Serialize;
use rateguard_core::models::api_key::ApiKey;
use rateguard_core::models::tier::Tier;
use crate::app::AppState;
use crate::services::{api_key_service, tier_service};

#[derive(Serialize)]
pub struct InternalConfigResponse {
    pub version: u64,
    pub tiers: Vec<Tier>,
    pub api_keys: Vec<ApiKey>,
}

#[get("/internal/config")]
pub async fn config(state: web::Data<AppState>) -> impl Responder {
    let tiers = tier_service::get_all_tiers(&state.db).await.unwrap_or_default();
    let api_keys = api_key_service::get_all_api_keys(&state.db).await.unwrap_or_default();

    HttpResponse::Ok().json(InternalConfigResponse {
        version: 1,
        tiers,
        api_keys,
    })
}
