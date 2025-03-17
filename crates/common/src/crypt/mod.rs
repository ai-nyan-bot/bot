// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use aes_gcm::aead::Aead;
use aes_gcm::{Aes256Gcm, Key, KeyInit};
pub use key::SecretKey;
pub use nonce::Nonce;

mod key;
mod nonce;

pub fn encrypt_string(key: &SecretKey, nonce: &Nonce, data: &[u8]) -> Option<String> {
    let nonce = aes_gcm::Nonce::from_slice(nonce.as_slice());
    let algo = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key.as_slice()));

    let encrypted_data = algo.encrypt(nonce, data.as_ref()).ok()?;
    Some(hex::encode(encrypted_data))
}

pub fn decrypt_string(key: &SecretKey, nonce: &Nonce, encrypted: String) -> Option<String> {
    let nonce = aes_gcm::Nonce::from_slice(nonce.as_slice());
    let algo = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key.as_slice()));

    let combined_data = hex::decode(encrypted).ok()?;
    let decrypted_data = algo.decrypt(nonce, combined_data.as_slice()).ok()?;

    String::from_utf8(decrypted_data).ok()
}

#[cfg(test)]
mod test {
    use crate::crypt::{decrypt_string, encrypt_string, Nonce, SecretKey};

    #[test]
    fn test_encrypt_decrypt() {
        let key = SecretKey::generate();
        let nonce = Nonce::generate();

        let encrypted = encrypt_string(&key, &nonce, "nyan.bot".as_bytes()).unwrap();
        let decrypted = decrypt_string(&key, &nonce, encrypted).unwrap();
        assert_eq!(decrypted, "nyan.bot");
    }

    #[test]
    fn test_different_nonce() {
        let key = SecretKey::generate();
        let nonce = Nonce::generate();

        let encrypted = encrypt_string(&key, &nonce, "nyan.bot".as_bytes()).unwrap();

        let nonce = Nonce::generate();
        let result = decrypt_string(&key, &nonce, encrypted);
        assert_eq!(result, None);
    }

    #[test]
    fn test_different_key() {
        let key = SecretKey::generate();
        let nonce = Nonce::generate();

        let encrypted = encrypt_string(&key, &nonce, "nyan.bot".as_bytes()).unwrap();

        let key = SecretKey::generate();
        let result = decrypt_string(&key, &nonce, encrypted);
        assert_eq!(result, None);
    }
}
