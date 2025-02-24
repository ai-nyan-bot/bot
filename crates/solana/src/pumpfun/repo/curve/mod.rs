// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

mod update;

use crate::pumpfun::rpc::LoadCurveInfo;
use common::model::Limit;
use std::ops::Deref;
use std::sync::Arc;

pub struct CurveQuery {
    pub limit: Limit,
}

#[derive(Debug, Clone)]
pub struct CurveRepo<L: LoadCurveInfo>(pub Arc<CurvePairRepoInner<L>>);

impl<L: LoadCurveInfo> Deref for CurveRepo<L> {
    type Target = CurvePairRepoInner<L>;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

#[derive(Debug)]
pub struct CurvePairRepoInner<L: LoadCurveInfo> {
    info_loader: L,
}

impl<L: LoadCurveInfo> CurveRepo<L> {
    pub fn new(info_loader: L) -> Self {
        Self(Arc::new(CurvePairRepoInner { info_loader }))
    }
}
