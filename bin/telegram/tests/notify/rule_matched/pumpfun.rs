// // Copyright (c) nyanbot.com 2025.
// // This file is licensed under the AGPL-3.0-or-later.
//
// use base::model::{Notification, NotificationChannel, NotificationPayload, NotificationType};
// use common::model::CreatedAt;
// use serde_json::Map;
// use sqlx::types::JsonValue;
// use telegram::{schema, send_notification, AppState, MessageState};
// use teloxide::dispatching::dialogue::InMemStorage;
// use teloxide::dptree;
// use teloxide_tests::{MockBot, MockMessageText};
// use testing::run_test_with_pool;
//
// #[test_log::test(tokio::test)]
// async fn test_sends_notification() {
//     run_test_with_pool(|pool| async move {
//         let state = AppState::testing(pool).await;
//
//         let notification = Notification {
//             id: 10.into(),
//             user: 1.into(),
//             channel: NotificationChannel::Telegram,
//             ty: NotificationType::RuleMatched,
//             payload: NotificationPayload(JsonValue::Object({
//                 let mut map = Map::new();
//                 map.insert(
//                     "venue".to_string(),
//                     JsonValue::String("PumpFun".to_string()),
//                 );
//                 map.insert("token_pair".to_string(), JsonValue::Number(1.into()));
//                 map
//             })),
//             created_at: CreatedAt::now(),
//         };
//
//         let bot = MockBot::new(MockMessageText::new(), schema());
//         bot.dependencies(dptree::deps![
//             InMemStorage::<MessageState>::new(),
//             state.clone()
//         ]);
//         bot.dispatch().await;
//
//         let result = send_notification(state.clone(), notification).await;
//
//         let responses = bot.get_responses();
//         let message = responses.sent_messages.last().unwrap();
//         dbg!(message);
//     })
//     .await;
// }
