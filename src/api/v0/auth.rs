use crate::app_state::AppState;
use crate::errors::{Error, Response};
use actix_web::{post, web, HttpResponse, Responder, Scope};
use log::error;
use serde::Deserialize;
use validator::Validate;

pub fn service() -> Scope {
    web::scope("/auth").service(signin)
}

#[derive(Deserialize, Validate)]
struct Signin {
    #[validate(length(min = 1, max = 500))]
    name: String,
    #[validate(length(min = 1, max = 50))]
    passwd: String,
}
const INCORRECT_CRDENTIAL: i32 = -1;
#[post("/signin")]
async fn signin(
    state: web::Data<AppState>,
    form: web::Form<Signin>,
) -> Result<impl Responder, Error> {
    match form.validate() {
        Ok(_) => (),
        Err(e) => {
            error!("{}", e);
            return Err(Error::BadClientData);
        }
    };
    let mut new_user = crate::model::user::new();
    new_user.name = form.name.to_owned();
    let is_correct = new_user
        .compare_password(
            &state.db,
            crate::model::user::IdentityType::Name,
            &form.passwd,
        )
        .await;
    if match is_correct {
        Err(e) => {
            return Err(Error::InternalError { e });
        }
        Ok(t) => t,
    } {
        Ok(HttpResponse::Ok().json(new_user))
    } else {
        Ok(HttpResponse::Ok().json(Response {
            code: INCORRECT_CRDENTIAL,
            message: "Incorrect credential.".into(),
        }))
    }
}
