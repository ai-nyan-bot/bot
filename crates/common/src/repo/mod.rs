// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use sqlx::{Postgres, Transaction};

pub mod cache;
pub mod error;
pub mod pool;

pub type Tx<'a> = Transaction<'a, Postgres>;

pub type RepoResult<T> = Result<T, error::RepoError>;
