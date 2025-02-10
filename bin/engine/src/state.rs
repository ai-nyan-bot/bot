// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::fact::FactService;
use base::service::{NotificationService, StrategyService};
use std::ops::Deref;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState(pub Arc<AppStateInner>);

impl Deref for AppState {
    type Target = AppStateInner;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

#[derive(Clone)]
pub struct AppStateInner {
    pub service: Service,
}

#[derive(Clone)]
pub struct Service {
    pub fact: FactService,
    pub notification: NotificationService,
    pub strategy: StrategyService,
}
