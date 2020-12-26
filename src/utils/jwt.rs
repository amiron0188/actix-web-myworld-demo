use chrono::{Utc, Duration};
use serde::{Serialize, Deserialize};
use jwt::{decode, encode, Header, TokenData, Validation, EncodingKey, DecodingKey, errors::Error};
use std::env;

use crate::models::user::User;
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id: Option<String>,
    pub exp: i64
}

pub struct JWT;

impl JWT {
    pub fn generate_jwt(&self, u: &Option<User>) -> Option<String> {
        let exp = (Utc::now() + Duration::hours(3)).timestamp();
        let claims = Claims {
            id: u.clone().unwrap().id,
            exp: exp
        };

        let header = Header::default();
        let secret = self.get_secret();
        let token = encode(&header, &claims, &EncodingKey::from_secret(secret.as_ref())).ok();

        return token;
    }

    pub fn decode_jwt(&self, token: &str) -> Result<TokenData<Claims>, Error> {
        return  decode::<Claims>(&token, &DecodingKey::from_secret(self.get_secret().as_ref()), &Validation::default());
    }

    pub fn get_secret(&self) -> String {
        env::var("JWT_SECRET").unwrap_or_else(|_| "secret".into())
    }
    
    
}


