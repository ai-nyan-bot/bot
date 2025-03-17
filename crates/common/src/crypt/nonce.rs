// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use aes_gcm::aead::rand_core::RngCore;
use aes_gcm::aead::OsRng;
use std::fmt::{Debug, Display, Formatter};

#[derive(Clone, Copy, PartialEq)]
pub struct Nonce([u8; 12]);

impl Nonce {
    pub fn as_slice(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl Nonce {
    pub fn generate() -> Self {
        let mut key = [0u8; 12];
        OsRng.fill_bytes(&mut key);
        Nonce(key)
    }
}

impl From<String> for Nonce {
    fn from(value: String) -> Self {
        let bytes = hex::decode(value).expect("invalid hex string");
        let array: [u8; 12] = bytes
            .as_slice()
            .try_into()
            .expect("secret key must be 12 bytes");
        Nonce(array)
    }
}

impl From<&str> for Nonce {
    fn from(value: &str) -> Self {
        let bytes = hex::decode(value).expect("invalid hex string");
        let array: [u8; 12] = bytes
            .as_slice()
            .try_into()
            .expect("secret key must be 12 bytes");
        Nonce(array)
    }
}

impl From<Nonce> for String {
    fn from(value: Nonce) -> Self {
        hex::encode(value.0)
    }
}

impl Debug for Nonce {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("****************")
    }
}

impl Display for Nonce {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("****************")
    }
}

#[cfg(test)]
mod tests {
    use crate::crypt::Nonce;

    #[test_log::test]
    fn test_nonce_de_enconding() {
        let key = Nonce::generate();
        let string: String = key.into();
        println!("{string}");
        let result = Nonce::from(string);
        assert_eq!(result, key);
    }
}
