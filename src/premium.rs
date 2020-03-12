use actix_web::{web, HttpResponse};
use crate::AppData;
use serde::Serialize;
use std::sync::Arc;
use crate::android_publisher_api::AndroidPublisherApi;
use chrono::Utc;
use crate::settings::Settings;

#[derive(derive_more::From)]
pub enum PremiumError {
    OauthError(yup_oauth2::Error),
    ReqwestError(reqwest::Error),
}

#[derive(Serialize)]
struct PremiumOnlineResponse {
    status: &'static str,
    entitlement: Option<String>
}

impl PremiumOnlineResponse {

    fn from_error(error: &'static str) -> HttpResponse {
        return HttpResponse::Forbidden().json(PremiumOnlineResponse {
            status: error, entitlement: None
        })
    }

}

async fn verify_subscription_impl(data: web::Data<Arc<AppData>>, sub_id: &str, user_token: &str) -> Result<HttpResponse, PremiumError> {
    if !Settings::get().premium.valid_subscription_ids.contains(sub_id) {
        return Ok (PremiumOnlineResponse::from_error("invalid_sub_id"))
    }

    let publisher = AndroidPublisherApi::from_authenticator(&data.google_authenticator).await?;
    let sub_info = publisher.get_subscription_info("io.mrarm.mctoolbox", sub_id, user_token).await?;
    let now = Utc::now().timestamp_millis() as u64;
    if sub_info.expiry_time_millis <= now {
        return Ok (PremiumOnlineResponse::from_error("expired"))
    }

    Ok (HttpResponse::Ok().json(PremiumOnlineResponse {
        status: "ok",
        entitlement: Some(String::from("TODO"))
    }))
}
async fn verify_subscription(data: web::Data<Arc<AppData>>, params: web::Path<(String, String)>) -> HttpResponse {
    match verify_subscription_impl(data, params.0.as_str(), params.1.as_str()).await {
        Ok (r) => r,
        Err (_) => HttpResponse::InternalServerError().body("")
    }
}

pub fn configure_premium(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/subscription/{sub_id}/{token}", web::get().to(verify_subscription));
}