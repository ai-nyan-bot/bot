// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::pumpfun::model::Curve;
use crate::pumpfun::repo::curve::CurveRepo;
use crate::pumpfun::rpc::LoadCurveInfo;
use base::model::TokenPairMint;
use common::repo::{RepoResult, Tx};

impl<L: LoadCurveInfo> CurveRepo<L> {

    pub async fn create_curves_if_not_exists<'a>(
        &self,
        tx: &mut Tx<'a>,
        token_mints: &[TokenPairMint],
    ) -> RepoResult<Vec<Curve>> {
        // let mut mints = Vec::with_capacity(token_mints.len());
        // let mut names = std::vec::Vec(token_mints.len());
        // let mut symbols = Vec::with_capacity(token_mints.len());
        // let mut decimals = Vec::with_capacity(token_mints.len());

        // for mint in token_mints {
        //     if !mints.contains(mint) {
        //         if let Some(info) = self.info_loader.load(mint.clone()).await {
        //             mints.push(info.mint);
        //             // names.push(info.name);
        //             // symbols.push(info.symbol);
        //             // decimals.push(info.decimals);
        //         } else {
        //             return Err(RepoError::NotFound);
        //         }
        //     }
        // }

        // Ok(sqlx::query(
        //     r#"with new_token as (
        //     insert into solana.token (mint,name,symbol,decimals)
        //     select
        //         unnest($1::varchar[]) as mint,
        //         unnest($2::varchar[]) as name,
        //         unnest($3::varchar[]) as symbol,
        //         unnest($4::int2[]) as decimals
        //     on conflict (mint) do update set
        //         mint = excluded.mint,
        //         name = excluded.name,
        //         symbol = excluded.symbol,
        //         decimals = excluded.decimals
        //     returning
        //         id,
        //         mint,
        //         name,
        //         symbol,
        //         decimals
        //     )
        //     select * from new_token"#,
        // )
        // .bind(&mints)
        // .bind(&names)
        // .bind(symbols)
        // .bind(decimals)
        // .fetch_all(&mut **tx)
        // .await?
        // .into_iter()
        // .map(|r| Token {
        //     id: r.get::<TokenId, _>("id"),
        //     mint: r.get::<TokenMint, _>("mint"),
        //     name: r.get::<TokenName, _>("name"),
        //     symbol: r.get::<TokenSymbol, _>("symbol"),
        //     decimals: r.get::<Decimals, _>("decimals"),
        // })
        // .collect::<Vec<_>>())
        todo!()
    }
}
