// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use solana::token_info::uri::TokenInfoUriLoader;

#[test_log::test(tokio::test)]
async fn test_pumpfun_ipfs() {
    let test_instance = TokenInfoUriLoader::new();

    let token_info = test_instance
        .load("https://ipfs.io/ipfs/QmW5K83aSvJakhDMsUGoBFzdBjmuD2kXZXUaTVR7QUZPDk".into())
        .await
        .unwrap();

    assert_eq!(token_info.name.unwrap(), "farting capybara");
    assert_eq!(token_info.symbol.unwrap(), "fartingcap");
    assert_eq!(
        token_info.description.unwrap(),
        "farting capybara sitting on crocodile holding banana on the head"
    );
    assert_eq!(
        token_info.image.unwrap(),
        "https://ipfs.io/ipfs/QmVSkzzGYea342mk8PvWz1WHokyGUFs8LQ7AgBFJMYdEaG"
    )
}
