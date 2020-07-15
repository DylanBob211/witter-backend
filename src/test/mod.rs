#![allow(dead_code)]

use super::*;
use async_trait::async_trait;
use tide::http::{Method, Request, Response, Url};


#[async_std::test]
async fn a_test() {
    let app = server().await;
    let res = Request::build().get().url("/").send(&app).await.to_json().await.unwrap();
    // let js = serde_json::from_str(&res.body_string().await.unwrap()).unwrap();
    assert_json_diff::assert_json_eq!(json!([1, 2, 3]), res);
}



#[async_trait]
trait BodyJson {
    async fn to_json(&mut self) -> Result<serde_json::Value, Box<dyn std::error::Error>>;
}

#[async_trait]
impl BodyJson for Response {
    async fn to_json(&mut self) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let body = self.body_string().await?;
        Ok(serde_json::from_str(&body)?)
    }
}

trait MakeRequestBuilder {
    fn build() -> RequestBuilder;
}

impl MakeRequestBuilder for Request {
    fn build() -> RequestBuilder {
        RequestBuilder::default()
    }
}

#[derive(Debug, Default)]
struct RequestBuilder {
    method: Option<Method>,
    url: Option<String>
}

impl RequestBuilder {
    pub fn get(mut self) -> Self {
        self.method = Some(Method::Get);
        self
    }

    pub fn post(mut self) -> Self {
        self.method = Some(Method::Post);
        self
    }

    pub fn put(mut self) -> Self {
        self.method = Some(Method::Put);
        self
    }

    pub fn patch(mut self) -> Self {
        self.method = Some(Method::Patch);
        self
    }

    pub fn delete(mut self) -> Self {
        self.method = Some(Method::Delete);
        self
    }

    pub fn url(mut self, url: &str) -> Self {
        self.url = Some(url.to_string());
        self
    }

    pub async fn send(mut self, server: &Server<State>) -> Response {
        let url = Url::parse(&format!("http://example.com{}", self.url.expect("url non definito"))).unwrap();
        let req = Request::new(self.method.take().unwrap(), url);
        server.respond(req).await.unwrap()
    }
}
