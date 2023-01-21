use actix_web::{error, http::StatusCode, HttpRequest, HttpResponse};

use anyhow;
use derive_more::{Display, Error};
use log::error;
use serde::Serialize;

#[derive(Debug, Display, Error)]
pub enum Error {
    #[display(fmt = "internal error")]
    InternalError { e: anyhow::Error },

    #[display(fmt = "bad request")]
    BadClientData,

    #[display(fmt = "timeout")]
    Timeout,
}

#[derive(Serialize)]
pub(crate) struct Response {
    pub(crate) code: i32,
    pub(crate) message: String,
}

impl error::ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match *self {
            Error::InternalError { e: _ } => StatusCode::INTERNAL_SERVER_ERROR,
            Error::BadClientData => StatusCode::BAD_REQUEST,
            Error::Timeout => StatusCode::GATEWAY_TIMEOUT,
        }
    }

    fn error_response(&self) -> HttpResponse {
        if let Error::InternalError { e } = self {
            error!("{:?}", e);
        }
        HttpResponse::build(self.status_code()).json(Response {
            code: -1001,
            message: self.to_string(),
        })
    }
}
