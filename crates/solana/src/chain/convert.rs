// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::{
    Balance, CompiledInstruction, InnerInstruction, InnerInstructions, Keys, Signature, SolBalance,
    TokenBalance, Transaction, TransactionStatus,
};
use base::model::{DecimalAmount, Mint, PublicKey};
use bigdecimal::BigDecimal;
use regex::Regex;
use solana_rpc_client::rpc_client::SerializableTransaction;
use solana_sdk::bs58;
use solana_sdk::transaction::VersionedTransaction;
use solana_transaction_status::option_serializer::OptionSerializer;
use solana_transaction_status::UiInstruction::{Compiled, Parsed};
use solana_transaction_status::{EncodedTransactionWithStatusMeta, UiTransactionStatusMeta};
use std::str::FromStr;

pub(crate) fn convert_transaction(tx: EncodedTransactionWithStatusMeta) -> Transaction {
    let meta = tx.meta.unwrap();

    let vtx = tx.transaction.decode().unwrap();
    let signature = Signature(vtx.get_signature().to_string());
    let keys = extract_keys(&vtx, &meta);
    let balance = extract_balance(&meta, &keys);

    let status = meta.err.map_or(TransactionStatus::Success, |err| {
        TransactionStatus::Error(err.to_string())
    });

    let instructions = vtx
        .message
        .instructions()
        .iter()
        .map(|i| CompiledInstruction {
            program_id_index: i.program_id_index,
            accounts: i.accounts.clone(),
            data: i.data.clone(),
        })
        .collect();

    let inner_instructions = match meta.inner_instructions {
        OptionSerializer::Some(inners) => inners
            .into_iter()
            .map(|i| InnerInstructions {
                index: i.index,
                instructions: i
                    .instructions
                    .into_iter()
                    .filter_map(|ii| match ii {
                        Compiled(c) => Some(InnerInstruction {
                            instruction: CompiledInstruction {
                                program_id_index: c.program_id_index,
                                accounts: c.accounts,
                                data: bs58::decode(c.data).into_vec().unwrap(),
                            },
                            stack_height: c.stack_height,
                        }),
                        Parsed(_) => {
                            unimplemented!()
                        }
                    })
                    .collect(),
            })
            .collect(),
        _ => Vec::new(),
    };

    let log_messages = meta.log_messages.unwrap_or(vec![]);

    Transaction {
        signature,
        status,
        balance,
        instructions,
        inner_instructions,
        log_messages,
        keys,
    }
}

fn extract_balance(meta: &UiTransactionStatusMeta, keys: &Keys) -> Balance {
    let mut sol = Vec::with_capacity(meta.pre_balances.len());
    for (idx, key) in keys.static_account.iter().enumerate() {
        let pre = meta.pre_balances[idx];
        let post = meta.post_balances[idx];
        sol.push(SolBalance {
            address: key.clone(),
            pre: DecimalAmount::new(pre, 9),
            post: DecimalAmount::new(post, 9),
        });
    }

    let mut token = Vec::new();

    if let (OptionSerializer::Some(pre), OptionSerializer::Some(post)) =
        (&meta.pre_token_balances, &meta.post_token_balances)
    {
        for (pre, post) in pre.iter().zip(post.iter()) {
            pre.ui_token_amount.decimals;
            let pre_balance = BigDecimal::from_str(pre.ui_token_amount.amount.as_str()).unwrap();
            let post_balance = BigDecimal::from_str(post.ui_token_amount.amount.as_str()).unwrap();

            token.push(TokenBalance {
                mint: Mint::from(pre.mint.clone()),
                pre: DecimalAmount::new(pre_balance, pre.ui_token_amount.decimals),
                post: DecimalAmount::new(post_balance, post.ui_token_amount.decimals),
            })
        }
    }

    Balance { sol, token }
}

fn extract_keys(vtx: &VersionedTransaction, meta: &UiTransactionStatusMeta) -> Keys {
    let static_account: Vec<PublicKey> = vtx
        .message
        .static_account_keys()
        .iter()
        .map(|key| (*key).into())
        .collect();

    let log_account = if let OptionSerializer::Some(logs) = &meta.log_messages {
        extract_keys_from_logs(logs.as_slice())
    } else {
        vec![]
    };

    Keys {
        static_account,
        log_account,
    }
}

fn extract_keys_from_logs(log_messages: &[impl AsRef<str>]) -> Vec<PublicKey> {
    let re = Regex::new(r"Program ([1-9A-HJ-NP-Za-km-z]{32,44})").unwrap();
    let mut keys = Vec::new();

    for log in log_messages {
        if let Some(caps) = re.captures(log.as_ref()) {
            if let Some(key) = caps.get(1) {
                match PublicKey::from_str(key.as_str()) {
                    Ok(key) => keys.push(key),
                    Err(_) => {}
                }
            }
        }
    }
    keys.sort();
    keys.dedup();
    keys
}

#[cfg(test)]
mod tests {
    use crate::convert::extract_keys_from_logs;

    #[test]
    fn test_extract_keys_from_logs() {
        let result = extract_keys_from_logs(&[
            "Program 11111111111111111111111111111111 invoke [1]",
            "Program 11111111111111111111111111111111 success",
            "Program 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P invoke [2]",
        ]);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], "11111111111111111111111111111111");
        assert_eq!(result[1], "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P");
    }
}
