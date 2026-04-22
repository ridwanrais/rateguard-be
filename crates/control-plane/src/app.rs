use actix_web::{web, App, HttpServer};
use sqlx::{Pool, Postgres};


pub struct AppState {
    pub db: Pool<Postgres>,
}

pub async fn run_server(pool: Pool<Postgres>) -> std::io::Result<()> {
    let state = web::Data::new(AppState { db: pool });

    println!("Control Plane listening on 0.0.0.0:3001");
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .configure(crate::routes::configure)
    })
    .bind(("0.0.0.0", 3001))?
    .run()
    .await
}
