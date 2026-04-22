use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse,
};
use std::pin::Pin;
use std::future::{ready, Ready};
use std::rc::Rc;
use crate::AppState;

pub struct RateLimit;

impl<S, B> Transform<S, ServiceRequest> for RateLimit
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<actix_web::body::EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = RateLimitMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RateLimitMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct RateLimitMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for RateLimitMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<actix_web::body::EitherBody<B>>;
    type Error = Error;
    type Future = Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let srv = self.service.clone();

        Box::pin(async move {
            let state = req.app_data::<actix_web::web::Data<AppState>>().unwrap();
            
            let api_key_header = req.headers().get("x-api-key").and_then(|h| h.to_str().ok());
            
            if api_key_header.is_none() {
                let res = HttpResponse::Unauthorized().finish().map_into_right_body();
                return Ok(req.into_response(res));
            }
            
            let api_key_val = api_key_header.unwrap().to_string();

            let (tier_name, limit, window) = if let Some(key_obj) = state.cache.get_api_key(&api_key_val) {
                if let Some(tier_obj) = state.cache.get_tier(&key_obj.tier) {
                    (tier_obj.name, tier_obj.limit, tier_obj.window_seconds)
                } else {
                    let res = HttpResponse::Unauthorized().finish().map_into_right_body();
                    return Ok(req.into_response(res));
                }
            } else {
                let res = HttpResponse::Unauthorized().finish().map_into_right_body();
                return Ok(req.into_response(res));
            };

            state.stats.record_request();

            match state.limiter.check_limit(&api_key_val, limit, window).await {
                Ok((count, reset_time)) => {
                    let remaining = (limit - count).max(0);
                    let now = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs();

                    let append_headers = move |mut res: ServiceResponse<actix_web::body::EitherBody<B>>| {
                        let headers = res.headers_mut();
                        headers.insert(
                            actix_web::http::header::HeaderName::from_static("x-ratelimit-limit"),
                            actix_web::http::header::HeaderValue::from_str(&limit.to_string()).unwrap(),
                        );
                        headers.insert(
                            actix_web::http::header::HeaderName::from_static("x-ratelimit-remaining"),
                            actix_web::http::header::HeaderValue::from_str(&remaining.to_string()).unwrap(),
                        );
                        headers.insert(
                            actix_web::http::header::HeaderName::from_static("x-ratelimit-reset"),
                            actix_web::http::header::HeaderValue::from_str(&reset_time.to_string()).unwrap(),
                        );
                        headers.insert(
                            actix_web::http::header::HeaderName::from_static("x-ratelimit-tier"),
                            actix_web::http::header::HeaderValue::from_str(&tier_name).unwrap(),
                        );
                        res
                    };

                    if count > limit {
                        state.stats.record_limited();
                        let body = serde_json::json!({
                            "error": "rate_limited",
                            "reason": "tier limit exceeded",
                            "retry_in": format!("{}s", reset_time - now)
                        });
                        let res = req.into_response(
                            HttpResponse::TooManyRequests().json(body).map_into_right_body()
                        );
                        return Ok(append_headers(res));
                    }

                    let res = srv.call(req).await?;
                    Ok(append_headers(res.map_into_left_body()))
                }
                Err(_) => {
                    let res = HttpResponse::InternalServerError().finish().map_into_right_body();
                    Ok(req.into_response(res))
                }
            }
        })
    }
}
