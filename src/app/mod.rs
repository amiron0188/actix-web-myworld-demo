use actix_web::{App, HttpResponse, HttpServer, Responder, middleware::Logger, web};
use std::env;

use crate::res::{JsonErr, Res};
use crate::db::RB;


mod routes;
mod services;
mod middlewares;


async fn index() -> impl Responder {
    Res::ok_data(&"Hello!")
}

pub async fn start() -> std::io::Result<()> {
    let server_url = env::var("SERVER_URL").expect("SERVER_URL is not set!");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set!");
      //ORM
    RB.link(&database_url).await.unwrap();
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            
            .service(web::resource("/").to(index))

            .service(web::scope("")
                .configure(routes::routes)
                .wrap(middlewares::auth::Authentication)
            )
            .default_service(web::to(|| HttpResponse::NotFound().json(JsonErr {
                code: 404,
                message: Some("Not Found!")
            })) )
            
    })
    .bind(&server_url)?
    .run()
    .await
}
