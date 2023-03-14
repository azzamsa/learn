use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct EncryptResponse {
    pub data: EncryptWrapper,
}

#[derive(Debug, Deserialize)]
pub struct EncryptWrapper {
    pub encrypt: Encrypt,
}

#[derive(Debug, Deserialize)]
pub struct Encrypt {
    pub secret: String,
}

#[derive(Debug, Deserialize)]
pub struct DecryptResponse {
    pub data: DecryptWrapper,
}

#[derive(Debug, Deserialize)]
pub struct DecryptWrapper {
    pub decrypt: Decrypt,
}

#[derive(Debug, Deserialize)]
pub struct Decrypt {
    pub plain: String,
}
