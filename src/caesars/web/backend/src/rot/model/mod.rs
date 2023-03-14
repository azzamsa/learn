use async_graphql::SimpleObject;

use crate::rot::entities;

#[derive(Debug, SimpleObject)]
pub struct Encrypt {
    pub secret: String,
    pub rotation: u8,
}

impl From<entities::Encrypt> for Encrypt {
    fn from(rot: entities::Encrypt) -> Self {
        Self {
            secret: rot.secret,
            rotation: rot.rotation,
        }
    }
}
#[derive(Debug, SimpleObject)]
pub struct Decrypt {
    pub plain: String,
    pub rotation: u8,
}

impl From<entities::Decrypt> for Decrypt {
    fn from(rot: entities::Decrypt) -> Self {
        Self {
            plain: rot.plain,
            rotation: rot.rotation,
        }
    }
}
