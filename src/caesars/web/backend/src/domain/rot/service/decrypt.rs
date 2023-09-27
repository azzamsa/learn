use nrot::{rot, rot_letter, Mode};

use super::Service;
use crate::domain::rot::entities;

impl Service {
    pub async fn decrypt(
        &self,
        secret: String,
        rotation: u8,
    ) -> Result<entities::Decrypt, crate::Error> {
        let input_length = secret.len();
        let input_bytes = secret.as_bytes();

        let bytes_result = rot(Mode::Decrypt, input_bytes, rotation);
        let mut plain = format!("{}", String::from_utf8_lossy(&bytes_result));

        if input_length == 1 {
            let byte_result = rot_letter(Mode::Decrypt, input_bytes[0], rotation);
            plain = format!("{}", String::from_utf8_lossy(&[byte_result]));
        };

        let rot = entities::Decrypt { plain, rotation };
        Ok(rot)
    }
}
