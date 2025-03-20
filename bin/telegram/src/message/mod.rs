// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::message::MessageError::UnableToSend;
use std::fmt::{Display, Formatter};
pub(crate) use summary::*;
use teloxide::{ApiError, RequestError};
use MessageError::Unknown;

mod summary;

#[derive(Debug, Clone)]
pub enum MessageError {
    UnableToSend(String),
    Unknown(String),
}

impl Display for MessageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            UnableToSend(msg) => f.write_fmt(format_args!("unable to send message: {}", msg)),
            Unknown(msg) => f.write_fmt(format_args!("failed to send message: {}", msg)),
        }
    }
}

impl std::error::Error for MessageError {}

impl From<RequestError> for MessageError {
    fn from(value: RequestError) -> Self {
        match &value {
            RequestError::Api(err) => match err {
                ApiError::BotBlocked
                | ApiError::InvalidToken
                | ApiError::MessageNotModified
                | ApiError::BotKicked
                | ApiError::BotKickedFromSupergroup
                | ApiError::UserDeactivated
                | ApiError::CantTalkWithBots => UnableToSend(err.to_string()),

                _ => Unknown(value.to_string()),
            },
            _ => Unknown(value.to_string()),
        }
    }
}

pub type MessageResult<T> = Result<T, MessageError>;
