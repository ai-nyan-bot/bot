// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use telegram::{schema, AppState, MessageState};
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::dptree;
use teloxide_tests::{MockBot, MockMessageText};
use testing::run_test_with_pool;

#[test_log::test(tokio::test)]
#[ignore]
async fn test_start_command_creates_user_if_not_exists() {
    run_test_with_pool(|pool| async move{
        let state = AppState::testing(pool).await;

        let bot = MockBot::new(MockMessageText::new().text("/start"), schema());
        bot.dependencies(dptree::deps![InMemStorage::<MessageState>::new(), state]);
        bot.dispatch().await;
        let responses = bot.get_responses();
        let message = responses.sent_messages.last().unwrap();
        // This is a regular teloxide::types::Message!

        assert_eq!(message.text(),Some("Welcome to AI Nyanbot\\!\n\nYou\\’ve just joined the purr\\-fect Telegram bot for navigating the Solana ecosystem\\.\nI\\’m Nyanbot, your AI\\-powered DEX screener to help you find your next 100x gem and dodge scams\\!\n\nTrading Tips & Rules💡:\nSet Your Conditions:\n🟢 Bonding Curve Progress: Aim for pump\\.fun tokens <50% bonded for early entries, or \\>80% for safer bets\\.\n\nVerify swaps via official links only\\.\n\n⚠\u{fe0f}Ad Disclaimer⚠\u{fe0f}:\nHeads up\\! We don\\’t control ads shown by Telegram here\\.\nBeware of fake airdrops, phishing links, or sketchy login pages\\.\nStick to [nyan\\.bot](https://nyan\\.bot) for the real deal\\.\n\nGet more alpha:\n💬Join our TG: @AI\\_Nyanbot\n🌐Check out our [Website](https://nyan\\.bot)\n🐥Follow us on X: [AI\\_Nyanbot](https://x\\.com/AI\\_Nyanbot)"))
    }).await
}
