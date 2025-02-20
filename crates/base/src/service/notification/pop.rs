// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::Notification;
use crate::service::NotificationService;
use common::model::Limit;
use common::repo::error::RepoError;
use common::service::{ServiceError, ServiceResult};
use log::error;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::future::Future;

#[derive(Debug)]
pub struct NotificationError(pub String);

impl Display for NotificationError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!("notification error: {}", self.0))
	}
}

impl Error for NotificationError {}

impl From<RepoError> for NotificationError {
	fn from(value: RepoError) -> Self {
		Self(value.to_string())
	}
}

impl From<ServiceError> for NotificationError {
	fn from(value: ServiceError) -> Self {
		Self(value.to_string())
	}
}

pub type NotificationResult<T> = Result<T, NotificationError>;

impl NotificationService {
	pub async fn pop<T, TFut, R>(&self, limit: impl Into<Limit>, mut fun: T) -> ServiceResult<Box<[R]>>
	where
		T: FnMut(Notification) -> TFut + Send + 'static,
		TFut: Future<Output=NotificationResult<R>> + Send,
	{
		let limit = limit.into();
		let mut tx = self.pool.begin().await?;

		let notifications = self.repo.pop(&mut tx, limit).await?;
		let mut result = Vec::with_capacity(notifications.len());

		for notification in notifications {
			match fun(notification).await {
				Ok(r) => result.push(r),
				Err(err) => {
					error!("{err}:#?");
					tx.rollback().await?;
					return Err(ServiceError::Internal(err.to_string()));
				}
			}
		}

		tx.commit().await?;
		Ok(result.into_boxed_slice())
	}
}
