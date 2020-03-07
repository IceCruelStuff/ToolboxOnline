mod stat;
mod settings;

use actix_web::{web, App, HttpServer};
use crate::settings::Settings;
use actix::System;

fn main() {
    let sys = System::new("toolbox");
    Settings::get();

    HttpServer::new(|| {
        App::new()
            .service(web::scope("/api/v1/stat").configure(stat::configure_stat))
    })
        .bind("127.0.0.1:8088").expect("Cannot bind")
        .run();
    let _ = sys.run();
}