use actix_web::{get, HttpResponse, Responder};
use serde_json::json;

#[utoipa::path(
    get,
    path = "/stats",
    responses(
        (status = 200, description = "Get global stats for rate limiting")
    )
)]
#[get("/stats")]
pub async fn get() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "total_requests": 1000,
        "total_limited": 42
    }))
}
