// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::solana::balance::index_token_balance;
use crate::solana::indexer::IndexerRepo;
use crate::solana::state::State;
use crate::solana::{jupiter, pumpfun};
use base::model::{AddressId, DecimalAmount, Decimals, Mint, PublicKey, TokenId};
use base::repo::TokenToInsert;
use solana::jupiter::parse::JupiterParser;
use solana::model::{Block, TransactionStatus};
use solana::parse::Parser;
use solana::pumpfun::PumpFunParser;
use solana::repo::TokenBalanceToInsert;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use std::time::Instant;
use tracing::{debug, info};

pub async fn index_block(state: State, block: Block) {
    info!("index {}", block.slot);

    let jupiter_parser = JupiterParser::new();
    let pumpfun_parser = PumpFunParser::new();

    let indexer_repo = IndexerRepo::default();

    let pumpfun_account =
        PublicKey::from_str("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P").unwrap();

    let jupiter_account =
        PublicKey::from_str("JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4").unwrap();

    // FIXME it would be interesting to see what the time difference is between indexing a block and the actual block time

    let mut jupiter_slot_swaps = solana::jupiter::repo::SlotSwaps {
        slot: block.slot,
        timestamp: block.timestamp,
        swaps: vec![],
    };

    let mut pumpfun_slot_swaps = solana::pumpfun::repo::SlotSwaps {
        slot: block.slot,
        timestamp: block.timestamp,
        swaps: vec![],
    };

    let mut pumpfun_token_mints = vec![];

    let tx_parsing_start = Instant::now();

    for transaction in &block.transactions {
        if transaction.status == TransactionStatus::Success {
            if transaction.keys.contains(&pumpfun_account) {
                if let Ok(instructions) = pumpfun_parser.parse(&transaction) {
                    for instruction in instructions {
                        match instruction {
                            solana::pumpfun::model::Instruction::Create {
                                name,
                                symbol,
                                uri,
                                mint,
                                user,
                                ..
                            } => {
                                let mut tx = state.pool.begin().await.unwrap();
                                let creator = state
                                    .address_repo
                                    .get_or_populate_by_key(&mut tx, user)
                                    .await
                                    .unwrap();

                                tx.commit().await.unwrap();

                                pumpfun_token_mints.push(TokenToInsert {
                                    mint,
                                    name: Some(name),
                                    symbol: Some(symbol),
                                    decimals: Decimals::from(6),
                                    supply: Some(DecimalAmount::from(1_000_000_000i64)),
                                    metadata: Some(uri),
                                    description: None,
                                    image: None,
                                    website: None,
                                    creator: Some(creator.id),
                                    block: Some(block.slot.into()),
                                    block_time: Some(block.timestamp),
                                })
                            }

                            solana::pumpfun::model::Instruction::Swap {
                                mint,
                                sol_amount,
                                token_amount,
                                is_buy,
                                user,
                                virtual_sol_reserves,
                                virtual_token_reserves,
                                ..
                            } => {
                                pumpfun_slot_swaps
                                    .swaps
                                    .push(solana::pumpfun::repo::SlotSwap {
                                        mint,
                                        amount_base: token_amount,
                                        amount_quote: sol_amount,
                                        is_buy,
                                        wallet: user,
                                        virtual_base_reserves: virtual_token_reserves,
                                        virtual_quote_reserves: virtual_sol_reserves,
                                        signature: transaction.signature.clone(),
                                    });
                            }
                        }
                    }
                }
            }

            if transaction.keys.contains(&jupiter_account) {
                if let Ok(instructions) = jupiter_parser.parse(&transaction) {
                    for instruction in instructions {
                        match instruction {
                            solana::jupiter::model::Instruction::Swap { swaps, signer } => {
                                for swap in &swaps {
                                    jupiter_slot_swaps.swaps.push(
                                        solana::jupiter::repo::SlotSwap {
                                            input_mint: swap.input_mint.clone(),
                                            input_amount: swap.input_amount.clone(),
                                            output_mint: swap.output_mint.clone(),
                                            output_amount: swap.output_amount.clone(),
                                            wallet: signer.clone(),
                                            signature: transaction.signature.clone(),
                                        },
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let mut seen_addresses = HashSet::new();
    let mut addresses = Vec::new();

    let mut seen_mints = HashSet::new();
    let mut mints = Vec::new();

    for transaction in &block.transactions {
        if transaction.status == TransactionStatus::Success {
            for t in &transaction.balance.token {
                if seen_addresses.insert(t.address.clone()) {
                    addresses.push(t.address.clone());
                }

                if seen_mints.insert(t.mint.clone()) {
                    mints.push(t.mint.clone());
                }
            }
            for s in &transaction.balance.sol {
                if seen_addresses.insert(s.address.clone()) {
                    addresses.push(s.address.clone());
                }
            }
        }
    }

    let mut tx = state.pool.begin().await.unwrap();

    let addresses = state
        .address_repo
        .list_or_populate(&mut tx, addresses)
        .await
        .unwrap();

    let tokens = state
        .token_repo
        .list_or_populate(&mut tx, mints)
        .await
        .unwrap();
    tx.commit().await.unwrap();

    let addresses: HashMap<PublicKey, AddressId> =
        addresses.into_iter().map(|a| (a.address, a.id)).collect();

    let tokens: HashMap<Mint, TokenId> = tokens.into_iter().map(|m| (m.mint, m.id)).collect();

    // let mut sol_balances: Vec<SolBalanceToInsert> = vec![];
    let mut token_balances: Vec<TokenBalanceToInsert> = vec![];
    for transaction in block.transactions {
        if transaction.status == TransactionStatus::Success {
            for token in transaction.balance.token {
                token_balances.push(TokenBalanceToInsert {
                    slot: block.slot,
                    timestamp: block.timestamp.0,
                    address: addresses[&token.address],
                    token: tokens[&token.mint].clone(),
                    pre: token.pre,
                    post: token.post,
                })
            }

            // for sol in transaction.balance.sol {
            //     sol_balances.push(SolBalanceToInsert {
            //         slot: block.slot,
            //         timestamp: block.timestamp.0,
            //         address: addresses[&sol.address],
            //         pre: sol.pre,
            //         post: sol.post,
            //     })
            // }
        }
    }

    let tx_parsing_done = Instant::now();
    debug!(
        "transaction parsing took {} ms",
        tx_parsing_done.duration_since(tx_parsing_start).as_millis()
    );

    let mut tx = state.pool.begin().await.unwrap();
    let slot = block.slot;

    let indexing_start = Instant::now();
    pumpfun::index_tokens(&mut tx, state.clone(), pumpfun_token_mints).await;
    pumpfun::index_swap(&mut tx, state.clone(), pumpfun_slot_swaps).await;
    jupiter::index_swap(&mut tx, state.clone(), jupiter_slot_swaps).await;
    index_token_balance(&mut tx, state.clone(), token_balances).await;
    // index_sol_balance(&mut tx, state.clone(), sol_balances).await;
    let indexing_done = Instant::now();

    debug!(
        "indexing took {} ms",
        indexing_done.duration_since(indexing_start).as_millis()
    );

    indexer_repo.set(&mut tx, slot).await.unwrap();
    tx.commit().await.unwrap();
}
