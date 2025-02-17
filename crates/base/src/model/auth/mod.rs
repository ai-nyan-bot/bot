// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::UserId;
use common::model::{CreatedAt, UpdatedAt};
use serde::{Deserialize, Serialize};

#[derive(Eq, Hash, Copy, Clone, Debug, PartialEq, sqlx::Type)]
#[sqlx(transparent)]
pub struct AuthId(pub i32);

impl PartialEq<i32> for AuthId {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl From<i32> for AuthId {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

#[derive(Eq, Hash, Clone, Debug, PartialEq, Deserialize, Serialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct AuthToken(pub String);

impl From<&str> for AuthToken {
    fn from(value: &str) -> Self {
        AuthToken(value.to_string())
    }
}

impl From<String> for AuthToken {
    fn from(value: String) -> Self {
        AuthToken(value)
    }
}

impl PartialEq<&str> for AuthToken {
    fn eq(&self, other: &&str) -> bool {
        self.0.as_str() == *other
    }
}

#[derive(Clone, Debug)]
pub struct AuthenticatedUser {
    pub id: UserId,
}

#[derive(Clone, Debug)]
pub struct Auth {
    pub id: AuthId,
    pub user: AuthenticatedUser,
    pub token: AuthToken,
    pub created_at: CreatedAt,
    pub updated_at: UpdatedAt,
}
