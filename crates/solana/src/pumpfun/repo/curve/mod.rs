// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

mod list_or_populate;
pub(crate) mod shared;
mod update;

use crate::pumpfun::model::Curve;
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

// #[derive(Debug, Clone)]
// pub struct ReadCurveRepo(pub Arc<ReadCurveRepoInner>);
// 
// impl Deref for ReadCurveRepo {
//     type Target = ReadCurveRepoInner;
//     fn deref(&self) -> &Self::Target {
//         self.0.deref()
//     }
// }
// 
// #[derive(Debug)]
// pub struct ReadCurveRepoInner {
//     cache: Cache<CurveId, CurveMint, CachedCurve>,
// }
// 
// impl ReadCurveRepo {
//     pub fn new() -> Self {
//         Self(Arc::new(ReadCurveRepoInner {
//             cache: Cache::default(),
//         }))
//     }
// }
// 
// impl ReadCurveRepo {
//     pub async fn populate_cache(&self, pairs: impl Iterator<Item = &Curve>) {
//         self.cache
//             .put_all(pairs.map(|pair| {
//                 (
//                     pair.id.clone(),
//                     (pair.base.mint.clone(), pair.quote.mint.clone()),
//                     CachedCurve {
//                         id: pair.id.clone(),
//                         mint: (pair.base.mint.clone(), pair.quote.mint.clone()),
//                         base_id: pair.base.id,
//                         quote_id: pair.quote.id,
//                     },
//                 )
//             }))
//             .await;
//     }
// }
