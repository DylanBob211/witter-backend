#![allow(dead_code)]

mod test_db;

use crate::{server, BodyJson, State};
pub use assert_json_diff::*;
use async_trait::async_trait;
use test_db::TestDb;
use tide::{
    http::{Body, Method, Request, Response, Url},
    Server,
};

pub async fn test_setup() -> TestBackend {
    let test_db = TestDb::new().await;
    let test_db_pool = test_db.db();

    let server = server(test_db_pool).await;
    TestBackend::new(server, test_db)
}

pub struct TestBackend {
    test_server: Server<State>,
    test_db: TestDb,
}

impl TestBackend {
    fn new(service: Server<State>, db: TestDb) -> Self {
        Self {
            test_server: service,
            test_db: db,
        }
    }

    pub fn server(&self) -> &Server<State> {
        &self.test_server
    }
}

#[async_trait]
impl BodyJson for Response {
    async fn to_json(&mut self) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let body = self.body_string().await?;
        Ok(serde_json::from_str(&body)?)
    }
}

#[derive(Debug)]
pub struct TestRequest {
    method: Method,
    url: String,
    body: Option<Body>,
}

impl TestRequest {
    pub async fn send(self, server: &Server<State>) -> Response {
        let url = Url::parse(&format!("http://example.com{}", self.url)).unwrap();
        let mut req = Request::new(self.method, url);
        if let Some(body) = self.body {
            req.set_body(body);
        }
        server.respond(req).await.unwrap()
    }
}

pub fn get(url: &str) -> TestRequest {
    TestRequest {
        url: url.to_string(),
        method: Method::Get,
        body: None,
    }
}
pub fn post(url: &str, body: impl Into<Body>) -> TestRequest {
    TestRequest {
        url: url.to_string(),
        method: Method::Post,
        body: Some(body.into()),
    }
}
