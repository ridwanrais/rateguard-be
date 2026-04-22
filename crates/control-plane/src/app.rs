use actix_web::{web, App, HttpServer};
use sqlx::{Pool, Postgres};


pub struct AppState {
    pub db: Pool<Postgres>,
}

pub async fn run_server(pool: Pool<Postgres>, port: u16) -> std::io::Result<()> {
    let state = web::Data::new(AppState { db: pool });

    println!("Control Plane listening on 0.0.0.0:{}", port);
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .configure(crate::routes::configure)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
