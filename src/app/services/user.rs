use rbatis::plugin::page::Page;
use validator::Validate;

use crate::{models::{pageable::Pageable, user::{User, UserForm}}};
use crate::error::Error;
use crate::db::user::UserDB;
use crate::utils::hasher::HASHER;

pub struct UserService;

impl UserService {
    pub async fn register(&self, uf: &UserForm) -> Result<(), Error> {
        Validate::validate(&uf.clone())?;
        let username = uf.username.clone().unwrap_or("".to_string());
        let existed_user = UserDB.find_by_username(&username).await?;
        
        if existed_user.is_some() {
            return Err(Error::Unauthorized(json!("username has been existed!")));
        }

        let mut u = User::map_from(&uf);    
        let pass = HASHER.hash(&u.password.clone().unwrap())?;
        u.password = Some(pass);
        UserDB.create(&u).await?;

        Ok(())
    }

    pub async fn update(&self, uf: &UserForm) -> Result<(), Error> {
        Validate::validate(&uf.clone())?;
        let username = uf.username.clone().unwrap();
        let u = UserDB.find_by_username(&username).await?;
        if u.is_none() {
            return Err(Error::InternalServerError);
        }
        
        let mut u = u.unwrap();
        let pass = HASHER.hash(&uf.password.clone())?;
        u.password = Some(pass);
        u.email = uf.email.clone();

        UserDB.update(&u).await?;

        Ok(())
    }

    pub async fn delete(&self, uf: &UserForm) -> Result<(), Error> {
        let u = User::map_from(&uf);
        UserDB.delete(&u).await?;
        Ok(())
    }

    pub async fn list(&self, p: &Pageable) -> Result<Page<User>, Error> {
        let pageable = p.clone();
        let user_page = UserDB.list(&pageable).await?;
        Ok(user_page)
    }
}

