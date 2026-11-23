use sha2::{Digest, Sha256};

pub struct Encriptor;

impl Encriptor {
    pub fn encript(value: &String) -> String {
        hex::encode(Sha256::digest(value))
    }

    pub fn validate(value: &String) -> bool {
        let encoded = hex::encode(Sha256::digest(value));

        if &encoded == value {
            return true;
        }
        false
    }
}
