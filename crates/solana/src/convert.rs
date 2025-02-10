// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::{CompiledInstruction, InnerInstruction, InnerInstructions, Signature, Transaction, TransactionStatus};
use common::model::PublicKey;
use solana_rpc_client::rpc_client::SerializableTransaction;
use solana_sdk::bs58;
use solana_transaction_status::option_serializer::OptionSerializer;
use solana_transaction_status::EncodedTransactionWithStatusMeta;
use solana_transaction_status::UiInstruction::{Compiled, Parsed};

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

    let account_keys: Vec<PublicKey> = vtx.message.static_account_keys().iter().map(|key| (*key).into()).collect();

    Transaction {
        signature: Signature(signature.to_string()),
        status,
        account_keys,
        instructions,
        inner_instructions,
        log_messages,
    }
}
