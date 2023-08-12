use crate::dao::Dao;
use axum::body::Body;
use axum::routing::{get, post};
use axum::Router;
use utoipa::OpenApi;

mod add_message;
mod get_messages;

use get_messages::*;


#[derive(OpenApi)]
#[openapi(
    paths(get_messages),
    components(schemas(MessageResponse)),
    tags()
)]
pub struct ApiDoc;

pub fn build_router<DAO: Dao>() -> Router<DAO, Body> {
    Router::new()
        .route("/message", get(get_messages::get_messages::<DAO>))
        .route("/message", post(add_message::add_messages::<DAO>))
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::dao::DaoTest;
    use axum::http;
    use axum::http::{Method, Request};
    use http::StatusCode;
    use serde_json::{json, Value};
    use tower::ServiceExt;
    #[tokio::test]
    async fn happy_path_test() {
        let dao = DaoTest::default();
        let app: Router<(), Body> = build_router::<DaoTest>().with_state(dao.clone());

        let add_message_request = Request::builder()
            .uri("/message")
            .method(Method::POST)
            .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(
                serde_json::to_vec(&json!({
                    "message": "oh hai"
                }))
                .unwrap(),
            ))
            .unwrap();
        let response = app.clone().oneshot(add_message_request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let fetch_messages = Request::builder()
            .uri("/message")
            .method(Method::GET)
            .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::empty())
            .unwrap();
        let response = app.oneshot(fetch_messages).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: Value = serde_json::from_slice(&body).unwrap();
        let msgs = body.get("messages").unwrap().as_array().unwrap();
        assert_eq!(msgs, json!(["oh hai"]).as_array().unwrap());
        assert_eq!(*dao.contents.lock().unwrap(), vec![String::from("oh hai")]);
    }
}
