use actix_web::{HttpResponse, web, post, delete, get, put};

use crate::{app::services::user::UserService, error::Error, models::{pageable::Pageable, user::UserForm}, res::Res};

#[post("/register")]
async fn register(form: web::Json<UserForm>) -> Result<HttpResponse, Error> {
    UserService.register(&form).await?;
    Ok(Res::ok())
}

#[put("/update")]
async fn update(form: web::Json<UserForm>) -> Result<HttpResponse, Error> {
    UserService.update(&form).await?;
    Ok(Res::ok())
}

#[delete("/delete")]
async fn delete(form: web::Json<UserForm>) -> Result<HttpResponse, Error> {
    UserService.delete(&form).await?;
    Ok(Res::ok())
}

#[get("/list")]
async fn list(pageable: web::Query<Pageable>) -> Result<HttpResponse, Error> {
    let user_page = UserService.list(&pageable).await?;
    Ok(Res::ok_data(&user_page))
}





