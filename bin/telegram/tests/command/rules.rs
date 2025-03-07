// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use common::repo::pool::PostgresConfig;
use common::ConfigValue;
use telegram::{schema, AppState, Config, MessageState, TelegramConfig};
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::dptree;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, WebAppInfo};
use teloxide_tests::{MockBot, MockMessageText};
use url::Url;

#[test_log::test(tokio::test)]
async fn test_rules_command() {
    let config = Config {
        telegram: TelegramConfig {
            token: ConfigValue::Value("1234567890:QWERTYUIOPASDFGHJKLZXCVBNMQWERTYUIO".to_string()), // same as mockbot
            webapp_url: ConfigValue::Value("https://test.nyanbot.com".to_string()),
        },
        postgres: PostgresConfig::default(),
    };

    let state = AppState::testing(config).await;

    let bot = MockBot::new(MockMessageText::new().text("/rules"), schema());
    bot.dependencies(dptree::deps![InMemStorage::<MessageState>::new(), state]);
    bot.dispatch().await;
    let responses = bot.get_responses();
    let message = responses.sent_messages.last().unwrap();

    assert_eq!(message.text(), Some(r#"
Set your custom rules to screen for potential 100x gems!

For example, you can choose to filter for the following 
🟢Market Cap: Min/Max (e.g., $50K-500K). 
🟢Tx Count: Min (e.g., 50) for hype (>100) or sleepers (<20).  
🟢Honeypot: Exclude scams (Yes/No).   
🟢Bonding Curve: 0-50% (snipe) or 80-99% (safe). 
 
Tweak, save, and pounce on profits! 
Remember to DYOR!    
"#));
    assert_eq!(
        message.reply_markup(),
        Some(&InlineKeyboardMarkup::new(vec![vec![
            InlineKeyboardButton::web_app(
                "Open rules".to_string(),
                WebAppInfo {
                    url: Url::parse("https://telegram.nyan.bot/rules").unwrap(),
                },
            )
        ]]))
    );
}
