use actix_web::{HttpResponse, ResponseError};
use rbatis::core::Error as RBError;
use libreauth::pass::ErrorCode as PassErrorCode;
use serde_json:: Value as JsonValue;
use jwt::errors::{Error as JwtError, ErrorKind as JwtErrorKind};
use validator::ValidationErrors;

use crate::res::JsonErr;
#[derive(Debug, Fail)]
pub enum Error {
    //401
    #[fail(display = "Unauthoried: {}", _0)]
    Unauthorized(JsonValue),

    // //403
    // #[fail(display = "Forbidden: {}", _0)]
    // Forbidden(JsonValue),

    // //404
    // #[fail(display = "Not Found: {}", _0)]
    // NotFound(JsonValue),

    // 422
    #[fail(display = "Unprocessable Entity: {}", _0)]
    UnprocessableEntity(JsonValue),

    //500
    #[fail(display = "Internal Server Error")]
    InternalServerError

}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match self {
            Error::Unauthorized(e) => HttpResponse::Unauthorized().json(JsonErr {
                code: 401,
                message: Some(e.clone())
            }),
            // Error::Forbidden(e) => HttpResponse::Forbidden().json(JsonErr {
            //     code: 403,
            //     message: Some(e.clone())
            // }),
            // Error::NotFound(e) => HttpResponse::NotFound().json(JsonErr {
            //     code: 404,
            //     message: Some(e.clone())
            // }),
            Error::UnprocessableEntity(e) => HttpResponse::UnprocessableEntity().json(JsonErr {
                code: 422,
                message: Some(e.clone())
            }),
            Error::InternalServerError => HttpResponse::InternalServerError().json(JsonErr {
                code: 500,
                message: Some(json!("Internal Server Error!".to_string()))
            })
        }
    }
}

impl From<ValidationErrors> for Error {
    fn from(e: ValidationErrors) -> Self {
        let mut err = JsonValue::default();

        for (_, e) in e.field_errors().iter() {
            let e: Vec<JsonValue> = e.iter().map(|e| {
                json!(e.message)
            }).collect();
            err = json!(e);
        }

        Error::UnprocessableEntity(json!(err))
    }
}

impl From<RBError> for Error {
    fn from(e: RBError) -> Self {
        Error::UnprocessableEntity(json!(e.to_string()))
    }
}

impl From<PassErrorCode> for Error {
    fn from(_e: PassErrorCode) -> Self {
        Error:: UnprocessableEntity(json!("Password code error!".to_string()))
    }
}

impl From<JwtError> for Error {
    fn from(e: JwtError) -> Self {
        match e.kind() {
            JwtErrorKind::InvalidToken => Error::Unauthorized(json!("Token is invalid!".to_string())),
            JwtErrorKind::InvalidIssuer => Error::Unauthorized(json!("Issuer is invalid!".to_string())),
            _ => Error::Unauthorized(json!("An issue was found with the token!".to_string()))
        }
    }
}