use actix_web::web;
use crate::handlers::{api_keys, internal, stats, tiers};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(tiers::create)
       .service(tiers::list)
       .service(api_keys::create)
       .service(api_keys::list)
       .service(stats::get)
       .service(internal::config);
}
