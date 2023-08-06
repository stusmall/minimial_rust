use crate::dao::Dao;
use crate::error::Error;
use axum::extract::State;
use axum::Json;
use serde::{Deserialize, Serialize};
use tracing::instrument;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub(super) struct MessageResponse {
    pub messages: Vec<String>,
}


#[utoipa::path(
    get,
    path = "/api/v1/message",
    responses(
        (status = 200, description = "Get a list of all messages added", body = [MessageResponse])
    )
)]
#[instrument(level = "debug", skip_all)]
pub(super) async fn get_messages<DAO: Dao>(
    State(dao): State<DAO>,
) -> Result<Json<MessageResponse>, Error> {
    Ok(Json(MessageResponse {
        messages: dao.get_items().await?,
    }))
}
