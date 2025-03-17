// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use aes_gcm::aead::rand_core::RngCore;
use aes_gcm::aead::OsRng;
use std::fmt::{Debug, Display, Formatter};

#[derive(Clone, Copy, PartialEq)]
pub struct SecretKey([u8; 32]);

impl SecretKey {
    pub fn as_slice(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl SecretKey {
    pub fn generate() -> Self {
        let mut key = [0u8; 32];
        OsRng.fill_bytes(&mut key);
        SecretKey(key)
    }
}

impl From<String> for SecretKey {
    fn from(value: String) -> Self {
        let bytes = hex::decode(value).expect("invalid hex string");
        let array: [u8; 32] = bytes
            .as_slice()
            .try_into()
            .expect("secret key must be 32 bytes");
        SecretKey(array)
    }
}

impl From<&str> for SecretKey {
    fn from(value: &str) -> Self {
        let bytes = hex::decode(value).expect("invalid hex string");
        let array: [u8; 32] = bytes
            .as_slice()
            .try_into()
            .expect("secret key must be 32 bytes");
        SecretKey(array)
    }
}

impl From<SecretKey> for String {
    fn from(value: SecretKey) -> Self {
        hex::encode(value.0)
    }
}

impl Debug for SecretKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("****************")
    }
}

impl Display for SecretKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("****************")
    }
}

#[cfg(test)]
mod tests {
    use crate::crypt::SecretKey;

    #[test_log::test]
    fn test_secret_key_de_enconding() {
        let key = SecretKey::generate();
        let string: String = key.into();
        println!("{}", string);

        let result = SecretKey::from(string);
        assert_eq!(result, key);
    }
}
