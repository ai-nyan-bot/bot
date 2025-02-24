// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use solana::pumpfun::Rpc;
use solana::rpc::RpcClient;
use solana_sdk::pubkey;

#[tokio::main]
async fn main() {
    let rpc = Rpc::new(RpcClient::new("https://api.mainnet-beta.solana.com"));

    // [crates/solana/src/venue/pumpfun/rpc/mod.rs:54:9] &account = Account {
    //     lamports: 32803512089,
    //     data.len: 49,
    //     owner: 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P,
    //     executable: false,
    //     rent_epoch: 18446744073709551615,
    //     data: 17b7f83760d8ac60901a02e92bd20100e9be4e9f0e0000009082ef9c9ad30000e9122ba3070000000080c6a47e8d030000,
    // }
    // [crates/solana/examples/pumpfun_curve.rs:14:5] &bc = BondingCurveAccount {
    //     discriminator: 6966180631402821399,
    //     virtual_token_reserves: 512561011366544,
    //     virtual_sol_reserves: 62802280169,
    //     real_token_reserves: 232661011366544,
    //     real_sol_reserves: 32802280169,
    //     token_total_supply: 1000000000000000,
    //     complete: false,
    // }
    // reserved_tokens: 206900000000000
    // irtr: 793100000000000
    // progress 71
    // 71

    // let ga = rpc.get_global_account().await;
    // dbg!(&ga);
    // 
    // let ca = rpc
    //     .get_curve_account(pubkey!("CpV7zK77DkyVvbCgGWtWrsqvh2VonFrqPg8cecrTpump"))
    //     .await
    //     .unwrap();
    // dbg!(&ca);

    // println!("{}", bc.get_market_cap_sol());
    // println!("{}", bc.get_final_market_cap_sol(1000));
    // println!("{}", ca.progress());
}
