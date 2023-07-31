use axum::body::Body;
use axum::Router;
use axum::routing::{get, post};
use crate::dao::Dao;

mod add_message;
mod get_messages;


pub fn build_router<DAO: Dao>() -> Router<DAO, Body>{
    Router::new()
        .route("/message", get(get_messages::get_messages::<DAO>))
        .route("/message", post(get_messages::get_messages::<DAO>))
}
#[cfg(test)]
mod test {
    use axum::http;
    use axum::http::{Method, Request};
    use serde_json::json;
    use http::StatusCode;
    use tower::ServiceExt;
    use crate::dao::DaoTest;
    use super::*;
    #[tokio::test]
    async fn happy_path_test() {
        let dao = DaoTest::default();
        let app:Router<(), Body> = build_router::<DaoTest>()
            .with_state(dao.clone());

        let add_message_request = Request::builder()
            .uri("/message")
            .method(Method::POST)
            .body(Body::from(
                serde_json::to_vec(&json!({
                    "message": "oh hai"
                })).unwrap(),
            )).unwrap();
        let response = app.oneshot(add_message_request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        todo!()
    }
}