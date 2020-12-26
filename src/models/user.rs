use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[crud_enable(table_name:t_user)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub id: Option<String>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub del: Option<i32>
}

impl User {
    pub fn map_from(uf: &UserForm) -> User {
        return User {
            id: uf.id.clone(),
            username: uf.username.clone(),
            email: uf.email.clone(),
            password: Some(uf.password.clone()),
            created_at: None,
            updated_at: None,
            del: None
        };
    }
}

#[derive(Serialize, Deserialize, Validate, Debug, Clone)]
pub struct UserForm {
    pub id: Option<String>,

    #[validate(length(min = 1, max = 20, message ="fails validation - username must be 1-20 characters long!"))]
    pub username: Option<String>,

    #[validate(email(message = "fails validation - eamil is not a valid email adderss!"))]
    pub email: Option<String>,

    #[validate(length(min = 6, max =20, message = "fails validation - password must be 6-20 characters long!"))]
    pub password: String,
}

