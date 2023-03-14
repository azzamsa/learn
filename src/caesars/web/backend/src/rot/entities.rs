#[derive(Debug)]
pub struct Encrypt {
    pub secret: String,
    pub rotation: u8,
}

#[derive(Debug)]
pub struct Decrypt {
    pub plain: String,
    pub rotation: u8,
}
