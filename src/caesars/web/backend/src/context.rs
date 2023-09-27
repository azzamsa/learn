use std::sync::Arc;

use crate::domain::{health, meta, rot};

#[derive(Clone)]
pub struct ServerContext {
    pub meta_service: Arc<meta::Service>,
    pub health_service: Arc<health::Service>,
    pub rot_service: Arc<rot::Service>,
}
