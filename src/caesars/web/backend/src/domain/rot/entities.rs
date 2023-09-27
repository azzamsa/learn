use frunk::LabelledGeneric;

#[derive(Debug, LabelledGeneric)]
pub struct Encrypt {
    pub secret: String,
    pub rotation: u8,
}

#[derive(Debug, LabelledGeneric)]
pub struct Decrypt {
    pub plain: String,
    pub rotation: u8,
}
