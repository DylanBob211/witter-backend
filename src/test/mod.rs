mod test_helpers;

use super::*;
use test_helpers::*;
use tide::http::Request;

#[async_std::test]
async fn creating_a_user() {
    let app = server().await;
    let res = Request::build()
        .get()
        .url("/users")
        .send(&app)
        .await
        .to_json()
        .await
        .unwrap();
    assert_json_eq!(json!([]), res);
}