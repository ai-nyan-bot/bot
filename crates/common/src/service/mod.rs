// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::repo::error::RepoError;

#[derive(Debug, PartialEq)]
pub enum ServiceError {
    Conflict(String),
    Internal(String),
    NotFound(String),
}

impl ServiceError {
    pub fn conflict(s: impl Into<String>) -> Self {
        Self::Conflict(s.into())
    }

    pub fn internal(s: impl Into<String>) -> Self {
        Self::Internal(s.into())
    }

    pub fn not_found(s: impl Into<String>) -> Self {
        Self::NotFound(s.into())
    }
}

impl From<sqlx::Error> for ServiceError {
    fn from(value: sqlx::Error) -> Self {
        Self::Internal(value.to_string())
    }
}

impl From<RepoError> for ServiceError {
    fn from(value: RepoError) -> Self {
        Self::Internal(value.to_string())
    }
}

pub type ServiceResult<T> = Result<T, ServiceError>;
