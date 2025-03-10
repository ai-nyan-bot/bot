// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::model::DecimalAmount;
use base::repo::{AddressRepo, TokenRepo};
use base::test::NeverCalledTokenInfoLoader;
use indexer::solana::block::index_block;
use indexer::solana::state::{State, StateInner};
use sqlx::Executor;
use std::sync::Arc;
use testing::run_test_with_pool_on_empty_db;

#[test_log::test(sqlx::test)]
async fn test_index_block_317897944() {
    run_test_with_pool_on_empty_db(|pool| async move {
		let block = serde_json::from_str(include_str!("block_317897944.json")).unwrap();

		let mut tx = pool.begin().await.unwrap();
		tx.execute(r#"
insert into solana.token (id, version, mint, name, symbol, decimals, supply, metadata, description, image, website, creator_id, updated_at) values
	(1001, 0, 'J2HfKQoybUgP9uLFrzLjf32fKGQikSvoqLJpErdRpump', 'Mowgli the Chug', 'mowgli', 6, 999999152.585054000000, 'https://ipfs.io/ipfs/QmVyvBMR21tNCo98EbbKYp7K3G4ShKP8B1eQqdUM9Ma3xp', null, null, null, null, '2025-03-10 03:31:13.815471 +00:00'),
	(1002, 0, '2dz9KxedMT3k3k7iZoDYqYZ5n5meZqkvFH88sjb1pump', 'FEBULLARY', 'FEBULLARY', 6, 999350056.643482000000, 'https://ipfs.io/ipfs/QmPCGMzLHMHg8D4idDed6zhUrMCYe2SrYGVjg6bTfjLY7g', null, null, null, null, '2025-03-10 03:31:13.815471 +00:00'),
	(1003, 0, '3GY2Znk6J7Hfu7VH8WnZnYdMUU1RD8Gw7rKB1iuFpump', 'College Jackie Chan', 'LiveJackie', 6, 999998506.139485000000, 'https://ipfs.io/ipfs/QmWPKXMNs6Q18j6jBfG7cw2NArSJqD8pseRwpxDzQPudPQ', null, null, null, null, '2025-03-10 03:31:13.815471 +00:00'),
	(1005, 0, 'usyYNEZMGnM56u1T9813y53pF7vztDibwEPDunXwQDW', 'TROBLOX', 'BLOX', 6, 1000000000.000000000000, 'https://ipfs.io/ipfs/Qma5XKnmMcSRxdAUNxdPGT6RrLScUci5f5HLWbEWb18oiY', null, null, null, null, '2025-03-10 03:31:13.815471 +00:00'),
	(1006, 0, '251yuzr1SEKkn9xA94KN3JGVtsrh4ahHVuudNgSHpump', 'TRUMPIUS MAXIMUS TERMINAL', '$T.M.T', 6, 1000000000.000000000000, 'https://ipfs.io/ipfs/QmcxZm4vyXGBqAKGNPoGchxq2BNHYxHSWpU99sEvG81J4Y', null, null, null, null, '2025-03-10 03:31:13.815471 +00:00'),
	(1007, 0, '2JoUZ4TuuBnS2M1o9TpTfhtmKyzKdx2Ki9z5Cv75pump', 'Monica', 'MONICA', 6, 1000000000.000000000000, 'https://ipfs.io/ipfs/QmfLKij7usrNNzMUfN76YQXDa3n55ZKrd8ZPRZYeYGpngy', null, null, null, null, '2025-03-10 03:31:13.815471 +00:00'),
	(1008, 0, '2WTrAr7vbZpg4cEhXjaYumEEQYAWr3DmoRFdBQ5Xpump', 'Solaris AI', 'SOLARIS', 6, 1000000000.000000000000, 'https://ipfs.io/ipfs/Qmb3gF2rLW2b9z4dLT7PLW3qabGqrfjbhiZptSLU7wJupB', null, null, null, null, '2025-03-10 03:31:13.815471 +00:00'),
	(1009, 0, '9Hv8f1M7EUZgAPtZz7UuPSjydnk35qU9mREWbSdapump', 'Solazard', '$SOLZ', 6, 1000000000.000000000000, 'https://ipfs.io/ipfs/QmTG2N92KTmZLDYCJn5g8T7Kqm3jwD4Cgtu7xZo8jX4AXt', null, null, null, null, '2025-03-10 03:31:13.815471 +00:00'),
	(1010, 0, '9Vjm63MkpKuiuH91wwgYNuC9uZS3xDvTULu5LCMfpump', 'Lacy', 'Lacy', 6, 1000000000.000000000000, 'https://ipfs.io/ipfs/QmQwBDBLaGvRbkL3qQUm5WqjWMvpMqLrcsYi72NDtqVmFU', null, null, null, null, '2025-03-10 03:31:13.815471 +00:00'),
	(1011, 0, 'FxidwFhfL1byzdiBBJ2HQFpUkCjFWce7eZwCK9GYwNjK', 'Ana Destierro', 'Ana', 6, 1000000000.000000000000, 'https://ipfs.io/ipfs/QmZn2Mi8zup1axTkZL5grkURjxYrfRSEfPyi59bxuxvQjT', null, null, null, null, '2025-03-10 03:31:13.815471 +00:00'),
	(1012, 0, '8UrUDAKVPtX69BkoVHWv3b3wz5zynNuJTmgpUs9aKQHN', 'United Pup Service', 'UPS', 6, 999944823.298702000000, 'https://ipfs.io/ipfs/QmdVw9LBoEwGG7nu9DDdMe2Ysk4joVU2zRSacW8VrrMKf5', null, null, null, null, '2025-03-10 03:31:13.815471 +00:00'),
	(1013, 0, '6EGtU6776cVwJyRexrHkhCKxRQWs91TccJfueMQhpump', 'Jeet of the United States', 'JETUS', 6, 998301371.233877000000, 'https://ipfs.io/ipfs/QmYA9H4EVg3R21Q3bS5L12eTuKLWex4H48f11Sr2aB7z2e', null, null, null, null, '2025-03-10 03:31:13.815471 +00:00'),
	(1014, 0, 'BYBEfivjEgouanqm7hTceDSDhCJtdBFo8k9r3Xtfpump', 'Bennu', 'Bennu', 6, 999938195.231148000000, 'https://ipfs.io/ipfs/QmZT8smK2v5v2usxQpLo1nrumKrC98QhNQUG3dDSqakxTx', null, null, null, null, '2025-03-10 03:31:13.815471 +00:00'),
	(1015, 0, 'D7ptz7kKS165kKYXt7ZSpZxXG6vcNAoXyq73asnDpump', 'Power b', 'Power b', 6, 1000000000.000000000000, 'https://ipfs.io/ipfs/QmYmt3NQd51NTh7rrLBusxpRqGiSG6Bq6TZW3ogdmv7hWF', null, null, null, null, '2025-03-10 03:31:13.815471 +00:00'),
	(1016, 0, 'AH5bPbharRJh8tnsGAHHUBTSgZhJSeHEHkjZXmZ4pump', 'pepeinatux', '$INA', 6, 999881314.822113000000, 'https://ipfs.io/ipfs/QmUSiVNe8rpprjhiRnfTNESYM81QGRHJbyX79C77TLWExH', null, null, null, null, '2025-03-10 03:31:13.815471 +00:00'),
	(1017, 0, 'Gi7gEU5KkHW1Cp42ggpShDJxaBgyxqntiumZYjgA994r', 'DINGO', 'DICC', 6, 999857290.349391000000, 'https://ipfs.io/ipfs/QmXGYa4kfEJP2ARo1i3qWouiDi2nRiXKdxVWpVwNgqAzWM', null, null, null, null, '2025-03-10 03:31:13.815471 +00:00'),
	(1018, 0, '5mbK36SZ7J19An8jFochhQS4of8g6BwUjbeCSxBSoWdp', 'michi', '$michi', 6, 555766634.641776000000, 'https://ipfs.io/ipfs/QmaFSKo4FX43NsVETn4nPfnFrXMqcGyK4mvKuCz2Pg65ji', null, null, null, null, '2025-03-10 03:31:13.815471 +00:00'),
	(1019, 0, '7GCihgDB8fe6KNjn2MYtkzZcRjQy3t9GHdC8uHYmW2hr', 'POPCAT', 'POPCAT', 9, 979940654.609217463000, 'https://arweave.net/IiX6OFxiM1wb8DOSidDSn_6KVHqCpwnshUzU8RU5EN8', null, null, null, null, '2025-03-10 03:31:13.815471 +00:00'),
	(1020, 0, '7dHbWXmci3dT8UFYWYZweBLXgycu7Y3iL6trKn1Y7ARj', 'Lido Staked SOL', 'stSOL', 9, 49411.128227340000, '', null, null, null, null, '2025-03-10 03:31:13.815471 +00:00'),
	(1021, 0, '6AJcP7wuLwmRYLBNbi825wgguaPsWzPBEHcHndpRpump', 'Vine Coin', 'VINE', 6, 999986401.230257000000, 'https://ipfs.io/ipfs/QmYTnHwVqLHCYL1xrwGQLY8e1YHdXVMimkLuYeHZNNyUiX', null, null, null, null, '2025-03-10 03:31:13.815471 +00:00'),
	(1022, 0, 'JUPyiwrYJFskUPiHa7hkeR8VUtAeFoSYbKedZNsDvCN', 'Jupiter', 'JUP', 6, 6999978415.463109000000, 'https://static.jup.ag/jup/metadata.json', null, null, null, null, '2025-03-10 03:31:13.815471 +00:00'),
	(1023, 0, '27G8MtK7VtTcCHkpASjSDdkWWYfoqT6ggEuKidVJidD4', 'Jupiter Perps LP', 'JLP', 6, 384232904.234896000000, 'https://static.jup.ag/jlp/metadata.json', null, null, null, null, '2025-03-10 03:31:13.815471 +00:00'),
	(1024, 0, '6p6xgHyF7AeE6TZkSmFsko444wqoP15icUSqi2jfGiPN', 'OFFICIAL TRUMP', 'TRUMP', 6, 999999555.349323000000, 'https://arweave.net/cSCP0h2n1crjeSWE9KF-XtLciJalDNFs7Vf-Sm0NNY0', null, null, null, null, '2025-03-10 03:31:13.815471 +00:00'),
	(1025, 0, '63LfDmNb3MQ8mw9MtZ2To9bEA2M71kZUUGq5tiJxcqj9', 'GIGACHAD', 'GIGA', 5, 9603912427.397640000000, 'https://bafkreiehz3jw7547ryrb5mr54rbnqmcjsqblkcoz46v7aejszqricvnbsa.ipfs.nftstorage.link', null, null, null, null, '2025-03-10 03:31:13.815471 +00:00'),
	(1026, 0, '9BB6NFEcjBCtnNLFko2FqVQBq8HHM13kCyYcdQbgpump', 'Fartcoin', 'Fartcoin', 6, 999993848.325539000000, 'https://ipfs.io/ipfs/QmYfe8zVGHA1heej47AkBX3Nnetg2h2kqj5yymz1xyKeHb', null, null, null, null, '2025-03-10 03:31:13.815471 +00:00'),
	(1027, 0, 'FeR8VBqNRSUD5NtXAj2n3j1dAHkZHfyDktKuLXD4pump', 'jelly-my-jelly', 'jellyjelly', 6, 999978603.463378000000, 'https://ipfs.io/ipfs/QmRaah2aa24T3F2hGQCf8XefSuaNFZM2wGx2W2UnsxfLxM', null, null, null, null, '2025-03-10 03:31:13.815471 +00:00'),
	(1028, 0, '2z1p8xCEjRzpBHjXWrx4tJnz7BFL6z7NnvbCxH7bpump', 'San Chan', 'San', 6, 941971535.365609000000, 'https://ipfs.io/ipfs/QmaWBS53yhphzpiVToxXwYQQxrBzedTKrb9utKF9qtuiRP', null, null, null, null, '2025-03-10 03:31:13.815471 +00:00'),
	(1029, 0, 'DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263', 'Bonk', 'Bonk', 5, 88858081070531.638580000000, 'https://arweave.net/QPC6FYdUn-3V8ytFNuoCS85S2tHAuiDblh6u3CIZLsw', null, null, null, null, '2025-03-10 03:31:13.815471 +00:00'),
	(1030, 0, 'Dfh5DzRgSvvCFDoYc2ciTkMrbDfRKybA4SoFbPmApump', 'Pippin', 'pippin', 6, 999947390.646544000000, 'https://ipfs.io/ipfs/QmWbM38Bhhcj4vQZpSyesRSotgoqicNjfaNoMmuDGtWZ2o', null, null, null, null, '2025-03-10 03:31:13.815471 +00:00'),
	(1031, 0, 'bSo13r4TkiE4KumL71LsHTPpL2euBYLFx6h9HP3piy1', null, null, 9, 931762.846692905000, null, null, null, null, null, '2025-03-10 03:31:13.815471 +00:00'),
	(1032, 0, '9cxTc3HExqWWwzS9QKKsagoyGcaHkfGtttFqfipFgMXj', 'NEO NAZI CULT', 'CVLT', 6, 999989252.232741000000, 'https://metadata.pumployer.fun/data/aa0b61e2-4c90-4008-8027-e7990d4f09f3.json', null, null, null, null, '2025-03-10 03:31:13.815471 +00:00'),
	(1033, 0, 'E93CRVtdgxcyQUQfhwdo1BS5Vu65ge6Qa3dacf29zFba', 'Whale Farts', 'WF', 6, 999462123.200240000000, 'https://ipfs.io/ipfs/QmaJ89iSrU6MqXMFG1rPeW3CdycLbw3gMZxXN48tZNBTPt', null, null, null, null, '2025-03-10 03:31:13.815471 +00:00'),
	(1034, 0, '3zQ1XAcbSejFZNdbTBGvFGQatvViYbcwgXZ5pQ3KRRaw', 'Aiccelerate', 'AICC', 9, 1099999370.901600180000, 'https://ipfs.io/ipfs/bafkreicsxgufxzke7jdaap6g4bdsuyfruei6ejyi23s67lro2oehnawpse', null, null, null, null, '2025-03-10 03:31:13.815471 +00:00'),
	(1035, 0, '61V8vBaqAGMpgDQi4JcAwo1dmBGHsyhzodcPqnEVpump', 'AI Rig Complex', 'arc', 6, 999995340.675296000000, 'https://ipfs.io/ipfs/QmZf8F4hXkAma1fUY8ZLhbxtpdi1anGAof5FqVAjrKrYhh', null, null, null, null, '2025-03-10 03:31:13.815471 +00:00'),
	(1036, 0, '8Ki8DpuWNxu9VsS3kQbarsCWMcFGWkzzA8pUPto9zBd5', 'LOCK IN', 'LOCKIN', 9, 994368771.108111432000, 'https://ipfs.io/ipfs/QmRoK51Ez4MPzYRMQAggvREoDPfGtj1fj1D5UtrixUwnAj', null, null, null, null, '2025-03-10 03:31:13.815471 +00:00'),
	(1037, 0, 'HNNVKPEtNmGjkNCBgNu4YcNcnSHZAVP8hCg2B5Uuawzu', 'DeepSeek', 'DeepSeek', 6, 98700018729.469122000000, 'https://ipfs.io/ipfs/Qmbf595oRK1seeY4H8CeSFk3u3fR4RhvnwkvWqd8seVxN9', null, null, null, null, '2025-03-10 03:31:13.815471 +00:00'),
	(1038, 0, 'KMNo3nJsBXfcpJTVhZcXLW7RmTwTt4GVFE7suUBo9sS', 'Kamino', 'KMNO', 6, 9999986214.735164000000, 'https://cdn.kamino.finance/kamino-metadata.json', null, null, null, null, '2025-03-10 03:31:13.815471 +00:00'),
	(1039, 0, '6UZ3bcBtfaxWkMyeyh9Gv3Ud4CJYnH33baNJ5CSbvXz9', 'World Liberty Financial', 'WLFI', 6, 99355158392.358326000000, 'https://ipfs.io/ipfs/QmRdhC6uvHQHkz7dSjUMWFY1dnH7gMwRYFEW4Ma6JeJ7Ru', null, null, null, null, '2025-03-10 03:31:13.815471 +00:00'),
	(1040, 0, '6NGPnEv2nAioTXyGBq4GMUickA5WuvU83bx8rQrMpump', 'Althea AI', 'ATH', 6, 999953757.057953000000, 'https://ipfs.io/ipfs/QmRFneF5to1Ju6wdGAg7Z23cy92R2zZiZoe446tBFuXoXa', null, null, null, null, '2025-03-10 03:31:13.815471 +00:00');
        "#).await.unwrap();

		tx.commit().await.unwrap();

		let token_repo = TokenRepo::testing_no_token_info();
		let pumpfun_swap_repo = solana::pumpfun::repo::SwapRepo::testing(Box::new(NeverCalledTokenInfoLoader {}));
		let jupiter_swap_repo = solana::jupiter::repo::SwapRepo::testing(Box::new(NeverCalledTokenInfoLoader {}));

		let state = State(Arc::new(StateInner {
			token_repo: token_repo.clone(),
			address_repo: AddressRepo::new(),
			pool: pool.clone(),
			pumpfun_swap_repo,
			pumpfun_curve_repo: Default::default(),
			jupiter_swap_repo,
		}));

		index_block(state, block).await;

		let mut tx = pool.begin().await.unwrap();
		let minted_token = token_repo.get_by_mint(&mut tx, "G3TpcmEy28TbzbyjL7TQy5noZbXrFBzJ5Vw5PqhTpump").await.unwrap();
		assert_eq!(minted_token.id, 1000);
		assert_eq!(minted_token.mint, "G3TpcmEy28TbzbyjL7TQy5noZbXrFBzJ5Vw5PqhTpump");
		assert_eq!(minted_token.symbol.unwrap(), "ТURTlS");
		assert_eq!(minted_token.decimals, 6);
		assert_eq!(minted_token.supply.unwrap(), DecimalAmount::from(1_000_000_000i64));
		assert_eq!(minted_token.creator.unwrap(), 1);
		assert_eq!(minted_token.block.unwrap(), 317897944);
	})
		.await
}
