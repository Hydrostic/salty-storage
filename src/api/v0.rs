use actix_web::{web, Scope};

mod auth;
mod user;

pub fn service() -> Scope {
    web::scope("/v0")
        .service(user::service())
        .service(auth::service())
}
