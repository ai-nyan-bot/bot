// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::Account;
use crate::rpc::{RpcClient, RpcResult};
use common::model::PublicKey;
use solana_client::rpc_config::RpcAccountInfoConfig;
use solana_sdk::commitment_config::CommitmentConfig;

impl RpcClient {
    pub async fn get_multiple_accounts(&self, keys: &[PublicKey]) -> RpcResult<Vec<Account>> {
        let mut result = Vec::with_capacity(keys.len());
        //
        // let mut tasks = FuturesOrdered::new();
        // for chunk in keys.chunks(100) {
        //     tasks.push_back(async {
        //         let response = self
        //             .rpc_client
        //             .get_multiple_accounts_with_config(
        //                 chunk,
        //                 RpcAccountInfoConfig {
        //                     encoding: None,
        //                     data_slice: None,
        //                     commitment: Some(CommitmentConfig::confirmed()),
        //                     min_context_slot: None,
        //                 },
        //             )
        //             .await?;
        //
        //         Ok::<_, anyhow::Error>(response.value)
        //     });
        // }
        //
        // while let Some(result) = tasks.next().await {
        //     result.extend(result?);
        // }

        // let mut handles = Vec::new();
        // // for chunk in keys.chunks(100) {
        // handles.push(
        //     tokio::spawn(async {
        let response = self
            .client
            .get_multiple_accounts_with_config(
                &*keys.into_iter().map(|k| (k.clone()).into()).collect::<Vec<_>>(),
                RpcAccountInfoConfig {
                    encoding: None,
                    data_slice: None,
                    commitment: Some(CommitmentConfig::confirmed()),
                    min_context_slot: None,
                },
            )
            .await
            .unwrap();

        for account in response.value {
            if let Some(account) = account {
                result.push(Account {
                    lamports: account.lamports,
                    data: account.data,
                    owner: account.owner.into(),
                    executable: account.executable,
                    rent_epoch: account.rent_epoch,
                })
            }
        }
        // })
        // );
        // }

        Ok(result)
    }
}
