use actix_web::HttpResponse;
use serde::{Serialize, Deserialize, ser};
pub struct Res;
#[derive(Serialize, Deserialize)]
struct JsonOkData<T: ser::Serialize> {
    pub code: u16,
    pub message: Option<String>,
    pub data: Option<T>
}

#[derive(Serialize, Deserialize)]
struct JsonOk {
    pub code: u16,
    message: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct JsonErr<T: ser::Serialize> {
    pub code: u16,
    pub message: Option<T>
}


impl Res {
    #[allow(dead_code)]
    pub fn ok() -> HttpResponse {
        HttpResponse::Ok().json(JsonOk {
            code: 200,
            message: Some("Ok!".to_string())
        })
    }

    #[allow(dead_code)]
    pub fn ok_data<T: ser::Serialize>(data: &T) -> HttpResponse {
        HttpResponse::Ok().json(JsonOkData {
            code: 200,
            message: Some("Ok!".to_string()),
            data: Some(data)
        })
    }
    
}