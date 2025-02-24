// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::ws::WsClient;

impl WsClient {
    // pub async fn subscribe_program(&self, public_key: PublicKey) -> WsClientResult<(Receiver<KeyedAccount>, JoinHandle<()>)> {
    // 	let (tx, rx) = flume::bounded::<KeyedAccount>(100);
    //
    // 	let client = self.client.clone();
    //
    // 	let handle = tokio::spawn(async move {
    // 		let (mut stream, unsubscribe) = client
    // 			.program_subscribe(
    // 				&public_key.into(),
    // 				Some(RpcProgramAccountsConfig {
    // 					filters: None,
    // 					account_config: RpcAccountInfoConfig {
    // 						encoding: None,
    // 						data_slice: None,
    // 						commitment: None,
    // 						min_context_slot: None,
    // 					},
    // 					with_context: Some(true),
    // 					sort_results: None,
    // 				}),
    // 			)
    // 			.await
    // 			.unwrap();
    //
    // 		while let Some(response) = stream.next().await {
    // 			trace!("received: {:?}", response);
    //
    // 			let _ = tx
    // 				.send_async(KeyedAccount {
    // 					slot: response.context.slot,
    // 					pubkey: response.value.pubkey.into(),
    // 					account: Account {
    // 						lamports: response.value.account.lamports,
    // 						data: response.value.account.data.decode().unwrap(),
    // 						owner: response.value.account.owner.into(),
    // 						executable: response.value.account.executable,
    // 						rent_epoch: response.value.account.rent_epoch,
    // 					},
    // 				})
    // 				.await;
    // 		}
    // 	});
    //
    // 	Ok((rx, handle))
    // }
}
