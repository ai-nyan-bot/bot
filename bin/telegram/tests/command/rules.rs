// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use telegram::{schema, AppState, MessageState};
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::dptree;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, WebAppInfo};
use teloxide_tests::{MockBot, MockMessageText};
use testing::run_test_with_pool;
use url::Url;

#[test_log::test(tokio::test)]
#[ignore]
async fn test_rules_command() {
    run_test_with_pool(|pool| async move {
        let state = AppState::testing(pool).await;

        let bot = MockBot::new(MockMessageText::new().text("/rules"), schema());
        bot.dependencies(dptree::deps![InMemStorage::<MessageState>::new(), state]);
        bot.dispatch().await;
        let responses = bot.get_responses();
        let message = responses.sent_messages.last().unwrap();

        assert_eq!(
            message.text(),
            Some(
                r#"
Set your custom rules to screen for potential 100x gems!

For example, you can choose to filter for the following
🟢Market Cap: Min/Max (e.g., $50K-500K).
🟢Tx Count: Min (e.g., 50) for hype (>100) or sleepers (<20).
🟢Honeypot: Exclude scams (Yes/No).
🟢Bonding Curve: 0-50% (snipe) or 80-99% (safe).

Tweak, save, and pounce on profits!
Remember to DYOR!
"#
            )
        );
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
    })
    .await
}
