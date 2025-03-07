// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use bigdecimal::{BigDecimal, ToPrimitive};
use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, Type)]
#[sqlx(transparent)]
pub struct Volume(pub BigDecimal);

impl Display for Volume {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let num = self.0.clone();
        let mut suffix = "";

        let billion = BigDecimal::from(1_000_000_000u64);
        let million = BigDecimal::from(1_000_000u64);
        let thousand = BigDecimal::from(1_000u64);

        let formatted = if num >= billion {
            suffix = "B";
            (num / &billion).to_f64().map(|x| format!("{:.1}", x))
        } else if num >= million {
            suffix = "M";
            (num / &million).to_f64().map(|x| format!("{:.1}", x))
        } else if num >= thousand.clone() * 10u64 {
            suffix = "K";
            (num / &thousand).to_f64().map(|x| format!("{:.1}", x))
        } else {
            num.to_f64().map(|x| format!("{:.1}", x))
        };

        let formatted = formatted.unwrap_or_else(|| "0".to_string());

        let cleaned = if formatted.ends_with(".0") {
            formatted[..formatted.len() - 2].to_string()
        } else {
            formatted
        };

        let mut result = cleaned.chars().take(5).collect::<String>();
        result.push_str(suffix);
        f.write_str(&result)
    }
}

impl From<i32> for Volume {
    fn from(value: i32) -> Self {
        Self(BigDecimal::from(value))
    }
}

impl From<i64> for Volume {
    fn from(value: i64) -> Self {
        Self(BigDecimal::from(value))
    }
}

impl From<u64> for Volume {
    fn from(value: u64) -> Self {
        Self(BigDecimal::from(value))
    }
}

impl PartialEq<i32> for Volume {
    fn eq(&self, other: &i32) -> bool {
        Self(BigDecimal::from(*other)).0.eq(&self.0)
    }
}

impl PartialOrd<i32> for Volume {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.0.partial_cmp(&BigDecimal::from(*other))
    }
}

impl PartialEq<BigDecimal> for Volume {
    fn eq(&self, other: &BigDecimal) -> bool {
        self.0.eq(other)
    }
}

impl PartialOrd<BigDecimal> for Volume {
    fn partial_cmp(&self, other: &BigDecimal) -> Option<Ordering> {
        self.0.partial_cmp(&other)
    }
}

impl PartialEq<&str> for Volume {
    fn eq(&self, other: &&str) -> bool {
        self.eq(&BigDecimal::from_str(other).unwrap())
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, Type)]
#[sqlx(transparent)]
pub struct VolumeUsd(pub BigDecimal);

impl Display for VolumeUsd {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let num = self.0.clone();
        let mut suffix = "";

        let billion = BigDecimal::from(1_000_000_000u64);
        let million = BigDecimal::from(1_000_000u64);
        let thousand = BigDecimal::from(1_000u64);

        let formatted = if num >= billion {
            suffix = "B";
            (num / &billion).to_f64().map(|x| format!("{:.1}", x))
        } else if num >= million {
            suffix = "M";
            (num / &million).to_f64().map(|x| format!("{:.1}", x))
        } else if num >= thousand.clone() * 10u64 {
            suffix = "K";
            (num / &thousand).to_f64().map(|x| format!("{:.1}", x))
        } else {
            num.to_f64().map(|x| format!("{:.1}", x))
        };

        let formatted = formatted.unwrap_or_else(|| "0".to_string());

        let cleaned = if formatted.ends_with(".0") {
            formatted[..formatted.len() - 2].to_string()
        } else {
            formatted
        };

        let mut result = cleaned.chars().take(5).collect::<String>();
        result.push_str(suffix);
        f.write_fmt(format_args!("$ {result}"))
    }
}

impl From<i32> for VolumeUsd {
    fn from(value: i32) -> Self {
        Self(BigDecimal::from(value))
    }
}

impl From<i64> for VolumeUsd {
    fn from(value: i64) -> Self {
        Self(BigDecimal::from(value))
    }
}

impl From<u64> for VolumeUsd {
    fn from(value: u64) -> Self {
        Self(BigDecimal::from(value))
    }
}

impl PartialEq<i32> for VolumeUsd {
    fn eq(&self, other: &i32) -> bool {
        Self(BigDecimal::from(*other)).0.eq(&self.0)
    }
}

impl PartialOrd<i32> for VolumeUsd {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.0.partial_cmp(&BigDecimal::from(*other))
    }
}

impl PartialEq<BigDecimal> for VolumeUsd {
    fn eq(&self, other: &BigDecimal) -> bool {
        self.0.eq(other)
    }
}

impl PartialOrd<BigDecimal> for VolumeUsd {
    fn partial_cmp(&self, other: &BigDecimal) -> Option<Ordering> {
        self.0.partial_cmp(&other)
    }
}

impl PartialEq<&str> for VolumeUsd {
    fn eq(&self, other: &&str) -> bool {
        self.eq(&BigDecimal::from_str(other).unwrap())
    }
}
