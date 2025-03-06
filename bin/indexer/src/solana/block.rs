// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::solana::indexer::IndexerRepo;
use crate::solana::state::State;
use crate::solana::{jupiter, pumpfun};
use base::model::{Mint, PublicKey};
use base::LoadTokenInfo;
use solana::jupiter::parse::JupiterParser;
use solana::model::{Block, TransactionStatus};
use solana::parse::Parser;
use solana::pumpfun::PumpFunParser;
use std::str::FromStr;
use std::time::Instant;
use tracing::{debug, info};

pub async fn index_block<L: LoadTokenInfo<Mint> + Clone>(state: State<L>, block: Block) {
    info!("index {}", block.slot);

    let jupiter_parser = JupiterParser::new();
    let pumpfun_parser = PumpFunParser::new();

    let indexer_repo = IndexerRepo::default();

    let pumpfun_account =
        PublicKey::from_str("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P").unwrap();

    let jupiter_account =
        PublicKey::from_str("JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4").unwrap();

    // FIXME it would be interesting to see what the time difference is between indexing a block and the actual block time

    let mut jupiter_slot_trades = solana::jupiter::repo::SlotTrades {
        slot: block.slot,
        timestamp: block.timestamp,
        trades: vec![],
    };

    let mut pumpfun_slot_trades = solana::pumpfun::repo::SlotTrades {
        slot: block.slot,
        timestamp: block.timestamp,
        trades: vec![],
    };

    let tx_parsing_start = Instant::now();

    for transaction in block.transactions {
        if transaction.status == TransactionStatus::Success {
            if transaction.keys.contains(&pumpfun_account) {
                if let Ok(instructions) = pumpfun_parser.parse(&transaction) {
                    for instruction in instructions {
                        match instruction {
                            solana::pumpfun::model::Instruction::Create { .. } => {}
                            solana::pumpfun::model::Instruction::Trade {
                                mint,
                                sol_amount,
                                token_amount,
                                is_buy,
                                user,
                                virtual_sol_reserves,
                                virtual_token_reserves,
                                ..
                            } => {
                                if sol_amount > 0 && token_amount > 0 {
                                    pumpfun_slot_trades.trades.push(
                                        solana::pumpfun::repo::SlotTrade {
                                            mint,
                                            amount_base: token_amount,
                                            amount_quote: sol_amount,
                                            is_buy,
                                            wallet: user,
                                            virtual_base_reserves: virtual_token_reserves,
                                            virtual_quote_reserves: virtual_sol_reserves,
                                            signature: transaction.signature.clone(),
                                        },
                                    );
                                }
                            }
                        }
                    }
                }
            }

            if transaction.keys.contains(&jupiter_account) {
                if let Ok(instructions) = jupiter_parser.parse(&transaction) {
                    for instruction in instructions {
                        match instruction {
                            solana::jupiter::model::Instruction::Trade { swaps, signer } => {
                                let first = swaps.first().unwrap();
                                let last = swaps.last().unwrap();

                                if first.input_amount > 0 && last.output_amount > 0 {
                                    jupiter_slot_trades.trades.push(
                                        solana::jupiter::repo::SlotTrade {
                                            input_mint: first.input_mint.clone(),
                                            input_amount: first.input_amount,
                                            output_mint: last.output_mint.clone(),
                                            output_amount: last.output_amount,
                                            wallet: signer,
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
    let tx_parsing_done = Instant::now();
    debug!(
        "transaction parsing took {} ms",
        tx_parsing_done.duration_since(tx_parsing_start).as_millis()
    );

    let mut tx = state.pool.begin().await.unwrap();
    let slot = block.slot;

    let indexing_start = Instant::now();
    pumpfun::index_trade(&mut tx, state.clone(), pumpfun_slot_trades).await;
    jupiter::index_trade(&mut tx, state.clone(), jupiter_slot_trades).await;
    let indexing_done = Instant::now();

    debug!(
        "indexing took {} ms",
        indexing_done.duration_since(indexing_start).as_millis()
    );

    indexer_repo.set(&mut tx, slot).await.unwrap();
    tx.commit().await.unwrap();
}
