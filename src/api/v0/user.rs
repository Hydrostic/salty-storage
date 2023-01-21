use crate::app_state::AppState;
use crate::errors::Error;
use actix_web::{post, web, HttpResponse, Responder, Scope};
use log::error;

pub fn service() -> Scope {
    web::scope("/user").service(create_user)
}

#[post("/create")]
async fn create_user(state: web::Data<AppState>) -> Result<impl Responder, Error> {
    let mut new_user = crate::model::user::new();
    new_user.name = "test".to_string();
    new_user.set_password("114514");
    new_user.email = Some("test@example.com".to_string());
    if let Err(e) = new_user.create(&state.db).await {
        error!("{}", e);
        return Err(Error::InternalError { e });
    }
    Ok(HttpResponse::Ok().body("get1"))
}
