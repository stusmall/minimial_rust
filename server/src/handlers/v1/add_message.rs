use crate::dao::Dao;
use crate::error::Error;
use axum::extract::State;
use axum::Json;
use serde::{Deserialize, Serialize};
use tracing::instrument;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub(super) struct AddMessageRequest {
    pub message: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub(super) struct AddMessageResponse {}

#[utoipa::path(
    post,
    path = "/api/v1/message",
    request_body = AddMessageRequest,
    responses(
        (status = 200, description = "", body = [AddMessageResponse]),
        (status = 500, description = "", body = [ErrorResponse])
    )
)]
#[instrument(level = "debug", skip_all)]
pub(super) async fn add_messages<DAO: Dao>(
    State(dao): State<DAO>,
    Json(body): Json<AddMessageRequest>,
) -> Result<Json<AddMessageResponse>, Error> {
    dao.add_item(&body.message).await?;
    Ok(Json(AddMessageResponse {}))
}
