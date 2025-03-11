// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

// This file includes portions of code from https://github.com/blockworks-foundation/traffic (AGPL 3.0).
// Original AGPL 3 License Copyright (c) blockworks-foundation 2024.

use crate::load_all;
use crate::model::{DecimalAmount, Mint, Token};
use crate::repo::token::insert::TokenToInsert;
use crate::repo::TokenRepo;
use common::repo::error::RepoError;
use common::repo::{RepoResult, Tx};
use log::error;
use std::ops::Deref;

impl TokenRepo {
    pub async fn populate_token<'a>(
        &self,
        tx: &mut Tx<'a>,
        token_mints: &[Mint],
    ) -> RepoResult<Vec<Token>> {
        if token_mints.is_empty() {
            return Ok(vec![]);
        }

        let mut to_insert = Vec::with_capacity(token_mints.len());

        for info in load_all(self.info_loader.deref(), token_mints).await {
            if let Some(info) = info {
                let decimals = info.decimals.expect("token decimals required");

                to_insert.push(TokenToInsert {
                    mint: info.mint.expect("token mint required"),
                    name: info.name,
                    symbol: info.symbol,
                    decimals: decimals.clone(),
                    supply: info
                        .supply
                        .map(|amount| DecimalAmount::new(amount, decimals)),
                    metadata: info.metadata,
                    description: info.description,
                    image: info.image,
                    website: info.website,
                    creator: None,
                    block: None,
                    block_time: None,
                });
            } else {
                error!("unable to load token info");
                return Err(RepoError::NotFound);
            }
        }

        self.insert_token(tx, to_insert).await
    }
}
