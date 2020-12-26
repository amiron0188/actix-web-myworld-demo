mod ignore;

use std::{pin::Pin, task::{Context, Poll}};
use actix_service::{Service, Transform};
use actix_web::{Error, dev::{ ServiceRequest, ServiceResponse}, http::HeaderValue, HttpResponse};
use futures::{future::{ok, Ready}, Future};
use ignore::IGNORE_AUTH_ROUTES;

use crate::{utils::jwt, res::JsonErr};

pub struct Authentication;

impl <S, B> Transform<S> for Authentication
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddleware { service })
    }
}

pub struct AuthMiddleware<S> {
    service: S,
}

impl<S, B> Service for AuthMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }


    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let mut is_ignore_authorized = false;

        for i in IGNORE_AUTH_ROUTES.iter() {
            let path = String::from(req.method().to_string() + " " + req.path());
            if path.find(i) == Some(0) {
                is_ignore_authorized = true;
                break;
            }
        }

        let unauthorized_value = HeaderValue::from_str(&"").unwrap();
        let token = req.headers()
                                .get("Authorization")
                                .unwrap_or(&unauthorized_value)
                                .to_str()
                                .ok()
                                .unwrap();
        let jwt_authorized = jwt::JWT.decode_jwt(&token).is_ok();

        if jwt_authorized == false && is_ignore_authorized != true {
            return Box::pin(async move {
                Ok(req.into_response(HttpResponse::Unauthorized().json(JsonErr {
                    code: 401,
                    message: Some("Unauthorized!")
                }).into_body()))
            });
        }

        
        let fut = self.service.call(req);
        // ;
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
            
        })
    }
}
