// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::repo::RepoResult;
use log::error;
use sqlx::postgres::PgDatabaseError;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub enum RepoError {
    AlreadyExists,
    ConstraintViolation,
    ForeignKeyViolation,
    NotFound,
    Serde,
    TransactionAborted,
}

impl Display for RepoError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RepoError::AlreadyExists => f.write_str("already exists"),
            RepoError::ConstraintViolation => f.write_str("constraint violation"),
            RepoError::ForeignKeyViolation => f.write_str("foreign key violation"),
            RepoError::NotFound => f.write_str("not found"),
            RepoError::Serde => f.write_str("serde error"),
            RepoError::TransactionAborted => f.write_str("transaction aborted"),
        }
    }
}

impl std::error::Error for RepoError {}

impl From<sqlx::Error> for RepoError {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::Configuration(_) => unimplemented!(),
            sqlx::Error::Database(error) => {
                error!("{error:#?}");
                let error = error.downcast_ref::<PgDatabaseError>();
                if error.code() == "23505" {
                    return RepoError::AlreadyExists;
                }
                if error.code() == "42601" {
                    panic!("{error:#?}")
                }
                if error.code() == "25P02" {
                    return RepoError::TransactionAborted;
                }
                if error.code() == "23503" {
                    return RepoError::ForeignKeyViolation;
                }
                if error.code() == "23502" {
                    return RepoError::ConstraintViolation;
                }
                if error.code() == "42804"{
                    return RepoError::Serde;
                }
                unimplemented!("{error:#?}")
            }
            sqlx::Error::Io(_) => unimplemented!(),
            sqlx::Error::Tls(_) => unimplemented!(),
            sqlx::Error::Protocol(_) => unimplemented!(),
            sqlx::Error::RowNotFound => RepoError::NotFound,
            sqlx::Error::TypeNotFound { .. } => unimplemented!(),
            sqlx::Error::ColumnIndexOutOfBounds { .. } => unimplemented!(),
            sqlx::Error::ColumnNotFound(_) => unimplemented!(),
            sqlx::Error::ColumnDecode { .. } => unimplemented!(),
            sqlx::Error::Encode(_) => unimplemented!(),
            sqlx::Error::Decode(_) => unimplemented!(),
            sqlx::Error::AnyDriverError(_) => unimplemented!(),
            sqlx::Error::PoolTimedOut => unimplemented!(),
            sqlx::Error::PoolClosed => unimplemented!(),
            sqlx::Error::WorkerCrashed => unimplemented!(),
            sqlx::Error::Migrate(_) => unimplemented!(),
            _ => unimplemented!(),
        }
    }
}

impl From<serde_json::Error> for RepoError {
    fn from(value: serde_json::Error) -> Self {
        error!("{value:#?}");
        RepoError::Serde
    }
}
