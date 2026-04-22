use actix_web::{get, HttpResponse, Responder};
use serde_json::json;

#[get("/stats")]
pub async fn get() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "total_requests": 1000,
        "total_limited": 42
    }))
}
