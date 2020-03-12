mod stat;
mod settings;
mod premium;
mod android_publisher_api;

use actix_web::{web, App, HttpServer};
use crate::settings::Settings;
use actix::System;
use std::path::Path;
use yup_oauth2 as oauth;
use std::io;
use yup_oauth2::authenticator::{Authenticator};
use hyper_rustls::HttpsConnector;
use hyper::client::HttpConnector;
use yup_oauth2::ServiceAccountAuthenticator;
use std::sync::Arc;

type AppAuthenticator = Authenticator<HttpsConnector<HttpConnector>>;

pub struct AppData {
    pub google_authenticator: AppAuthenticator
}

async fn create_google_authenticator() -> io::Result<AppAuthenticator> {
    let settings = &Settings::get().google;
    let secret = oauth::read_service_account_key(Path::new(settings.key_path.as_str()))
        .await?;
    ServiceAccountAuthenticator::builder(secret)
        .persist_tokens_to_disk(settings.token_cache.as_str())
        .build()
        .await
}

fn main() {
    env_logger::init();
    log::info!("Starting Toolbox Online Services Server");

    let mut sys = System::new("toolbox");
    Settings::get();

    let google_authenticator = sys.block_on(create_google_authenticator()).unwrap();
    let app_data = Arc::new(AppData { google_authenticator: google_authenticator });

    HttpServer::new(move || {
        App::new()
            .data(Arc::clone(&app_data))
            .service(web::scope("/api/v1/stat").configure(stat::configure_stat))
            .service(web::scope("/api/v1/premium").configure(premium::configure_premium))
    })
        .bind("127.0.0.1:8088").expect("Cannot bind")
        .run();
    let _ = sys.run();
}