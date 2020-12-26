#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate validator_derive;
#[macro_use]
extern crate rbatis_macro_driver;
#[macro_use]
extern crate lazy_static;

extern crate jsonwebtoken as jwt;

use app::start;
use env_logger::Env;

mod error;
mod res;
mod app;
mod db;
mod utils;
mod models;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    start().await

}
