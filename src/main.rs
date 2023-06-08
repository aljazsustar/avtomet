#[macro_use]
extern crate dotenv_codegen;
pub mod request;
pub mod parser;
pub mod mail;
pub mod db;

use std::env;

use db::{connection::Connection, MongoDB};
use futures::TryStreamExt;
use mail::send_mail;
use tokio;
use dotenv::dotenv;

use crate::parser::parse_ads;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let mut mongo_host = dotenv!("MONGO_HOST");

    if env::var("PROD").is_ok() && env::var("PROD").unwrap() == "1" {
        mongo_host = dotenv!("MONGO_HOST_PROD");
    }
    println!("{}", mongo_host);
    let db = MongoDB::init(String::from(mongo_host), String::from(dotenv!("MONGO_PORT")), String::from(dotenv!("MONGO_DB"))).await;

    let req = request::RequestBuilder::new().build();
    let res = req.send().await.expect("Request failed");
    let ads = parse_ads(&res);

    for ad in ads.iter() {
        println!("{}, {}, {}, {}, {}",ad.naziv, ad.model, ad.kilometri, ad.naslov, ad.prva_registracija);
    }

    db.insert_ads(&ads).await;
    send_mail(ads[0].clone()).await;

    let mut db_content = db.fetch_all_ads().await;

    println!("-------------DB---------------");

    while let Some(entry) = db_content.try_next().await.unwrap() {
        println!("{}", entry.naziv);
    }
}
