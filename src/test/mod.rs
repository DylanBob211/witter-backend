mod test_helpers;

use super::*;
use test_helpers::*;
use tide::http::Request;

#[async_std::test]
async fn creating_a_user() {
    let backend = test_setup().await;

    let mut res = get("/users").send(backend.server()).await;

    assert_eq!(res.status(), 200);
    assert_json_eq!(json!([]), res.to_json().await.unwrap());

    let res = post("/users", json!({ "username": "bob" }))
        .send(backend.server())
        .await;

    assert_eq!(res.status(), 201);

    let res = get("/users").send(backend.server()).await.to_json().await.unwrap();

    assert_json_include!(actual: res, expected: json!([{ "username": "bob" }]));
}
