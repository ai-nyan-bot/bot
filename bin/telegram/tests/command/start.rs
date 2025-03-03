// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use common::repo::pool::PostgresConfig;
use common::ConfigValue;
use telegram::{schema, AppState, Config, MessageState, TelegramConfig};
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::dptree;
use teloxide_tests::{MockBot, MockMessageText};

#[tokio::test]
async fn test_start_command_creates_user_if_not_exists() {
    let config = Config {
        telegram: TelegramConfig {
            token: ConfigValue::Value("1234567890:QWERTYUIOPASDFGHJKLZXCVBNMQWERTYUIO".to_string()), // same as mockbot
            webapp_url: ConfigValue::Value("https://test.nyanbot.com".to_string()),
        },
        postgres: PostgresConfig::default(),
    };

    let state = AppState::testing(config).await;

    let bot = MockBot::new(MockMessageText::new().text("/start"), schema());
    bot.dependencies(dptree::deps![InMemStorage::<MessageState>::new(), state]);
    bot.dispatch().await;
    let responses = bot.get_responses();
    let message = responses.sent_messages.last().unwrap();
    // This is a regular teloxide::types::Message!
    // dbg!(&message);

    assert_eq!(message.text(), Some("
Welcome to AI Nyanbot!
  
You’ve just joined the purr-fect Telegram bot for navigating the Solana ecosystem. 
I’m Nyanbot, your AI-powered DEX screener to help you find your next 100x gem and dodge scams!
 
Trading Tips & Rules💡: 
Set Your Conditions:
🟢 Bonding Curve Progress: Aim for pump.fun tokens <50% bonded for early entries, or >80% for safer bets.  
 
Verify trades via official links only.
 
⚠\u{fe0f}Ad Disclaimer⚠\u{fe0f}: 
Heads up! We don’t control ads shown by Telegram here. 
Beware of fake airdrops, phishing links, or sketchy login pages.
Stick to https://nyan.bot for the real deal.  
 
Get more alpha: 
🌐Check out our website: https://nyan.bot 
🐥Follow us on X: https://x.com/AI_Nyanbot
💬Join our TG: @AI_Nyanbot
"));
}
