#![allow(dead_code)]
use crate::{BodyJson, State};
pub use assert_json_diff::*;
use async_trait::async_trait;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use sqlx::prelude::Connect;
use sqlx::{Postgres, PgPool, Pool};
use tide::{
    http::{Body, Method, Request, Response, Url},
    Server,
};

#[async_trait]
impl BodyJson for Response {
    async fn to_json(&mut self) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let body = self.body_string().await?;
        Ok(serde_json::from_str(&body)?)
    }
}

pub trait MakeRequestBuilder {
    fn build() -> RequestBuilder;
}

impl MakeRequestBuilder for Request {
    fn build() -> RequestBuilder {
        RequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct RequestBuilder {
    method: Option<Method>,
    url: Option<String>,
    body: Option<Body>,
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

    pub fn body(mut self, body: impl Into<Body>) -> Self {
        self.body = Some(body.into());
        self
    }

    pub async fn send(mut self, server: &Server<State>) -> Response {
        let url = Url::parse(&format!(
            "http://example.com{}",
            self.url.expect("url non definito")
        ))
        .unwrap();
        let mut req = Request::new(self.method.take().unwrap(), url);
        if let Some(body) = self.body {
            req.set_body(body);
        }
        server.respond(req).await.unwrap()
    }
}

pub fn db_url() -> String {
    let rng = thread_rng();
    let suffix: String = rng.sample_iter(&Alphanumeric).take(16).collect();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL missing from environment");
    format!("{}_{}", db_url, suffix)
}

fn parse_db_url(db_url: &str) -> (&str, &str) {
    let separator_pos = db_url.rfind("/").unwrap();
    let pg_conn = &db_url[..=separator_pos];
    let db_name = &db_url[separator_pos + 1..];
    (pg_conn, db_name)
}

async fn create_db(db_url: &str) {
    let (pg_conn, db_name) = parse_db_url(db_url);
    println!("{}", pg_conn);
    let mut conn = sqlx::PgConnection::connect(pg_conn).await.unwrap();

    let sql = format!(r#"CREATE DATABASE "{}""#, &db_name);
    sqlx::query::<Postgres>(&sql)
        .execute(&mut conn)
        .await
        .unwrap();
}

async fn drop_db(db_url: &str) {
    let (pg_conn, db_name) = parse_db_url(db_url);
    let mut conn = sqlx::PgConnection::connect(pg_conn).await.unwrap();
    let sql = format!(
        r#"
    SELECT pg_terminate_backend(pg_stat_activity.pid)
    FROM pg_stat_activity
    WHERE pg_stat_activity.datname = '{db}'
    AND pid <> pg_backend_pid();
    "#,
        db = db_name
    );
    sqlx::query::<Postgres>(&sql)
        .execute(&mut conn)
        .await
        .unwrap();
    let sql = format!(r#"DROP DATABASE "{db}""#, db = db_name);
    sqlx::query::<Postgres>(&sql)
        .execute(&mut conn)
        .await
        .unwrap();
}

pub async fn run_migrations(db_url: &str) {
    let mut conn = sqlx::PgConnection::connect(db_url).await.unwrap();  
    
    let sql = async_std::fs::read_to_string("bin/setup.sql").await.unwrap();
    sqlx::query::<Postgres>(&sql)
        .execute(&mut conn)
        .await
        .unwrap();
}

pub struct TestDb {
    db_url: String,
    db_pool: Option<PgPool>
}

impl TestDb {
    pub async fn new() -> Self {
        let db_url = db_url();
        create_db(&db_url).await;
        run_migrations(&db_url).await;

        let db = Pool::new(&db_url).await.unwrap();

        Self {
            db_url,
            db_pool: Some(db)
        }
    }

    pub fn db(&self) -> PgPool {
        self.db_pool.clone().unwrap()
    }
}


impl Drop for TestDb {
    fn drop(&mut self) {
        let _ = self.db_pool.take();
        futures::executor::block_on(drop_db(&self.db_url));
    }
}