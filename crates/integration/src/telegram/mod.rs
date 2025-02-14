// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

//! A rust implementation of the verification of Telegram Login requests.
//!
//! Based on the example from the [Telegram docs](https://core.telegram.org/widgets/login#checking-authorization).

extern crate serde;

use log::trace;
use ring::hmac;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use urlencoding::decode;

#[derive(Clone, Debug, PartialEq)]
pub enum TelegramLoginError {
    InvalidAuthDate,
    InvalidHash,
    InvalidQueryId,
    InvalidSignature,
    InvalidUser,
    VerificationFailed,
}

impl Display for TelegramLoginError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TelegramLoginError::InvalidAuthDate => f.write_str("invalid auth date"),
            TelegramLoginError::InvalidHash => f.write_str("invalid hash"),
            TelegramLoginError::InvalidQueryId => f.write_str("invalid query id"),
            TelegramLoginError::InvalidSignature => f.write_str("invalid signature"),
            TelegramLoginError::InvalidUser => f.write_str("invalid user"),
            TelegramLoginError::VerificationFailed => f.write_str("verification failed"),
        }
    }
}

impl std::error::Error for TelegramLoginError {}

///
/// The Telegram Login data object that is returned from the Telegram Auth endpoint.
///
#[derive(Deserialize, Clone, Debug)]
pub struct TelegramLogin {
    auth_date: String,
    hash: String,
    query_id: String,
    signature: String,
    pub user: TelegramUser,
    user_json: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TelegramUser {
    pub id: i64,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub language_code: Option<String>,
    pub allows_write_to_pm: Option<bool>,
    pub photo_url: Option<String>,
}

impl TelegramLogin {
    pub fn from_query_string(query: String) -> Result<Self, TelegramLoginError> {
        let decoded_query = Self::decode_query(query.as_str());

        let user_json = decoded_query.get("user").ok_or(TelegramLoginError::InvalidUser)?.as_str();

        let user: TelegramUser = serde_json::from_str(user_json).map_err(|_| TelegramLoginError::InvalidUser)?;

        let hash = decoded_query.get("hash").ok_or(TelegramLoginError::InvalidHash)?.clone();

        let auth_date = decoded_query.get("auth_date").ok_or(TelegramLoginError::InvalidAuthDate)?.to_string();

        let signature = decoded_query.get("signature").ok_or(TelegramLoginError::InvalidSignature)?.clone();

        let query_id = decoded_query.get("query_id").ok_or(TelegramLoginError::InvalidQueryId)?.clone();

        Ok(TelegramLogin {
            auth_date,
            hash,
            query_id,
            signature,
            user,
            user_json: user_json.to_string(),
        })
    }

