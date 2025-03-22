// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::command::CommandResult;
use crate::i18n::{Language, I18N};
use crate::markdown;
use crate::state::AppState;
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::Message;
use teloxide::requests::Requester;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, ParseMode, WebAppInfo};
use teloxide::Bot;
use url::Url;

pub(crate) async fn start(bot: Bot, msg: Message, state: AppState) -> CommandResult {
    let i18n = I18N::load(Language::En).await;

    let options = InlineKeyboardMarkup::new(vec![
        vec![InlineKeyboardButton::web_app(
            format!("{}", i18n.button_text_rules),
            WebAppInfo {
                url: Url::parse("https://telegram.nyan.bot/rules").unwrap(),
            },
        )], // vec![
            //     InlineKeyboardButton::callback(format!("↑ {}", i18n.button_text_send), "send"),
            //     InlineKeyboardButton::callback(format!("↓ {}", i18n.button_text_receive), "receive"),
            //     InlineKeyboardButton::web_app(
            //         "↑↓ Swap",
            //         WebAppInfo {
            //             url: Url::parse("https://nyanbot.com/wallet/swap").unwrap(),
            //         },
            //     ),
            // ],
            // vec![
            //     InlineKeyboardButton::callback(i18n.start, "send"),
            //     InlineKeyboardButton::callback(i18n.help, "receive"),
            // ],
            // vec![
            // InlineKeyboardButton::callback(format!("📊 {}", i18n.button_text_balance), "balance"),
            // InlineKeyboardButton::callback(format!("🏦 {}", i18n.button_text_wallet), "wallet"),
            // ],
    ]);

    if let Some(user) = msg.from {
        if !user.is_bot {
            let (_user, wallet, _created) = state
                .user_service()
                .get_or_create_telegram_user(user.id.0 as i64)
                .await
                .unwrap();

            let address = wallet.public_key.to_string();

            bot.send_message(msg.chat.id, markdown!(r#"
Welcome to AI Nyanbot!
  
You’ve just joined the purr-fect Telegram bot for navigating the Solana ecosystem. 
I’m Nyanbot, your trading companion to help you find your next 100x gem and dodge scams!
 
Your address: 
;`{address};` (click to copy)
 
Trading Tips & Rules💡: 
Set Your Conditions:
🟢 Bonding Curve Progress: Aim for pump.fun tokens <50% bonded for early entries, or >80% for safer bets.  
 
Verify trades via official links only.

⚠ Ad Disclaimer⚠ :
Heads up! We don’t control ads shown by Telegram here.
Beware of fake airdrops, phishing links, or sketchy login pages.
Stick to ;[nyan.bot;];(https://nyan.bot;) for the real deal.

Get more alpha: 
💬Join our TG: @AI_Nyanbot
🌐Check out our ;[Website;];(https://nyan.bot;) 
🐥Follow us on X: ;[AI_Nyanbot;];(https://x.com/AI_Nyanbot;)
"#))
            .parse_mode(ParseMode::MarkdownV2)
            .reply_markup(options)
            .await?;
        }
    }

    Ok(())
}
