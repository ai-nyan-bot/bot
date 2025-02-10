// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use solana_sdk::bs58;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

#[derive(Clone, Debug)]
pub struct KeyPair {
    pub public: PublicKey,
    pub private: PrivateKey,
}

impl KeyPair {
    pub fn generate() -> Self {
        let key_pair = Keypair::new();
        Self {
            public: PublicKey::from(key_pair.pubkey()),
            private: PrivateKey::from(key_pair),
        }
    }

    pub fn from_base58(str: &str) -> Self {
        let kp = Keypair::from_base58_string(str);
        Self {
            public: kp.pubkey().into(),
            private: kp.into(),
        }
    }
}

impl From<KeyPair> for Keypair {
    fn from(value: KeyPair) -> Self {
        Keypair::from_base58_string(value.private.0.as_str())
    }
}

#[derive(Debug)]
pub enum PublicKeyError {
    Invalid(String),
}

impl fmt::Display for PublicKeyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PublicKeyError::Invalid(tag) => write!(f, "Invalid public key: {}", tag),
        }
    }
}

impl std::error::Error for PublicKeyError {}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct PublicKey(pub String);

impl PartialEq<&str> for PublicKey {
    fn eq(&self, other: &&str) -> bool {
        self.0.as_str() == *other
    }
}

impl Display for PublicKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PublicKey {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

impl FromStr for PublicKey {
    type Err = PublicKeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let key = Pubkey::from_str(s).map_err(|_| PublicKeyError::Invalid(s.to_string()))?;
        Ok(PublicKey(key.to_string()))
    }
}

impl From<String> for PublicKey {
    fn from(value: String) -> Self {
        PublicKey::from_str(value.as_str()).unwrap()
    }
}

impl From<PublicKey> for Pubkey {
    fn from(value: PublicKey) -> Self {
        Pubkey::from_str(value.0.as_str()).unwrap()
    }
}

impl From<Pubkey> for PublicKey {
    fn from(value: Pubkey) -> Self {
        Self(value.to_string())
    }
}

impl From<&str> for PublicKey {
    fn from(value: &str) -> Self {
        PublicKey::from_str(value).unwrap()
    }
}

impl From<Keypair> for PublicKey {
    fn from(value: Keypair) -> Self {
        Self(value.pubkey().to_string())
    }
}

#[derive(Debug)]
pub enum PrivateKeyError {
    Invalid(String),
}

impl fmt::Display for PrivateKeyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PrivateKeyError::Invalid(tag) => write!(f, "Invalid private key: {}", tag),
        }
    }
}

impl std::error::Error for PrivateKeyError {}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct PrivateKey(pub String);

impl PartialEq<&str> for PrivateKey {
    fn eq(&self, other: &&str) -> bool {
        self.0.as_str() == *other
    }
}

impl Display for PrivateKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PrivateKey {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

impl From<PrivateKey> for Keypair {
    fn from(value: PrivateKey) -> Self {
        Keypair::from_base58_string(value.0.as_str())
    }
}

impl From<Keypair> for PrivateKey {
    fn from(value: Keypair) -> Self {
        Self(value.to_base58_string())
    }
}

impl FromStr for PrivateKey {
    type Err = PrivateKeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let decoded = bs58::decode(s).into_vec().map_err(|_| PrivateKeyError::Invalid(s.to_string()))?;
        let key = Keypair::from_bytes(&decoded).map_err(|_| PrivateKeyError::Invalid(s.to_string()))?;
        Ok(PrivateKey(key.to_base58_string()))
    }
}

impl From<&str> for PrivateKey {
    fn from(value: &str) -> Self {
        PrivateKey::from_str(value).unwrap()
    }
}
