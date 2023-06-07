#[macro_use]
extern crate dotenv_codegen;
pub mod request;
pub mod parser;
pub mod mail;
use mail::send_mail;
use tokio;
use dotenv::dotenv;

use crate::parser::parse_ads;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let req = request::RequestBuilder::new().build();
    let res = req.send().await.expect("Request failed");
    let ads = parse_ads(&res);

    for ad in ads.iter() {
        println!("{}, {}, {}, {}, {}",ad.naziv, ad.model, ad.kilometri, ad.naslov, ad.prva_registracija);
    }

    send_mail(ads[0].clone()).await;
}
