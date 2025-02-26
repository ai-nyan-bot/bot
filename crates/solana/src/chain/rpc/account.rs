// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::{AccountInfo, AccountInfoAtSlot};
use crate::rpc::{RpcClient, RpcResult};
use base::model::PublicKey;
use solana_account_decoder_client_types::UiAccountEncoding;
use solana_client::rpc_config::RpcAccountInfoConfig;
use solana_sdk::commitment_config::CommitmentConfig;

impl RpcClient {
    pub async fn get_account(
        &self,
        key: impl Into<PublicKey>,
    ) -> RpcResult<Option<AccountInfoAtSlot>> {
        let key = key.into();

        let response = self
            .delegate
            .get_account_with_config(
                &key.into(),
                RpcAccountInfoConfig {
                    encoding: Some(UiAccountEncoding::Base64Zstd),
                    data_slice: None,
                    commitment: Some(CommitmentConfig::confirmed()),
                    min_context_slot: None,
                },
            )
            .await?;

        let slot = response.context.slot;

        Ok(response.value.map(|account| AccountInfoAtSlot {
            slot: slot.into(),
            account: AccountInfo {
                lamports: account.lamports,
                data: account.data,
                owner: account.owner.into(),
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            },
        }))
    }

    pub async fn list_accounts(
        &self,
        keys: impl IntoIterator<Item = impl Into<PublicKey>>,
    ) -> RpcResult<Vec<Option<AccountInfoAtSlot>>> {
        let keys = keys.into_iter().map(|id| id.into()).collect::<Vec<_>>();

        let response = self
            .delegate
            .get_multiple_accounts_with_config(
                &keys
                    .into_iter()
                    .map(|k| (k.clone()).into())
                    .collect::<Vec<_>>(),
                RpcAccountInfoConfig {
                    encoding: Some(UiAccountEncoding::Base64Zstd),
                    data_slice: None,
                    commitment: Some(CommitmentConfig::confirmed()),
                    min_context_slot: None,
                },
            )
            .await?;

        let slot = response.context.slot;

        Ok(response
            .value
            .into_iter()
            .map(|account| match account {
                None => None,
                Some(account) => Some(AccountInfoAtSlot {
                    slot: slot.into(),
                    account: AccountInfo {
                        lamports: account.lamports,
                        data: account.data,
                        owner: account.owner.into(),
                        executable: account.executable,
                        rent_epoch: account.rent_epoch,
                    },
                }),
            })
            .collect::<Vec<_>>())
    }
}
