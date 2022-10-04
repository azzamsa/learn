use nrot::{rot, rot_letter, Mode};

use super::Service;
use crate::rot::entities;

impl Service {
    pub async fn encrypt(
        &self,
        secret: String,
        rotation: u8,
    ) -> Result<entities::Encrypt, crate::Error> {
        let input_length = secret.len();
        let input_bytes = secret.as_bytes();

        let secret = {
            if input_length == 1 {
                let byte_result = rot_letter(Mode::Encrypt, input_bytes[0], rotation);
                format!("{}", String::from_utf8_lossy(&[byte_result]))
            } else {
                let bytes_result = rot(Mode::Encrypt, input_bytes, rotation);
                format!("{}", String::from_utf8_lossy(&bytes_result))
            }
        };

        let rot = entities::Encrypt { secret, rotation };
        Ok(rot)
    }
}
