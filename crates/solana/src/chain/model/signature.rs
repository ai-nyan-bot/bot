// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct Signature(pub String);

impl Display for Signature {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Signature {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

impl From<String> for Signature {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for Signature {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl PartialEq<&str> for Signature {
    fn eq(&self, other: &&str) -> bool {
        self.0.as_str() == *other
    }
}
