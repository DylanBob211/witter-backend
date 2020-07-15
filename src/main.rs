use dotenv;
use serde_json;
use serde_json::json;
use sqlx;
use sqlx::{query, PgPool, Pool};
use tide;
use tide::http::StatusCode;
use tide::{Request, Response, Server};

#[async_std::main]
async fn main() {
    let app = server().await;
    app.listen("127.0.0.1:9000").await.unwrap();
}

async fn server() -> Server<State> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();
    let db_url = std::env::var("DATABASE_URL").unwrap();
    let db_pool: PgPool = Pool::new(&db_url).await.unwrap();

    let mut app = tide::with_state(State { db_pool });

    app.at("/").get(|req: Request<State>| async move {
        // let db_pool = &req.state().db_pool;
        // let rows = query!("Select 1 as one where 1 = 2").fetch_one(db_pool).await?;

        // let resp = json!({
        //     "code": 200,
        //     "success": true
        // });

        let json = json!([1, 2, 3]);
        Ok(json)
    });

    app
}

#[derive(Debug)]
struct State {
    db_pool: PgPool,
}

#[cfg(test)]
mod test;