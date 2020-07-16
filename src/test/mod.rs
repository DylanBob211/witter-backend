mod test_helpers;

use super::*;
use test_helpers::*;
use tide::http::Request;

#[async_std::test]
async fn a_test() {
    let app = server().await;
    let res = Request::build()
        .get()
        .url("/")
        .send(&app)
        .await
        .to_json()
        .await
        .unwrap();
    assert_json_diff::assert_json_eq!(json!([1, 2, 3]), res);
}


