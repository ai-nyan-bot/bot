// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use telegram::{schema, AppState, MessageState};
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::dptree;
use teloxide_tests::{MockBot, MockMessageText};
use testing::run_test_with_pool;

#[test_log::test(tokio::test)]
async fn test_start_command_creates_user_if_not_exists() {
    run_test_with_pool(|pool| async move{
        let state = AppState::testing(pool).await;

        let bot = MockBot::new(MockMessageText::new().text("/start"), schema());
        bot.dependencies(dptree::deps![InMemStorage::<MessageState>::new(), state.clone()]);
        bot.dispatch().await;
        let responses = bot.get_responses();
        let message = responses.sent_messages.last().unwrap();

        let wallet = state.user_service().get_wallet(3).await.unwrap();
        let address = wallet.public_key.to_string();

        assert_eq!(message.text(),Some(format!(r#"Welcome to AI Nyanbot\!

You’ve just joined the purr\-fect Telegram bot for navigating the Solana ecosystem\.
I’m Nyanbot, your trading companion to help you find your next 100x gem and dodge scams\!

Your address:
`{address}` \(click to copy\)

Trading Tips & Rules💡:
Set Your Conditions:
🟢 Bonding Curve Progress: Aim for pump\.fun tokens <50% bonded for early entries, or \>80% for safer bets\.

Verify trades via official links only\.

⚠ Ad Disclaimer⚠ :
Heads up\! We don’t control ads shown by Telegram here\.
Beware of fake airdrops, phishing links, or sketchy login pages\.
Stick to [nyan\.bot](https://nyan\.bot) for the real deal\.

Get more alpha:
💬Join our TG: @AI\_Nyanbot
🌐Check out our [Website](https://nyan\.bot)
🐥Follow us on X: [AI\_Nyanbot](https://x\.com/AI\_Nyanbot)"#).as_str()))
    }).await
}
