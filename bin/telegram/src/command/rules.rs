// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::{AppState, HandlerResult};
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::{Message, Requester};
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, WebAppInfo};
use teloxide::Bot;
use url::Url;

pub(crate) async fn rules(bot: Bot, msg: Message, _state: AppState) -> HandlerResult {
    let options = InlineKeyboardMarkup::new(vec![vec![InlineKeyboardButton::web_app(
        "Open rules".to_string(),
        WebAppInfo {
            url: Url::parse("https://telegram.nyan.bot/rules").unwrap(),
        },
    )]]);

    bot.send_message(msg.chat.id, r#"
Set your custom rules to screen for potential 100x gems!

For example, you can choose to filter for the following 
🟢Market Cap: Min/Max (e.g., $50K-500K). 
🟢Tx Count: Min (e.g., 50) for hype (>100) or sleepers (<20).  
🟢Honeypot: Exclude scams (Yes/No).   
🟢Bonding Curve: 0-50% (snipe) or 80-100% (safe). 
 
Tweak, save, and pounce on profits! 
Remember to DYOR!    
"#)
        .reply_markup(options)
        .await?;
    Ok(())
}
