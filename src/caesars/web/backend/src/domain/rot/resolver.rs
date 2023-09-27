use std::sync::Arc;

use async_graphql::{Context, Error, FieldResult, Object};
use frunk_core::labelled::Transmogrifier;

use super::model;
use crate::context::ServerContext;

#[derive(Default)]
pub struct RotQuery;

#[Object]
impl RotQuery {
    pub async fn encrypt(
        &self,
        ctx: &Context<'_>,
        plain: String,
        rotation: u8,
    ) -> FieldResult<model::Encrypt> {
        let server_ctx = ctx.data::<Arc<ServerContext>>()?;

        let result = server_ctx.rot_service.encrypt(plain, rotation).await;
        match result {
            Ok(res) => Ok(res.transmogrify()),
            Err(err) => Err(Error::new(err.to_string())),
        }
    }

    pub async fn decrypt(
        &self,
        ctx: &Context<'_>,
        secret: String,
        rotation: u8,
    ) -> FieldResult<model::Decrypt> {
        let server_ctx = ctx.data::<Arc<ServerContext>>()?;

        let result = server_ctx.rot_service.decrypt(secret, rotation).await;
        match result {
            Ok(res) => Ok(res.transmogrify()),
            Err(err) => Err(Error::new(err.to_string())),
        }
    }
}
