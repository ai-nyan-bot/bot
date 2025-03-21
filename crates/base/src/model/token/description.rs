// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct Description(pub String);

impl From<&str> for Description {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for Description {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl PartialEq<&str> for Description {
    fn eq(&self, other: &&str) -> bool {
        self.0.as_str() == *other
    }
}

impl Description {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

impl Display for Description {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
