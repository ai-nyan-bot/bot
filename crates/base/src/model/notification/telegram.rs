// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::Value;
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TelegramActionButtonConfig {
	None,
	Buy {
		value: Value
	},
	Sell {
		value: Value
	},
}

#[cfg(test)]
mod tests {
	use crate::model::notification::telegram::TelegramActionButtonConfig;
	use crate::model::Value;

	#[test]
	fn serde_none() {
		let serialised = serde_json::to_string(&TelegramActionButtonConfig::None).unwrap();
		assert_eq!(serialised, r#"{"action":"NONE"}"#);
		assert_eq!(serde_json::from_str::<TelegramActionButtonConfig>(serialised.as_str()).unwrap(), TelegramActionButtonConfig::None);
	}

	#[test]
	fn serde_buy() {
		let serialised = serde_json::to_string(&TelegramActionButtonConfig::Buy { value: Value::sol(23.4) }).unwrap();
		assert_eq!(serialised, r#"{"action":"BUY","value":{"type":"SOL","value":23.4}}"#);
		assert_eq!(serde_json::from_str::<TelegramActionButtonConfig>(serialised.as_str()).unwrap(), TelegramActionButtonConfig::Buy { value: Value::sol(23.4) });
	}

	#[test]
	fn serde_sell() {
		let serialised = serde_json::to_string(&TelegramActionButtonConfig::Sell { value: Value::percent(51.2) }).unwrap();
		assert_eq!(serialised, r#"{"action":"SELL","value":{"type":"PERCENT","value":51.2}}"#);
		assert_eq!(serde_json::from_str::<TelegramActionButtonConfig>(serialised.as_str()).unwrap(), TelegramActionButtonConfig::Sell { value: Value::percent(51.2) });
	}
}