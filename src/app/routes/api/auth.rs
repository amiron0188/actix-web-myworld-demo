use actix_web::{web, post, HttpResponse};

use crate::{error::Error, models::login::LoginForm, res::Res, app::services::auth::AuthSerivce };


#[post("/login")]
async fn login(form: web::Json<LoginForm>) -> Result<HttpResponse, Error> {
    let login_info = AuthSerivce.login(&form).await?;
    Ok(Res::ok_data(&login_info))
}