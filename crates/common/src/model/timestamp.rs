// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/blockworks-foundation/traffic (AGPL 3.0).
// Original AGPL 3 License Copyright (c) blockworks-foundation 2024.

use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sqlx::types::time::OffsetDateTime;
use time::error;

#[derive(Clone, Copy, Debug, PartialEq, Ord, PartialOrd, Eq, sqlx::Type)]
#[sqlx(transparent)]
pub struct Timestamp(pub OffsetDateTime);

impl Display for Timestamp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.0.microsecond() == 0 {
            write!(
                f,
                "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
                self.0.year(),
                self.0.month() as u8,
                self.0.day(),
                self.0.hour(),
                self.0.minute(),
                self.0.second()
            )
        } else {
            write!(
                f,
                "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:06}Z",
                self.0.year(),
                self.0.month() as u8,
                self.0.day(),
                self.0.hour(),
                self.0.minute(),
                self.0.second(),
                self.0.microsecond()
            )
        }
    }
}

impl Serialize for Timestamp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

impl<'de> Deserialize<'de> for Timestamp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;

        if let Ok(date_time) =
            OffsetDateTime::parse(&s, &time::format_description::well_known::Rfc3339)
        {
            return Ok(Timestamp(date_time));
        }

        if let Ok(timestamp) = s.parse::<i64>() {
            return Ok(Timestamp::from_epoch_second(timestamp).unwrap());
        }

        Err(serde::de::Error::custom(
            "Expected ISO8601 or Unix timestamp",
        ))
    }
}

impl Timestamp {
    pub fn now() -> Self {
        Self::from_epoch_micros(unix_micros_now() as i64)
    }

    pub fn from_offset_date_time(odt: OffsetDateTime) -> Self {
        Self(odt)
    }

    pub fn from_epoch_micros(epoch_micros: i64) -> Self {
        assert!(
            epoch_micros >= 1_000_000_000_000,
            "timestamp does not look like micros, was <{}>",
            epoch_micros
        );
        Self(
            OffsetDateTime::from_unix_timestamp_nanos(epoch_micros as i128 * 1_000)
                .expect("timestamp conversion"),
        )
    }

    pub fn from_epoch_second(epoch_second: i64) -> Result<Self, error::ComponentRange> {
        Ok(Self(OffsetDateTime::from_unix_timestamp(epoch_second)?))
    }

    pub fn from_epoch_millis(epoch_millis: i64) -> Self {
        assert!(
            epoch_millis >= 1_000_000_000_000,
            "timestamp does not look like millis, was <{}>",
            epoch_millis
        );
        Self(
            OffsetDateTime::from_unix_timestamp_nanos((epoch_millis * 1_000_000) as i128)
                .expect("timestamp conversion"),
        )
    }

    pub fn to_offset_date_time(&self) -> OffsetDateTime {
        self.0
    }

    pub fn to_epoch_seconds(&self) -> i64 {
        self.0.unix_timestamp()
    }

    pub fn to_epoch_micros(&self) -> i64 {
        (self.0.unix_timestamp_nanos() / 1_000) as i64
    }
}

pub fn unix_seconds_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}

pub fn unix_millis_now() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis()
}

pub fn unix_micros_now() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_micros()
}

impl PartialEq<str> for Timestamp {
    fn eq(&self, other: &str) -> bool {
        self.0.to_string().as_str().eq(other)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Ord, PartialOrd, Eq, sqlx::Type)]
#[sqlx(transparent, no_pg_array)]
pub struct CreatedAt(pub Timestamp);

impl CreatedAt {
    pub fn now() -> Self {
        Self(Timestamp::now())
    }
}

impl Deref for CreatedAt {
    type Target = Timestamp;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Ord, PartialOrd, Eq, sqlx::Type)]
#[sqlx(transparent, no_pg_array)]
pub struct UpdatedAt(pub Timestamp);

impl Deref for UpdatedAt {
    type Target = Timestamp;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl UpdatedAt {
    pub fn now() -> Self {
        Self(Timestamp::now())
    }

    pub fn age_sec(&self) -> i64 {
        (unix_seconds_now() as i64) - self.0.to_epoch_seconds()
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use super::*;

    #[test]
    fn serialize_without_microseconds() {
        let ts = Timestamp::from_epoch_micros(1_728_399_712_000_000);
        let result = json!(ts).to_string();
        assert_eq!(result, "\"2024-10-08T15:01:52Z\"");
    }

    #[test]
    fn serialize_with_microseconds() {
        let ts = Timestamp::from_epoch_micros(1_728_399_712_999_000);
        let result = json!(ts).to_string();
        assert_eq!(result, "\"2024-10-08T15:01:52.999000Z\"");
    }

    #[test]
    fn deserialize_unix_epoch_seconds() {
        let ts: Timestamp = serde_json::from_value(json!("1704067199")).unwrap();
        assert_eq!(ts.to_epoch_seconds(), 1704067199)
    }

    #[test]
    fn deserialize_iso8601() {
        let ts: Timestamp = serde_json::from_value(json!("2023-12-31T23:59:59Z")).unwrap();
        assert_eq!(ts.to_epoch_seconds(), 1704067199)
    }

    #[test]
    fn deserialize_iso8601_with_microseconds() {
        let ts: Timestamp = serde_json::from_value(json!("2023-12-31T23:59:59.123456Z")).unwrap();
        assert_eq!(ts.to_epoch_micros(), 1704067199123456)
    }

    #[test]
    fn deserialize_iso8601_with_timezone() {
        let ts: Timestamp = serde_json::from_value(json!("2023-12-31T23:59:59+08:00")).unwrap();
        assert_eq!(ts.to_epoch_seconds(), 1704038399)
    }
}
