use reqwest::{redirect::Policy, Client, Error, header::{HeaderMap, HeaderValue}};

use self::filter::Filter;

pub mod filter;

#[derive(Debug, Clone, Copy)]
pub enum RequestType {
    GET,
    POST,
    OPTIONS,
    HEAD,
}

#[derive(Debug, Clone)]
pub struct Request {
    pub request_type: RequestType,
    pub url: String,
    client: Client,
    headers: HeaderMap
}

impl Default for Request {
    fn default() -> Self {

        let mut headers = HeaderMap::new();
        headers.append("Accept", HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8"));
        headers.append("Accept-Language", HeaderValue::from_static("en-US,en;q=0.5"));
        headers.append("Accept-Encoding", HeaderValue::from_static("gzip, deflate, br"));
        headers.append("DNT", HeaderValue::from_static("1"));
        headers.append("Connection", HeaderValue::from_static("keep-alive"));
        headers.append("Sec-Fetch-Dest", HeaderValue::from_static("document"));
        headers.append("Sec-Fetch-Mode", HeaderValue::from_static("navigate"));
        headers.append("Sec-Fetch-Site", HeaderValue::from_static("cross-site"));
        headers.append("Sec-GPC", HeaderValue::from_static("1"));
        headers.append("User-Agent", HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:109.0) Gecko/20100101 Firefox/113.0"));

        let policy = Policy::custom(|attempt| {
            println!("{}", attempt.url());
            if attempt.url().path().contains("error") {
                attempt.error("Request error")
            } else {
                attempt.follow()
            }
        });
        let client = Client::builder().redirect(policy).gzip(true).build().unwrap();
        Self {
            request_type: RequestType::GET,
            url: String::from("https://www.avto.net/Ads/results.asp"),
            client,
            headers
        }
    }

}

impl Request {
    pub async fn send(&self) -> Result<String, Error> {
       
        self.client
            .execute(self.client.get(&self.url).headers(self.headers.clone()).query(&Filter::default()).build().unwrap())
            .await?
            .text()
            .await
    }
}

pub struct RequestBuilder {
    request: Request,
}

impl RequestBuilder {
    pub fn new() -> Self {
        RequestBuilder {
            request: Request::default(),
        }
    }

    pub fn add_url(&mut self, url: String) -> &mut Self {
        self.request.url = url;
        self
    }

    pub fn add_request_type(&mut self, request_type: RequestType) -> &mut Self {
        self.request.request_type = request_type;
        self
    }

    pub fn build(&mut self) -> Request {
        self.request.clone()
    }
}
