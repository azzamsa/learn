#[cxx_qt::bridge]
mod my_object {

    use nrot::{rot, rot_letter, Mode};

    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    #[cxx_qt::qobject(qml_uri = "caesar", qml_version = "1.0")]
    pub struct Rot {
        #[qproperty]
        plain: QString,
        #[qproperty]
        secret: QString,
    }

    impl Default for Rot {
        fn default() -> Self {
            Self {
                plain: QString::from(""),
                secret: QString::from(""),
            }
        }
    }

    impl qobject::Rot {
        #[qinvokable]
        pub fn encrypt(&self, plain: &QString) -> QString {
            let rotation = 13; // common ROT rotation
            let plain = plain.to_string();
            let plain = plain.as_bytes();

            let bytes_result = rot(Mode::Encrypt, plain, rotation);
            let mut secret = format!("{}", String::from_utf8_lossy(&bytes_result));

            if plain.len() == 1 {
                let byte_result = rot_letter(Mode::Encrypt, plain[0], rotation);
                secret = format!("{}", String::from_utf8_lossy(&[byte_result]));
            };

            QString::from(&secret)
        }

        #[qinvokable]
        pub fn decrypt(&self, secret: &QString) -> QString {
            let rotation = 13; // common ROT rotation
            let secret = secret.to_string();
            let secret = secret.as_bytes();

            let bytes_result = rot(Mode::Decrypt, secret, rotation);
            let mut plain = format!("{}", String::from_utf8_lossy(&bytes_result));

            if secret.len() == 1 {
                let byte_result = rot_letter(Mode::Decrypt, secret[0], rotation);
                plain = format!("{}", String::from_utf8_lossy(&[byte_result]));
            };

            QString::from(&plain)
        }
    }
}
