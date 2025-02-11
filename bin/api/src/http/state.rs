// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::config::Config;
use base::service::RuleService;
use base::service::UserService;
use std::ops::Deref;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState(pub Arc<AppStateInner>);

impl AppState {
    pub fn rule_service(&self) -> RuleService {
        self.service.rule.clone()
    }
    pub fn user_service(&self) -> UserService {
        self.service.user.clone()
    }
}

impl Deref for AppState {
    type Target = AppStateInner;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

#[derive(Clone)]
pub struct AppStateInner {
    pub config: Config,
    pub service: Service,
}

#[derive(Clone)]
pub struct Service {
    pub rule: RuleService,
    pub user: UserService,
}
