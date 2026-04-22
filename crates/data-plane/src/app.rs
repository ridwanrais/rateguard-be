use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use crate::middleware::rate_limit::RateLimit;
use crate::AppState;

pub async fn run_server(state: AppState) -> std::io::Result<()> {
    println!("Data Plane listening on 0.0.0.0:3000");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .wrap(RateLimit)
            .default_service(web::route().to(proxy_handler))
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}

async fn proxy_handler() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({"message": "Proxy response OK"}))
}
