// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use common::ByteReader;
use solana_client::rpc_config::RpcTransactionConfig;
use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use solana_transaction_status::UiTransactionEncoding;
use std::str::FromStr;
use std::sync::Arc;
use std::u64;

const SWAP_EVENT_DISCRIMINANT: [u8; 8] = [189, 219, 127, 211, 78, 230, 97, 238];

#[derive(Clone, Debug)]
pub struct SwapEvent {
    pub mint: Pubkey,
    pub sol_amount: u64,
    pub token_amount: u64,
    pub is_buy: bool,
    pub user: Pubkey,
    pub timestamp: i64,
    pub virtual_sol_reserves: u64,
    pub virtual_token_reserves: u64,
}

impl SwapEvent {
    pub fn decode(reader: &ByteReader) -> Self {
        SwapEvent {
            mint: Pubkey::try_from(reader.read_range(32).unwrap()).unwrap(),
            sol_amount: reader.read_u64().unwrap(),
            token_amount: reader.read_u64().unwrap(),
            is_buy: reader.read_u8().unwrap() == 1,
            user: Pubkey::try_from(reader.read_range(32).unwrap()).unwrap(),
            timestamp: reader.read_u64().unwrap() as i64,
            virtual_sol_reserves: reader.read_u64().unwrap(),
            virtual_token_reserves: reader.read_u64().unwrap(),
        }
    }
}

#[tokio::main]
pub async fn main() {
    let client = Arc::new(RpcClient::new("https://api.mainnet-beta.solana.com".to_string()));

    // AUvnh9oF5xEAG2f2ccZ8vbaKNcHwQf4CcD5Bas8HuAw8qXd26B5XxJkJYLmjVYyyVzxmwUp9AzZeDZqm4JRYE7X
    // 2RqhBZykXDPG6qt5fDRJujKuZL9yqAXxQuMkJ4JC9u9fAJEe774dBVNi4E8UpAbdWB47GpBm1avug1a6VGNN3Ujv

    let tx = client
        .get_transaction_with_config(
            &Signature::from_str("2RqhBZykXDPG6qt5fDRJujKuZL9yqAXxQuMkJ4JC9u9fAJEe774dBVNi4E8UpAbdWB47GpBm1avug1a6VGNN3Ujv").unwrap(),
            RpcTransactionConfig {
                encoding: Some(UiTransactionEncoding::Base58),
                commitment: Some(CommitmentConfig::confirmed()),
                max_supported_transaction_version: Some(0),
            },
        )
        .await
        .unwrap();

    // let tx: TransactionToParse = (Slot::from(tx.slot), tx.transaction).try_into().unwrap();
    // let p = PumpFunParser::new();
    //
    // let result = p.parse(&tx).unwrap();

    // let mut result = vec![];
    /*    for inner in &tx.transaction.meta.unwrap().inner_instructions.unwrap() {
        for instruction in &inner.instructions {
            match instruction {
                UiInstruction::Compiled(instruction) => {
                    let decoded = bs58::decode(instruction.data.as_bytes()).into_vec().unwrap();

                    if decoded.len() > 16 {
                        let reader = ByteReader::new(&decoded);
                        reader.seek(8).unwrap(); // skip anchor method identifier

                        let disc = reader.read_range(8).unwrap();
                        if disc == SWAP_EVENT_DISCRIMINANT {
                            assert_eq!(disc, SWAP_EVENT_DISCRIMINANT);

                            let evt = SwapEvent::decode(&reader);
                        }
                    }
                }
                UiInstruction::Parsed(_) => {}
            }
        }
    }*/
}
