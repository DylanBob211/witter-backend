use async_trait::async_trait;
use dotenv;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::json;
use sqlx;
pub use sqlx::prelude::*;
use sqlx::{query, query_as, PgPool, Pool};
use tide;
use tide::{Request, Response, Server, StatusCode};
use uuid::Uuid;

#[async_std::main]
async fn main() {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let db_pool: PgPool = make_db_pool().await;
    let app = server(db_pool).await;
    app.listen("127.0.0.1:9000").await.unwrap();
}


async fn make_db_pool() -> PgPool {
    let db_url = std::env::var("DATABASE_URL").unwrap();
    Pool::new(&db_url).await.unwrap()
}


async fn server(db_pool: PgPool) -> Server<State> {
    let mut app = tide::with_state(State { db_pool });

    app.at("/users")
        .get(|req: Request<State>| async move {
            let db_pool = &req.state().db_pool;

            let users: Vec<User> = query_as!(User, "select id, username from users")
                .fetch_all(db_pool)
                .await?;
            let mut res = Response::new(StatusCode::Ok);
            res.set_body(json!(users));

            Ok(res)
        })
        .post(|mut req: Request<State>| async move {
            let db_pool = &req.state().db_pool.clone();
            // let body = req.to_json().await.unwrap(); // TODO: solve unwrapping
            // println!("{}", body);
            let body = req.body_json::<CreatedUser>().await?;
            query!(
                r#"
                    insert into users (id, username)
                    values ($1, $2)
                "#,
                Uuid::new_v4(),
                body.username,
            )
            .execute(db_pool)
            .await?;
            let res = Response::new(StatusCode::Created);
            Ok(res)
        });

    app
}

#[derive(Debug)]
pub struct State {
    db_pool: PgPool,
}

#[cfg(test)]
mod test;

#[derive(Debug, Serialize)]
struct User {
    id: Uuid,
    username: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct CreatedUser {
    username: String,
}

#[async_trait]
pub trait BodyJson {
    async fn to_json(&mut self) -> Result<serde_json::Value, Box<dyn std::error::Error>>;
}

#[async_trait]
impl BodyJson for Request<State> {
    async fn to_json(&mut self) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let s = self.body_string().await?;
        Ok(serde_json::from_str(&s)?)
    }
}
