use nrot::{rot, rot_letter, Mode};

use super::Service;
use crate::{rot::entities::Decrypt, Error};

impl Service {
    pub async fn decrypt(&self, secret: String, rotation: u8) -> Result<Decrypt, Error> {
        let input_length = secret.len();
        let input_bytes = secret.as_bytes();

        let plain = {
            if input_length == 1 {
                let byte_result = rot_letter(Mode::Decrypt, input_bytes[0], rotation);
                format!("{}", String::from_utf8_lossy(&[byte_result]))
            } else {
                let bytes_result = rot(Mode::Decrypt, input_bytes, rotation);
                format!("{}", String::from_utf8_lossy(&bytes_result))
            }
        };

        let rot = Decrypt { plain, rotation };
        Ok(rot)
    }
}
