use std::sync::Arc;

use async_graphql::{Context, Error, FieldResult, Object};
use axum::{response::IntoResponse, Json};

use super::model;
use crate::context::ServerContext;

#[derive(Default)]
pub struct HealthQuery;

#[Object]
impl HealthQuery {
    pub async fn health(&self, ctx: &Context<'_>) -> FieldResult<model::Health> {
        let server_ctx = ctx.data::<Arc<ServerContext>>()?;

        let result = server_ctx.health_service.find_health().await;
        match result {
            Ok(res) => Ok(res.into()),
            Err(err) => Err(Error::new(err.to_string())),
        }
    }
}

/// Test server health wihout invoking many
/// moving parts.
#[utoipa::path(
        get,
        path = "/health",
        responses(
            (status = 200, description = "server is running", body = HealthResponse),
        ),
    )]
pub async fn health() -> impl IntoResponse {
    let data = model::Health {
        status: "running".into(),
    };
    let response = model::HealthResponse { data };

    Json(response)
}