    fn decode_query(query: &str) -> HashMap<String, String> {
        query
            .split('&')
            .filter_map(|pair| {
                let mut parts = pair.split('=');
                let key = parts.next()?.to_string();
                let value = parts.next().map(|v| decode(v.replace("%5C%2F", r#"\/"#).as_str()).unwrap().to_string())?;
                Some((key, value))
            })
            .collect()
    }
}

/// Verifies the Telegram user using the provided data and bot token.
pub fn verify_telegram_user(bot_token: &str, login: TelegramLogin) -> Result<(), TelegramLoginError> {
    let data_string = generate_data_check_string(&login);

    let secret_key = generate_secret_key(bot_token);
    let signature = generate_hmac_signature(&secret_key, data_string);
    let generated_hash = hex::encode(signature);

    trace!("generated hash: {}", generated_hash);
    if generated_hash == login.hash {
        Ok(())
    } else {
        Err(TelegramLoginError::VerificationFailed)
    }
}

fn generate_secret_key(bot_token: &str) -> Vec<u8> {
    let secret = b"WebAppData";
    let key = hmac::Key::new(hmac::HMAC_SHA256, secret);
    let tag = hmac::sign(&key, bot_token.as_bytes());
    tag.as_ref().to_vec()
}

/// Generates an HMAC-SHA256 signature using the provided bot token.
fn generate_hmac_signature(secret_key: &[u8], data: String) -> Vec<u8> {
    let key = hmac::Key::new(hmac::HMAC_SHA256, secret_key);
    let tag = hmac::sign(&key, data.as_bytes());
    tag.as_ref().to_vec()
}

/// Converts the Struct object received from Telegram into the data_check_string as required
/// in the verification process.
fn generate_data_check_string(login: &TelegramLogin) -> String {
    struct KeyValue {
        key: String,
        value: String,
    }

    fn field(key: impl Into<String>, value: impl Into<String>) -> KeyValue {
        KeyValue {
            key: key.into(),
            value: value.into(),
        }
    }

    // Put the key, value pairs in order
    let fields = vec![
        field("auth_date", login.auth_date.as_str()),
        field("query_id", login.query_id.as_str()),
        field("signature", login.signature.as_str()),
        field("user", login.user_json.as_str()),
    ];

    let mut result = fields.into_iter().fold("".to_string(), |acc, f| format!("{}{}={}\n", acc, f.key, f.value));

    // Remove the final "\n" before returning the result
    result.pop();
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::telegram::TelegramLoginError::VerificationFailed;

    #[test]
    fn ok() {
        let login = TelegramLogin::from_query_string(
            "query_id=AAGqmHAaAwAAAKqYcBo0s6pa&user=%7B%22id%22%3A6886037674%2C%22first_name%22%3A%22Dee%22%2C%22last_name%22%3A%22Dee%22%2C%22username%22%3A%22deedee1337%22%2C%22language_code%22%3A%22en%22%2C%22allows_write_to_pm%22%3Atrue%2C%22photo_url%22%3A%22https%3A%5C%2F%5C%2Ft.me%5C%2Fi%5C%2Fuserpic%5C%2F320%5C%2FETDX5qwxULtIfWuOA5pSNI9hzRzil7XA4Tnx5NqNypDqBM6OTFA_li21aEd-wI4r.svg%22%7D&auth_date=1738054894&signature=HkX8UoSMG7njc50r3GDQ95XNHNqc6E0E95GGVsYbqMDObyIUL6omfTa_gkBFhXzbg8Z6KSX07Fzzd9R4WwAPAg&hash=aa7bdf3fff1121862c7a118ba15ddeca6d7299a9f9e882ad824010edceb67e6a".to_string()
        ).unwrap();

        assert_eq!(login.query_id, "AAGqmHAaAwAAAKqYcBo0s6pa");
        assert_eq!(
            login.signature,
            "HkX8UoSMG7njc50r3GDQ95XNHNqc6E0E95GGVsYbqMDObyIUL6omfTa_gkBFhXzbg8Z6KSX07Fzzd9R4WwAPAg"
        );
        assert_eq!(login.user.id, 6886037674);
        assert_eq!(login.user.username, Some("deedee1337".to_string()));
        assert_eq!(login.user.first_name, Some("Dee".to_string()));
        assert_eq!(login.user.last_name, Some("Dee".to_string()));
        assert_eq!(
            login.user.photo_url,
            Some("https://t.me/i/userpic/320/ETDX5qwxULtIfWuOA5pSNI9hzRzil7XA4Tnx5NqNypDqBM6OTFA_li21aEd-wI4r.svg".to_string())
        );
        assert_eq!(login.auth_date, "1738054894");
        assert_eq!(login.hash, "aa7bdf3fff1121862c7a118ba15ddeca6d7299a9f9e882ad824010edceb67e6a".to_string());
        assert_eq!(verify_telegram_user("7212584558:AAFyZo37lw4VPHPIdbynqKtMacHPwF0uMGE", login), Ok(()));
    }

    #[test]
    fn hash_does_not_match() {
        let login = TelegramLogin::from_query_string(
            "query_id=AAGqmHAaAwAAAKqYcBo0s6pa&user=%7B%22id%22%3A6886037674%2C%22first_name%22%3A%22Dee%22%2C%22last_name%22%3A%22Dee%22%2C%22username%22%3A%22deedee1337%22%2C%22language_code%22%3A%22en%22%2C%22allows_write_to_pm%22%3Atrue%2C%22photo_url%22%3A%22https%3A%5C%2F%5C%2Ft.me%5C%2Fi%5C%2Fuserpic%5C%2F320%5C%2FETDX5qwxULtIfWuOA5pSNI9hzRzil7XA4Tnx5NqNypDqBM6OTFA_li21aEd-wI4r.svg%22%7D&auth_date=1738054894&signature=HkX8UoSMG7njc50r3GDQ95XNHNqc6E0E95GGVsYbqMDObyIUL6omfTa_gkBFhXzbg8Z6KSX07Fzzd9R4WwAPAg&hash=DOES_NOT_MATCH_EVER".to_string()
        ).unwrap();

        assert_eq!(
            verify_telegram_user("7212584558:AAFyZo37lw4VPHPIdbynqKtMacHPwF0uMGE", login),
            Err(VerificationFailed)
        );
    }

    #[test]
    fn signature_does_not_match_bot_token() {
        let login = TelegramLogin::from_query_string(
            "query_id=AAGqmHAaAwAAAKqYcBo0s6pa&user=%7B%22id%22%3A6886037674%2C%22first_name%22%3A%22Dee%22%2C%22last_name%22%3A%22Dee%22%2C%22username%22%3A%22deedee1337%22%2C%22language_code%22%3A%22en%22%2C%22allows_write_to_pm%22%3Atrue%2C%22photo_url%22%3A%22https%3A%5C%2F%5C%2Ft.me%5C%2Fi%5C%2Fuserpic%5C%2F320%5C%2FETDX5qwxULtIfWuOA5pSNI9hzRzil7XA4Tnx5NqNypDqBM6OTFA_li21aEd-wI4r.svg%22%7D&auth_date=1738054894&signature=HkX8UoSMG7njc50r3GDQ95XNHNqc6E0E95GGVsYbqMDObyIUL6omfTa_gkBFhXzbg8Z6KSX07Fzzd9R4WwAPAg&hash=aa7bdf3fff1121862c7a118ba15ddeca6d7299a9f9e882ad824010edceb67e6a".to_string()
        ).unwrap();

        assert_eq!(verify_telegram_user("0987654321:AAAAAAAAAAAAAAAAA", login), Err(VerificationFailed));
    }

    #[test]
    fn missing_query_id() {
        let result = TelegramLogin::from_query_string(
            "user=%7B%22id%22%3A6886037674%2C%22first_name%22%3A%22Dee%22%2C%22last_name%22%3A%22Dee%22%2C%22username%22%3A%22deedee1337%22%2C%22language_code%22%3A%22en%22%2C%22allows_write_to_pm%22%3Atrue%2C%22photo_url%22%3A%22https%3A%5C%2F%5C%2Ft.me%5C%2Fi%5C%2Fuserpic%5C%2F320%5C%2FETDX5qwxULtIfWuOA5pSNI9hzRzil7XA4Tnx5NqNypDqBM6OTFA_li21aEd-wI4r.svg%22%7D&auth_date=1738054894&signature=HkX8UoSMG7njc50r3GDQ95XNHNqc6E0E95GGVsYbqMDObyIUL6omfTa_gkBFhXzbg8Z6KSX07Fzzd9R4WwAPAg&hash=aa7bdf3fff1121862c7a118ba15ddeca6d7299a9f9e882ad824010edceb67e6a".to_string()
        );

        assert_eq!(result.err().unwrap(), TelegramLoginError::InvalidQueryId);
    }

    #[test]
    fn missing_user() {
        let result = TelegramLogin::from_query_string(
            "query_id=AAGqmHAaAwAAAKqYcBo0s6pa&&auth_date=1738054894&signature=HkX8UoSMG7njc50r3GDQ95XNHNqc6E0E95GGVsYbqMDObyIUL6omfTa_gkBFhXzbg8Z6KSX07Fzzd9R4WwAPAg&hash=aa7bdf3fff1121862c7a118ba15ddeca6d7299a9f9e882ad824010edceb67e6a".to_string()
        );

        assert_eq!(result.err().unwrap(), TelegramLoginError::InvalidUser);
    }

    #[test]
    fn missing_auth_date() {
        let result = TelegramLogin::from_query_string(
            "query_id=AAGqmHAaAwAAAKqYcBo0s6pa&user=%7B%22id%22%3A6886037674%2C%22first_name%22%3A%22Dee%22%2C%22last_name%22%3A%22Dee%22%2C%22username%22%3A%22deedee1337%22%2C%22language_code%22%3A%22en%22%2C%22allows_write_to_pm%22%3Atrue%2C%22photo_url%22%3A%22https%3A%5C%2F%5C%2Ft.me%5C%2Fi%5C%2Fuserpic%5C%2F320%5C%2FETDX5qwxULtIfWuOA5pSNI9hzRzil7XA4Tnx5NqNypDqBM6OTFA_li21aEd-wI4r.svg%22%7D&signature=HkX8UoSMG7njc50r3GDQ95XNHNqc6E0E95GGVsYbqMDObyIUL6omfTa_gkBFhXzbg8Z6KSX07Fzzd9R4WwAPAg&hash=aa7bdf3fff1121862c7a118ba15ddeca6d7299a9f9e882ad824010edceb67e6a".to_string()
        );

        assert_eq!(result.err().unwrap(), TelegramLoginError::InvalidAuthDate);
    }

    #[test]
    fn missing_signature() {
        let result = TelegramLogin::from_query_string(
            "query_id=AAGqmHAaAwAAAKqYcBo0s6pa&user=%7B%22id%22%3A6886037674%2C%22first_name%22%3A%22Dee%22%2C%22last_name%22%3A%22Dee%22%2C%22username%22%3A%22deedee1337%22%2C%22language_code%22%3A%22en%22%2C%22allows_write_to_pm%22%3Atrue%2C%22photo_url%22%3A%22https%3A%5C%2F%5C%2Ft.me%5C%2Fi%5C%2Fuserpic%5C%2F320%5C%2FETDX5qwxULtIfWuOA5pSNI9hzRzil7XA4Tnx5NqNypDqBM6OTFA_li21aEd-wI4r.svg%22%7D&auth_date=1738054894&hash=aa7bdf3fff1121862c7a118ba15ddeca6d7299a9f9e882ad824010edceb67e6a".to_string()
        );

        assert_eq!(result.err().unwrap(), TelegramLoginError::InvalidSignature);
    }

    #[test]
    fn missing_hash() {
        let result = TelegramLogin::from_query_string(
            "query_id=AAGqmHAaAwAAAKqYcBo0s6pa&user=%7B%22id%22%3A6886037674%2C%22first_name%22%3A%22Dee%22%2C%22last_name%22%3A%22Dee%22%2C%22username%22%3A%22deedee1337%22%2C%22language_code%22%3A%22en%22%2C%22allows_write_to_pm%22%3Atrue%2C%22photo_url%22%3A%22https%3A%5C%2F%5C%2Ft.me%5C%2Fi%5C%2Fuserpic%5C%2F320%5C%2FETDX5qwxULtIfWuOA5pSNI9hzRzil7XA4Tnx5NqNypDqBM6OTFA_li21aEd-wI4r.svg%22%7D&auth_date=1738054894&signature=HkX8UoSMG7njc50r3GDQ95XNHNqc6E0E95GGVsYbqMDObyIUL6omfTa_gkBFhXzbg8Z6KSX07Fzzd9R4WwAPAg".to_string()
        );

        assert_eq!(result.err().unwrap(), TelegramLoginError::InvalidHash);
    }
}
