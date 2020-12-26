use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Validate ,Debug, Clone)]
pub struct LoginForm {
    #[validate(length(min = 1, message = "fails validation - username must be not empty!"))]
    pub username: Option<String>,

    #[validate(length(min = 6, max = 20, message = "fails validation - password must be 6-20 characters long!" ))]
    pub password: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginInfo {
    pub token: Option<String>,
    pub username: Option<String>
}