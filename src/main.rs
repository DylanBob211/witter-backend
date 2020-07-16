use dotenv;
use serde_json;
use serde_json::json;
use sqlx;
use sqlx::{query, query_as, PgPool, Pool};
use tide;
use tide::{Request, Server};
use uuid::Uuid;
use serde::Serialize;

#[async_std::main]
async fn main() {
    let app = server().await;
    app.listen("127.0.0.1:9000").await.unwrap();
}

#[cfg(not(test))]
async fn make_db_pool() -> PgPool {
    let db_url = std::env::var("DATABASE_URL").unwrap();
    Pool::new(&db_url).await.unwrap()
}

#[cfg(test)]
async fn make_db_pool() -> PgPool {
    let db_url = std::env::var("DATABASE_URL_TEST").unwrap();
    Pool::new(&db_url).await.unwrap()
}

async fn server() -> Server<State> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();
    let db_pool: PgPool = make_db_pool().await;

    let mut app = tide::with_state(State { db_pool });

    app.at("/users").get(|req: Request<State>| async move {
        let db_pool = &req.state().db_pool;

        let users: Vec<User> = query_as!(User, "select id, username from users")
            .fetch_all(db_pool)
            .await?;

    
        let resp = json!(users);
        Ok(resp)
    });

    app
}

#[derive(Debug)]
struct State {
    db_pool: PgPool,
}

#[cfg(test)]
mod test;

#[derive(Debug, Serialize)]
struct User {
    id: Uuid,
    username: String,
}
