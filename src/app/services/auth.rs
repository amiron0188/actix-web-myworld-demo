use libreauth::pass::HashBuilder;
use validator::Validate;

use crate::{db::user::UserDB, error::Error, models::login::{LoginForm, LoginInfo}, utils::{hasher::{ PWD_SCHEME_VERSION, HASHER }, jwt::JWT}};

pub struct AuthSerivce;
impl AuthSerivce {
    pub async fn login(&self, lf: &LoginForm) -> Result<Option<LoginInfo>, Error> {
        Validate::validate(&lf.clone())?;
    
        let user = UserDB.find_by_username(&lf.username.as_ref().unwrap_or(&"".to_string())).await?;
        if user.is_none() {
            return Err(Error::Unauthorized(json!("'lf.username.as_ref()' not exist!")));
        }
    
        let mut user = user.unwrap();
        let checker = HashBuilder::from_phc(&user.password.clone().unwrap())?;
        if checker.is_valid(lf.password.as_ref()) {
            if checker.needs_update(Some(PWD_SCHEME_VERSION)) {
                let new_password = HASHER.hash(&lf.password)?;
                user.password = Some(new_password);
                UserDB.update(&user).await?;
            } 
            let token = JWT.generate_jwt(&Some(user.clone()));
            let login_info = LoginInfo {
                username: user.username,
                token: token
            };
    
            Ok(Some(login_info))
        } else {
            return Err(Error::Unauthorized(json!("Wrong password!")));
        }
    }
}
