#[macro_use]
extern crate rbatis;

mod api;
mod app_state;
mod config;
mod db;
mod errors;
mod model;
mod utils;

use crate::config::AppConfig;
use http::StatusCode;

use actix_web::{error, web, App, HttpResponse, HttpServer};
use errors::Response;
use lazy_static::lazy_static;
use log::{info, LevelFilter};
use std::env;

const DEFAULT_SETTING_PATH: &str = "./settings.toml";
lazy_static! {
    pub static ref SETTINGS: AppConfig = config::AppConfig::init(match env::var("CONFIG_PATH") {
        Ok(path) => {
            if path.is_empty() {
                DEFAULT_SETTING_PATH.to_string()
            } else {
                path
            }
        }
        Err(_) => DEFAULT_SETTING_PATH.to_string(),
    })
    .unwrap();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    lazy_static::initialize(&SETTINGS);

    fast_log::init(fast_log::Config::new().level(LevelFilter::Info).console()).unwrap();

    info!("commit hash: {}", env!("GIT_HASH"));
    info!(
        "Http Server starting on {}:{}",
        SETTINGS.server.address, SETTINGS.server.port
    );

    let db = db::Instance::init().await;
    let state = app_state::AppState::new(db.engine, SETTINGS.clone());
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .configure(api::services)
            .app_data(web::FormConfig::default().error_handler(|err, _req| {
                error::InternalError::from_response(
                    "",
                    HttpResponse::BadRequest()
                        .content_type("application/json")
                        .body(format!(r#"{{"message":"{}","code":-1001}}"#, err)),
                )
                .into()
            }))
    })
    .bind((SETTINGS.server.address.clone(), SETTINGS.server.port))?
    .run()
    .await
}
