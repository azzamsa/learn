use async_graphql::SimpleObject;
use frunk::LabelledGeneric;

#[derive(Debug, SimpleObject, LabelledGeneric)]
pub struct Encrypt {
    pub secret: String,
    pub rotation: u8,
}

#[derive(Debug, SimpleObject, LabelledGeneric)]
pub struct Decrypt {
    pub plain: String,
    pub rotation: u8,
}
