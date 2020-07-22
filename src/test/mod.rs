mod test_helpers;

use super::*;
use test_helpers::*;
use tide::http::Request;

#[async_std::test]
async fn creating_a_user() {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let db = TestDb::new().await;
    let db_pool = db.db();
    
    let app = server(db_pool).await;
    
    let mut res = Request::build()
        .get()
        .url("/users")
        .send(&app)
        .await;
    
        
    assert_eq!(res.status(), 200);
    assert_json_eq!(json!([]), res.to_json().await.unwrap());

    let res = Request::build()
        .post()
        .url("/users")
        .body(json!({ "username": "bob" }))
        .send(&app)
        .await;

    assert_eq!(res.status(), 201);

    let res = Request::build()
        .get()
        .url("/users")
        .send(&app)
        .await
        .to_json()
        .await
        .unwrap();
    
    assert_json_include!(actual: res, expected: json!([{ "username": "bob" }]));
}
