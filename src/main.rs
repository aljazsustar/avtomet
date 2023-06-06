pub mod request;
use tokio;

#[tokio::main]
async fn main() {
    let req = request::RequestBuilder::new().build();
    let res = req.send().await.expect("Request failed");
    println!("{}", res);
}
