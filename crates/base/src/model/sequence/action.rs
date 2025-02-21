// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::{Sequence, TelegramActionButtonConfig};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Action {
	AndThen { action: Box<Action>, sequence: Box<Sequence> },
	Buy,
	NotifyTelegram {
		buttons: Vec<TelegramActionButtonConfig>
	},
	Sell,
}
