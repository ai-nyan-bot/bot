// // Copyright (c) nyanbot.com 2025.
// // This file is licensed under the AGPL-3.0-or-later.
// 
// // This file includes portions of code from https://github.com/blockworks-foundation/traffic (AGPL 3.0).
// // Original AGPL 3 License Copyright (c) blockworks-foundation 2024.
// 
// use crate::model::Slot;
// use crate::pumpfun::model::Curve;
// use crate::pumpfun::repo::curve::ReadCurveRepo;
// use base::model::{Amount, Percent, Token, TokenId, TokenMint, TokenPairId};
// use common::repo::{RepoResult, Tx};
// use sqlx::Row;
// use std::collections::HashSet;
// use std::vec;
// 
// pub(crate) fn find_missing(ids: &[TokenPairId], curves: &[Curve]) -> Vec<TokenPairId> {
//     let token_ids = curves.iter().map(|c| c.id).collect::<HashSet<_>>();
// 
//     let mut result: Vec<TokenPairId> = Vec::with_capacity(ids.len() - curves.len());
//     for id in ids {
//         if !token_ids.contains(id) {
//             result.push(id.clone());
//         }
//     }
// 
//     result
// }
// 
// impl ReadCurveRepo {
//     pub async fn read_curves_from_cache(
//         &self,
//         cache: &Cache<TokenId, TokenMint, Token>,
//         ids: &[TokenPairId],
//     ) -> RepoResult<Vec<Token>> {
//         let mut result = Vec::with_capacity(ids.len());
// 
//         for id in ids {
//             if let Some(token) = cache.get_by_id(id.clone()).await {
//                 result.push(token)
//             }
//         }
// 
//         Ok(result)
//     }
// 
//     pub async fn read_curves_from_db<'a>(
//         &self,
//         tx: &mut Tx<'a>,
//         ids: &[TokenPairId],
//     ) -> RepoResult<Vec<Curve>> {
//         if ids.is_empty() {
//             return Ok(vec![]);
//         }
// 
//         Ok(sqlx::query(
//             r#"select
//                 id,
//                 slot,
//                 virtual_base_reserves,
//                 virtual_quote_reserves,
//                 real_base_reserves,
//                 real_quote_reserves,
//                 total_supply,
//                 progress,
//                 complete
//               from pumpfun.curve_most_recent
//               where id in (select unnest($1::int4[]))"#,
//         )
//         .bind(&ids)
//         .fetch_all(&mut **tx)
//         .await?
//         .into_iter()
//         .map(|r| Curve {
//             id: r.get::<TokenPairId, _>("id"),
//             slot: r.get::<Slot, _>("slot"),
//             virtual_base_reserves: r.get::<Amount, _>("virtual_base_reserves"),
//             virtual_quote_reserves: r.get::<Amount, _>("virtual_quote_reserves"),
//             real_base_reserves: r.get::<Amount, _>("real_base_reserves"),
//             real_quote_reserves: r.get::<Amount, _>("real_quote_reserves"),
//             total_supply: r.get::<Amount, _>("slot"),
//             progress: r.get::<Percent, _>("progress"),
//             complete: r.get::<bool, _>("complete"),
//         })
//         .collect::<Vec<_>>())
//     }
// }
