// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use solana::rpc::RpcClient;

#[tokio::main]
async fn main() {
    let block = RpcClient::new("http://api.mainnet-beta.solana.com")
        .get_block(328089723)
        .await
        .unwrap()
        .unwrap();

    for tx in block.transactions {
        if tx.signature == "67knbKQwD2VBNgHMYcPM89bJf8hGcqDk9qTDFzGC53iRrGPUiRwymwRdyvZuf43j4uPkpHQc9MtMMk9eXBqEpnzT"{
			dbg!(&tx);
		}
    }
}

