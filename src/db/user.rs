use rbatis::{crud::CRUD, plugin::page::{Page, PageRequest}};
use rbatis::core::Result;
use rbatis::core::Error;
use chrono::NaiveDateTime;
use rbatis::core::value::DateTimeNow;

use crate::{db::RB, models::pageable::Pageable};
use crate::models::user::User;

pub struct UserDB ;
impl UserDB {
    pub async fn create(&self, u: &User) -> Result<u64> {
        let mut user = u.clone();
        let id = rbatis::plugin::snowflake::async_snowflake_id().await.to_string();
        user.id = Some(id);
        user.updated_at = Some(NaiveDateTime::now());
        user.created_at = Some(NaiveDateTime::now());
        user.del = Some(0);

        return Ok(RB.save("", &user).await?.rows_affected);
    }

    pub async fn update(&self, u: &User) -> Result<u64> {
        let mut user = u.clone();
        user.updated_at = Some(NaiveDateTime::now());

        return RB.update_by_id("", &user).await;
    }

    pub async fn find_by_username(&self, username: &str) -> Result<Option<User>> {
        let wrapper = RB.new_wrapper().eq("username", username).check()?;
    
        return RB.fetch_by_wrapper("", &wrapper).await;
    }

    pub async fn delete(&self, u: &User) -> Result<u64> {
        let user = u.clone();
        if user.id.is_none() {
            return Err(Error::from("id is empty!"));
        }
        RB.remove_by_id::<User>("", &user.id.unwrap().to_string()).await

    }

    pub async fn list(&self, pageable: &Pageable) -> Result<Page<User>> {
        let wrapper = RB.new_wrapper().check()?;
        let user_list = RB.fetch_page_by_wrapper("", &wrapper, &PageRequest::new(pageable.page.unwrap_or(0), pageable.size.unwrap_or(10))).await?;
        Ok(user_list)
    }


}
