// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::{
    CompiledInstruction, InnerInstruction, InnerInstructions, Keys, Signature, Transaction,
    TransactionStatus,
};
use base::model::PublicKey;
use regex::Regex;
use solana_rpc_client::rpc_client::SerializableTransaction;
use solana_sdk::bs58;
use solana_sdk::transaction::VersionedTransaction;
use solana_transaction_status::option_serializer::OptionSerializer;
use solana_transaction_status::UiInstruction::{Compiled, Parsed};
use solana_transaction_status::{EncodedTransactionWithStatusMeta, UiTransactionStatusMeta};
use std::str::FromStr;

pub(crate) fn convert_transaction(tx: EncodedTransactionWithStatusMeta) -> Transaction {
    let meta = tx.meta.clone().unwrap();

    let status = match meta.err {
        None => TransactionStatus::Success,
        Some(err) => TransactionStatus::Error(err.to_string()),
    };

    let decoded = tx.transaction.decode().unwrap();
    let signature = decoded.get_signature();
    // let tx_to_parse = (slot.clone(), tx).try_into().unwrap();

    let vtx = tx.transaction.decode().unwrap();
    let meta = tx.meta.unwrap();

    let keys = extract_keys(&vtx, &meta);

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
            .map(|i| {
                let inner_ixs: Vec<InnerInstruction> = i
                    .instructions
                    .into_iter()
                    .map(|ii| match ii {
                        Compiled(c) => InnerInstruction {
                            instruction: CompiledInstruction {
                                program_id_index: c.program_id_index,
                                accounts: c.accounts,
                                data: bs58::decode(c.data).into_vec().unwrap(),
                            },
                            stack_height: c.stack_height,
                        },
                        Parsed(_) => unimplemented!(),
                    })
                    .collect::<Vec<_>>();

                InnerInstructions {
                    index: i.index,
                    instructions: inner_ixs,
                }
            })
            .collect(),
        OptionSerializer::None => vec![],
        OptionSerializer::Skip => vec![],
    };

    let log_messages = match meta.log_messages {
        OptionSerializer::Some(l) => l,
        OptionSerializer::None => vec![],
        OptionSerializer::Skip => vec![],
    };

    Transaction {
        signature: Signature(signature.to_string()),
        status,
        instructions,
        inner_instructions,
        log_messages,
        keys,
    }
}

pub(crate) fn extract_keys(vtx: &VersionedTransaction, meta: &UiTransactionStatusMeta) -> Keys {
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
