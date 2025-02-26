// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::repo::error::RepoError;
use log::error;
use std::fmt::{Display, Formatter};

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

impl From<serde_json::Error> for ServiceError {
	fn from(value: serde_json::Error) -> Self {
		error!("{value:#?}");
		ServiceError::Internal(format!("serde error: {}", value))
	}
}

impl Display for ServiceError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			ServiceError::Conflict(err) => f.write_fmt(format_args!("conflict error: {err}")),
			ServiceError::Internal(err) => f.write_fmt(format_args!("internal error: {err}")),
			ServiceError::NotFound(err) => f.write_fmt(format_args!("not found error: {err}")),
		}
	}
}

impl std::error::Error for ServiceError {}

pub type ServiceResult<T> = Result<T, ServiceError>;
