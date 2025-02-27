// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct Uri(pub String);

impl From<String> for Uri {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for Uri {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl PartialEq<&str> for Uri {
    fn eq(&self, other: &&str) -> bool {
        self.0.as_str() == *other
    }
}

impl Uri {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

impl Display for Uri {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
