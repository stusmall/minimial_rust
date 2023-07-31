use axum::extract::State;
use axum::Json;
use tracing::instrument;
use crate::dao::Dao;
use  serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::error::Error;

#[derive(Serialize, Deserialize, ToSchema)]
pub(super) struct MessageResponse {
    pub messages: Vec<String>
}

#[instrument(level = "debug", skip_all)]
pub(super) async fn get_messages<DAO: Dao>(State(dao): State<DAO>) -> Result<Json<MessageResponse>, Error> {
    Ok(Json(MessageResponse {
        messages:  dao.get_items().await?
    }))
}
