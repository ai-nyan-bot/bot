// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use telegram::{schema, MessageState};
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::prelude::*;
use teloxide_tests::{MockBot, MockMessageText};

#[test_log::test(tokio::test)]
async fn test_help_command() {
    let bot = MockBot::new(MockMessageText::new().text("/help"), schema());
    bot.dependencies(dptree::deps![InMemStorage::<MessageState>::new()]);
    bot.dispatch().await;
    let responses = bot.get_responses();
    let message = responses.sent_messages.last().unwrap();
    // This is a regular teloxide::types::Message!
    assert_eq!(message.text(), Some("Getting help here"));
}
