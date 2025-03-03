// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::i18n::{Language, I18N};
use crate::state::AppState;
use crate::{markdown, HandlerResult};
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::Message;
use teloxide::requests::Requester;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, ParseMode, WebAppInfo};
use teloxide::Bot;
use url::Url;

pub(crate) async fn start(bot: Bot, msg: Message, state: AppState) -> HandlerResult {
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
            // println!("User {user:#?} started");
            // println!("{:#?}", state.config);
            let (_user, _wallet, _created) = state
                .user_service()
                .get_or_create_telegram_user(user.id.0)
                .await
                .unwrap();
        }
    }

    //     bot.send_message(msg.chat.id, r#"
    // Welcome to AI Nyanbot!
    // You’ve just joined the purr-fect Telegram bot for navigating the Solana ecosystem. I’m Nyanbot, your AI-powered DEX screener to help you find your next 100x gem and dodge scams!
    //
    // Your Wallet:
    // Solana · 🅴
    // Your Wallet Address: [Insert User Wallet Here] (Tap to copy)
    // Balance: [Insert SOL Balance] SOL ($[Insert USD Value])
    // —
    // Hit the Refresh button to update your balance anytime!
    //
    // 🚀Top Movers of the Day:
    // [Dynamic List e.g., "1. MEOW (+15%) | 2. PURR (+10%) | 3. CATNIP (+8%)"]
    //
    // 💡Trading Tips & Rules:
    // Set Your Conditions:
    // 🟢 Market Cap Filter: Set a min/max market cap (e.g., $10-100k) to spot hidden gems or avoid pumps.
    // 🟢 Bonding Curve Progress: Aim for pump.fun tokens <50% bonded for early entries, or >80% for safer bets.
    // 🟢 Volume Spike Filter: Look for tokens that pumped >$30k over last 6 hours.
    // 🟢 Trades per hour: Watch out for tokens that have rapid buy activity, like >30+ trades per hour as they are nearing the graduation threshold.
    // 🟢 1st 70 buyers: Check if the first 70 buyers are still holding, or if they bought more, and if the tokens have high concentration, e.g. >50% screams insider control or dev dumping!
    // 🟢 Wallet Setup: Ensure your SOL balance is >0.1 SOL for trades—don’t get caught napping!
    //
    // Verify trades via official links only.
    //  ⚠️Ad Disclaimer:
    //  Heads up! We don’t control ads shown by Telegram here. Beware of fake airdrops, phishing links, or sketchy login pages—stick to https://nyan.bot for the real deal.
    //
    // 💡Pro Tip:
    //  You’re in Basic Mode now with [max] number of filters. Want more? Tap [Here] to upgrade!
    //
    // Get more alpha:
    // 🌐Check out our website: https://nyan.bot
    // 🐥Follow us on X: http://x.com/AI_Nyanbot
    // 💬Join our TG: @AI_Nyanbot
    //     "#)
    //         .reply_markup(options)
    //         .await?;
    //     Ok(())

    bot.send_message(msg.chat.id, markdown!(r#"
Welcome to AI Nyanbot!
  
You’ve just joined the purr-fect Telegram bot for navigating the Solana ecosystem. 
I’m Nyanbot, your AI-powered DEX screener to help you find your next 100x gem and dodge scams!
 
Trading Tips & Rules💡: 
Set Your Conditions:
🟢 Bonding Curve Progress: Aim for pump.fun tokens <50% bonded for early entries, or >80% for safer bets.  
 
Verify trades via official links only.

⚠️Ad Disclaimer⚠️:
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
    Ok(())
}
