use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use rateguard_core::models::tier::{Tier, RouteLimit};
use rateguard_core::models::api_key::ApiKey;
use crate::handlers::{api_keys, internal, stats, tiers};

#[derive(OpenApi)]
#[openapi(
    paths(
        tiers::create,
        tiers::list,
        api_keys::create,
        api_keys::list,
        stats::get,
        internal::config
    ),
    components(
        schemas(Tier, RouteLimit, ApiKey, internal::InternalConfigResponse, api_keys::CreateApiKeyRequest)
    ),
    tags(
        (name = "RateGuard", description = "Control Plane Management API")
    )
)]
struct ApiDoc;

pub fn configure(cfg: &mut web::ServiceConfig) {
    let openapi = ApiDoc::openapi();

    cfg.service(
        SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone())
    )
    .service(tiers::create)
    .service(tiers::list)
    .service(api_keys::create)
    .service(api_keys::list)
    .service(stats::get)
    .service(internal::config);
}
