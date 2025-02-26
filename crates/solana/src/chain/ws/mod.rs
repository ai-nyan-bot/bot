// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

mod error;
mod program;
mod response;
mod slot;

use crate::ws::error::WsClientError;
use common::model::WsUrl;
use solana_client::nonblocking::pubsub_client::PubsubClient;
use std::sync::Arc;

pub struct WsClient {
    client: Arc<PubsubClient>,
    // FIXME keep list of subscriptions
}

// type UnsubscribeFn = Box<dyn FnOnce() -> BoxFuture<'static, ()> + Send>;

// struct Subscription {
//     handle: JoinHandle<()>,
//     exit_tx: Sender<()>,
// unsubscribe_fn: UnsubscribeFn,
// }

// FIXME impl drop for subscription

impl WsClient {
    pub async fn new(url: impl Into<WsUrl>) -> WsClientResult<Self> {
        let url = url.into();
        Ok(Self {
            client: Arc::new(PubsubClient::new(url.as_str()).await?),
        })
    }

    // join
}

pub type WsClientResult<T = ()> = Result<T, WsClientError>;
