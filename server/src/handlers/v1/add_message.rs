use axum::extract::State;
use axum::Json;
use tracing::instrument;
use crate::dao::Dao;
use  serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::error::Error;

#[derive(Serialize, Deserialize, ToSchema)]
pub(super) struct AddMessageRequest {
    pub message: String
}

#[derive(Serialize, Deserialize, ToSchema)]
pub(super) struct AddMessageResponse {

}

#[instrument(level = "debug", skip_all)]
pub(super) async fn add_messages<DAO: Dao>(State(dao): State<DAO>, Json(body): Json<AddMessageRequest>) -> Result<Json<AddMessageResponse>, Error> {
    dao.add_item(&body.message).await?;
    Ok(Json(AddMessageResponse {
    }))
}
