// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::repo::{AddressRepo, TokenRepo};
use base::test::NeverCalledTokenInfoLoader;
use indexer::solana::block::index_block;
use indexer::solana::state::{State, StateInner};
use solana::model::Block;
use sqlx::Executor;
use std::sync::Arc;
use testing::{jupiter, pumpfun, run_test_with_pool_on_empty_db};

#[test_log::test(sqlx::test)]
async fn test_index_block_323481688() {
    run_test_with_pool_on_empty_db(|pool| async move {
		let block: Block = serde_json::from_str(include_str!("./block_323481688.json")).unwrap();

		pool.acquire().await.unwrap().execute(r#"
insert into solana.token (id, version, mint, name, symbol, decimals, supply, metadata, description, image, website, updated_at) values
        (1000, 0, '5UQAYbGFuVP9qVmj6dbWzVQUc3hj8ZsWMrmfnhBWpump', 'Duolingo New Mascot', 'ZIG', 6, 999559843992530, 'https://ipfs.io/ipfs/QmYtrsBR7kjRCW1pbwrHrQ7JRVQbDYA7y4475YBYZzDqi9', null, null, null, '2025-02-28 23:15:51.718564 +00:00'),
        (1001, 0, '9M2FAqirwVZ2x8TbsFbBkMu7AAPUyQ2DSUgEvjewpump', 'a banana on a pig', 'bananapig', 6, 1000000000000000, 'https://ipfs.io/ipfs/QmUEtcwRihndyUF7U7vV8HDf3egD9ftBitu6rWyURx6G6y', null, null, null, '2025-02-28 23:15:51.718564 +00:00'),
        (1002, 0, 'AfUu4bMzvfKCevoRk1vbuySwmGEgyixvhL5A3Qrqpump', 'Popstein', 'POPSTEIN', 6, 999999999993469, 'https://ipfs.io/ipfs/QmNMozK8KwAwGxchGaCGA1imTdQ5HbfPbVWapof7WmtuMC', null, null, null, '2025-02-28 23:15:51.718564 +00:00'),
        (1003, 0, '7cUSXCVjMDDxEZVjvBTkj8yFZZGWcVf34V2WRrjapump', 'bread dog', 'bd', 6, 999999999996895, 'https://ipfs.io/ipfs/QmaYgjv5V9jfdtkxk997zDFLuuWHYzvKmZJ4GqHqxegh4s', null, null, null, '2025-02-28 23:15:51.718564 +00:00'),
        (1004, 0, '698UyMjzw2KedzBxeR8PuBUUa5Tn4ZjUNfHFLrkFpump', 'growing shrek', 'gSHREK', 6, 999999999990711, 'https://ipfs.io/ipfs/QmSxhmdDX1p9um2oLPoDVkMFZbDiidYqjECMXdgKATobX7', null, null, null, '2025-02-28 23:15:51.718564 +00:00'),
        (1005, 0, '3ij4jeLgooZuBU5zLbgddR4dvtQd9AHGhuHxcYZkpump', 'IQDNV', 'IQDNV', 6, 1000000000000000, 'https://ipfs.io/ipfs/Qmbj5Mjjcci3BiihTgQt6D3Z98ijtWL3adPXf3pbntnHec', null, null, null, '2025-02-28 23:15:51.718564 +00:00'),
        (1006, 0, 'DufQbmMJrmZTtcWafRmvwLykkJVx6q2fgYk51MFYpump', 'Orbi''s Sister', 'karoliny', 6, 1000000000000000, 'https://ipfs.io/ipfs/QmWpQe68MjN1HxVzu6x5U8eUuj34B7hhuk42myuH8gJDQR', null, null, null, '2025-02-28 23:15:51.718564 +00:00'),
        (1007, 0, '2v1VUMtmRKy81CpNgmViGVLkA1rrepNY7xk53Wtepump', 'OneRuble', 'Ruble', 6, 1000000000000000, 'https://ipfs.io/ipfs/QmRgShEuYBbuHGB7bmxfjq9GLaZGBt91CUScPCuwDXiffH', null, null, null, '2025-02-28 23:15:51.718564 +00:00'),
        (1008, 0, 'CkTyKbKbcpckb7RzQLx7fgvPEpTi28UpSRetkQJdpump', 'First Shrek Currency', 'SHREKLE', 6, 1000000000000000, 'https://ipfs.io/ipfs/QmdAtTbiTw5qzzEiyqVeKKvqjK4nf8PDXSTbbn5Gmpnw3w', null, null, null, '2025-02-28 23:15:51.718564 +00:00'),
        (1009, 0, '4u4ZDoyHEDwmJBzxBdLnbzbkfBuE4jgXk6RSUobdpump', 'orbi as emoji', '🤨', 6, 1000000000000000, 'https://ipfs.io/ipfs/QmRHQkSuzC4YC1g9N1b7N3JhDAzKihSnEwKZRSjXVLABj2', null, null, null, '2025-02-28 23:15:51.718564 +00:00'),
        (1010, 0, 'EGGqybynBUFyVeSZQ2L4nkqyWMSQeSR86H5oiVPjpump', 'Zenithia_AI', 'Zai', 6, 999999999990531, 'https://ipfs.io/ipfs/QmNWrea6CgZVnTsrhc1h1E2JH13opXisCw81Zx85YySXaC', null, null, null, '2025-02-28 23:15:51.718564 +00:00'),
        (1011, 0, 'DeRdQAfjnG7KW9av9jy88TiUhN8yxoohGmi7J92Dpump', 'Ravelix Science AI', 'Ravelis', 6, 1000000000000000, 'https://ipfs.io/ipfs/QmczfMMnn8CbfaNSixCLfg9YtUVxp45KGiZER6FjjiyajV', null, null, null, '2025-02-28 23:15:51.718564 +00:00'),
        (1012, 0, '6NL1wXkByhr8GLDQrU7huAUBgdDBTEPgL6vqMWJjpump', 'Meme Family', 'Family', 6, 1000000000000000, 'https://ipfs.io/ipfs/QmZTctnVuoTBWJi8ULsWfqzUk9LvX3uKjfLf6hke6u5itH', null, null, null, '2025-02-28 23:15:51.718564 +00:00'),
        (1013, 0, 'G9KhCMmYWvb3nnzGEEkywKiLVaNCC2Dk9hJvDZefpump', 'Zendaya as Shrek''s Daughter', 'Zhrek 5', 6, 1000000000000000, 'https://ipfs.io/ipfs/QmNV63djdk79Gz3BX6RzBNWLkVaLFRcEP7EpASCfeDCDag', null, null, null, '2025-02-28 23:15:51.718564 +00:00'),
        (1014, 0, 'CFXBFXDeCe1BmNRuSfmF4i6oiS13w9dp1JnkZojNpump', 'Good boy', 'GBOY', 6, 1000000000000000, 'https://ipfs.io/ipfs/QmZpdxzn72v5vvFffSJaGaXvBGwmJWK6cuYnNFcrJLv71V', null, null, null, '2025-02-28 23:15:51.718564 +00:00'),
        (1015, 0, 'CHnmtGxv5G6VBbixMwUuwj9qARYvXXLmdvjQSZSvpump', 'Doge Father', 'DogeFather', 6, 1000000000000000, 'https://ipfs.io/ipfs/QmSp3rKFg2kr8id21yRsACTtMchcNPV2TvmumimTq2se8v', null, null, null, '2025-02-28 23:15:51.718564 +00:00'),
        (1016, 0, 'DN678Y6fXjjwBMPddwmvqeroqKDZopgjf27cHScopump', '2026 London Olympics Mascot', 'MELTAN', 6, 1000000000000000, 'https://ipfs.io/ipfs/QmcpUsJVNURJvEg3VYSQXyVB7B7HSai1E5ZjiS4FgHjrxU', null, null, null, '2025-02-28 23:15:51.718564 +00:00'),
        (1017, 0, '5m5ax8WHd9Jhm9tWpU6Ncehke3FSjp23vZ1kGmq3pump', 'Summer', 'SU', 6, 1000000000000000, 'https://ipfs.io/ipfs/QmWN8Vag1GFmFiWRenpTDW6oGUmjcj43wF32TRLdjb5iwb', null, null, null, '2025-02-28 23:15:51.718564 +00:00'),
        (1018, 0, '5V7sVrJPszHssASArL8sPUFgnKSxP9qxTMxWKEiCEJyn', 'Open Ai Services', 'OAS', 6, 1000000000000000, 'https://ipfs.io/ipfs/QmR1TmMMAeCNm3keUNUFF9K2nySwEkrHynPFy3zMRSkejV', null, null, null, '2025-02-28 23:15:51.718564 +00:00'),
        (1019, 0, '6Ce4DFTuRAjF16WsAjirvGeDT4sUryjRCNA6UgAfpump', 'Shrek Fart Dust', 'SFD', 6, 999999428118594, 'https://ipfs.io/ipfs/QmRqLWy3QmR4p6EcvkRW2UgnWFZtJo8A8bVHMG8Cu1BknN', null, null, null, '2025-02-28 23:15:51.718564 +00:00'),
        (1020, 0, 'ycZSyeBNgLK4rYopu9svNRQeyQvDihCRKBwSY28pump', 'SHRETARD', 'SHRETARD', 6, 999999999990368, 'https://ipfs.io/ipfs/QmSEHNAcG3qXCWqHZzCuKgDwdgMjL2faWtkMJRJW5JJFcU', null, null, null, '2025-02-28 23:15:51.718564 +00:00'),
        (1021, 0, '9PWaS9T6c8sGZGv7zu1JKUUYGZS492P8iPhDoGNapump', 'Charizrad', 'CHARiZARD', 6, 1000000000000000, 'https://ipfs.io/ipfs/QmPmvsF7DDSkowmNbjSAK6EBKDwGGZ76r6fAEAX9rsNfp2', null, null, null, '2025-02-28 23:15:51.718564 +00:00'),
        (1022, 0, 'DheLJ1VDUqBzmi7LSvXvsCrRu2VQ5m1FZz2uBhuuaSLb', 'JOON WIF HAT', 'JWH', 6, 999612794213487, 'https://ipfs.io/ipfs/QmRc6dy9mtZtpKSRgnw972X3eDrJUcfam8tTHSQYBbFjUr', null, null, null, '2025-02-28 23:15:51.718564 +00:00'),
        (1023, 0, '3vM8sKL2AgptCZ3hA8hE2c8JwnDVyRqWGtyAspLppump', 'cauliflower', 'Cauli', 6, 999598453328247, 'https://ipfs.io/ipfs/QmYdhox5FAfEEPniyJWGZR9pSawGmwy46KHN7PK3wSfVcL', null, null, null, '2025-02-28 23:15:51.718564 +00:00'),
        (1024, 0, '5dCMQpaQdkX8u3kWvDKGsiC1fZiQritZGFSRZbd4pump', 'FIRST MEMECOIN SUICIDE', 'DANIEL1', 6, 999981199395058, 'https://ipfs.io/ipfs/QmNR3zFu4Uw44W4W2tNn1rQuRKcWqSaY36v2ALwm7xhJTT', null, null, null, '2025-02-28 23:15:51.718564 +00:00'),
        (1025, 0, '4vZawEAwyxjuk9MbZrurGcPa2dpa92KnZWEPQfHvkCBk', 'beo the dancing hippo', 'beo', 6, 999829419769959, 'https://ipfs.io/ipfs/QmUrVP9DT4xspWGeXYgfXeB97nPwqutorsgvTzUNsWfeGU', null, null, null, '2025-02-28 23:15:51.718564 +00:00'),
        (1026, 0, '8y56V29H3zNARbMrb33Nhg8ufCX1NfmXhkkfsy8Cpump', 'smolpain', 'pain', 6, 1000000000000000, 'https://ipfs.io/ipfs/QmQyG6KPq86K51EmEE9jmGc5nV2gamojArxhWeUkYZvxmF', null, null, null, '2025-02-28 23:15:51.718564 +00:00'),
        (1027, 0, 'CbNV9RFVMXjBDRWnQVz4JUxifVRAdJgvL9jx1Ayhpump', 'I''m Shrekt', 'ShRekt', 6, 1000000000000000, 'https://ipfs.io/ipfs/QmNu1n1qUDBzjoZya3oFV4dU6prUT4j2Z785cgDBX4a46G', null, null, null, '2025-02-28 23:15:51.718564 +00:00'),
        (1028, 0, 'FWfaKzeg7ZnYJHE8th1c1j6Ap8FTWtMicDeBXS1Hpump', 'Minik Toad', 'Minik', 6, 999816911150562, 'https://ipfs.io/ipfs/QmU3SugX9AhhWgu3H71tgjVXwYqyHUfjF8VGD17WyN7uao', null, null, null, '2025-02-28 23:15:51.718564 +00:00'),
        (1029, 0, 'Fu4mWfg42KJRfzeFLup59CbFd1oNfHMku7d1C6N5pump', 'smol pikachu', 'smolpika', 6, 999829728032389, 'https://ipfs.io/ipfs/QmWkp4KnEu69sj33rKD4Q9BqBhJaUVH98tn3ZFVFZYn3PS', null, null, null, '2025-02-28 23:15:51.718564 +00:00'),
        (1030, 0, '9Tji5x1zQaVMC8QxRJEhY3D3a3iBvWufRpcKGrg7pump', 'Tencent Hunyuan LLM', 'TurboS', 6, 999796363195425, 'https://ipfs.io/ipfs/QmVnR3ZmKXdNNY5E68NzHnyMTB79ZbnhiTeZ4brxh6VxeN', null, null, null, '2025-02-28 23:15:51.718564 +00:00'),
        (1031, 0, '2M1JuYot3qZsCbFpBzn2BB7Eh3ws3a85xZsFjz6Hpump', 'DirtyJeeto', 'DirtyJeeto', 6, 998870454622329, 'https://ipfs.io/ipfs/QmS8G7q1MFstRBzcBbB9Ymn2qt5JhTfk9eVCaZTfadabRy', null, null, null, '2025-02-28 23:15:51.718564 +00:00'),
        (1032, 0, '27G8MtK7VtTcCHkpASjSDdkWWYfoqT6ggEuKidVJidD4', 'Jupiter Perps LP', 'JLP', 6, 384846276487800, 'https://static.jup.ag/jlp/metadata.json', null, null, null, '2025-02-28 23:15:51.718564 +00:00'),
        (1033, 0, '6Ujm6bJEqj1CSNs3EfqVQFinvywsCymuP1isAB8spump', 'Elon Doge', 'ELONDOGE', 6, 999995046861117, 'https://ipfs.io/ipfs/QmZvCKWc6n7VtAtuRaMx9XLasuZg6s6JcWnfL25yTVeGfx', null, null, null, '2025-02-28 23:15:51.718564 +00:00'),
        (1034, 0, '4qSocbwLebAZNUH7CM2C7eEP7ta2KAZNoprM8g96hL5t', 'Tooncinator', 'TOON', 6, 81441542304170467, 'https://olive-defensive-anaconda-707.mypinata.cloud/ipfs/bafkreiax2gzuj3ez53gnre4cyc3747456h2rks7knrs7muic4qe37wch54', null, null, null, '2025-02-28 23:15:51.718564 +00:00'),
        (1035, 0, 'GUdXKe5B35QBcqa8VrPQqn8LCQrK2X1aecCQSJs3EVrS', 'Storm Money', 'STORM', 6, 988576754160334, 'https://r32wrcv3bsahny4eg22o6p2hqafitodbyq4yhb6mak3re4tigf4a.arweave.net/jvVoirsMgHbjhDa07z9HgAqJuGHEOYOHzAK3EnJoMXg', null, null, null, '2025-02-28 23:15:51.718564 +00:00'),
        (1036, 0, 'h5NciPdMZ5QCB5BYETJMYBMpVx9ZuitR6HcVjyBhood', 'Robinhood', 'HOOD', 6, 44545458432514413, 'https://ipfs.io/ipfs/bafkreift5u7kzayj2ul64ex4ep442naah52cxwrwuhlz5uzt3w4vbath6m', null, null, null, '2025-02-28 23:15:51.718564 +00:00'),
        (1037, 0, 'JhKY6d3c84QG93fdjfSm7FsZkYd7H5zisQmtJpEPUUg', 'Minik Toad', 'MINIK', 6, 999998962505504, 'https://arweave.net/UULdJDDgQUejpNd6qL7f3dIA7gLk4emP1FYhXNuUBn8', null, null, null, '2025-02-28 23:15:51.718564 +00:00'),
        (1038, 0, '6p6xgHyF7AeE6TZkSmFsko444wqoP15icUSqi2jfGiPN', 'OFFICIAL TRUMP', 'TRUMP', 6, 999999575456861, 'https://arweave.net/cSCP0h2n1crjeSWE9KF-XtLciJalDNFs7Vf-Sm0NNY0', null, null, null, '2025-02-28 23:15:51.718564 +00:00'),
        (1039, 0, 'PNCHtJUtaMhgwke4LXExrWXRJDpq51b2tRPdY6DWNLv', 'Ponchiqs', 'PONCH', 9, 399999999587835953, 'https://jade-persistent-rat-575.mypinata.cloud/ipfs/bafkreifagwlchgxsrlot3zp6lwmemhtwdujktd66zjrhavtgxvcvp42tsi', null, null, null, '2025-02-28 23:15:51.718564 +00:00'),
        (1040, 0, 'H6Jy27JyMXSB2xZymykCE4e2oFBXMxAUPW7PHwP8t4Yo', 'EVA COIN', 'EVA', 9, 999999912248865129, 'https://ipfs.io/ipfs/Qmcsr2TPCMfZRzSzcj4uTkRVNUvHh3ocBFCizD9yaiHX7f', null, null, null, '2025-02-28 23:15:51.718564 +00:00'),
        (1041, 0, 'Kruj63Qx9EQX9QzukLCBgx5g9AGW69gPDsSK25FRZAi', 'EnKryptedAI', 'KRAI', 6, 1000199999221895, 'https://ipfs.io/ipfs/QmXi2c5yYHFvhohPa7hnkKFbEyfDquhWHARfk78TvuZgby', null, null, null, '2025-02-28 23:15:51.718564 +00:00'),
        (1042, 0, '3rA3F3ZrTcy7QwQubQyvS9ceZ6q1HuRQ4bkEFyN46Mv9', 'MEMHASH', 'MEMHASH', 9, 997700461468882060, 'https://ipfs.io/ipfs/QmQ58MuaiaTo8x6sxUXmTT8twqVTr5mhgxja8RxAU6fDi4', null, null, null, '2025-02-28 23:15:51.718564 +00:00'),
        (1043, 0, 'EnpGPwV8YLGM6Eqjy1ASXfA8MY7FhtSet44HivTnpump', 'Just a Broke Guy', 'brokeguy', 6, 999675043829273, 'https://ipfs.io/ipfs/QmYtEgySdKTyKtJqJsWD3iD9igQzhGzuYqNK9DpXrNr9FP', null, null, null, '2025-02-28 23:15:51.718564 +00:00'),
        (1044, 0, '3vD5SD6VUKaqGbEz2BvVDo4fh9JwzoA2v6io9iJ2pump', 'BLOCKCOIN', '$BLOCK', 6, 999628515770206, 'https://ipfs.io/ipfs/QmVDtjsdfZQySQwoCTKbPoEf1MHwZAF7VCsRXRjxLPbzas', null, null, null, '2025-02-28 23:15:51.718564 +00:00'),
        (1045, 0, 'Gjw4VmFEe7g2S2HFe8BXXNbHXtBAauYU34BgQZPL22Ja', 'Pi Network', 'PI', 6, 99999999883403548, 'https://ipfs.io/ipfs/QmRk5ZvFkP95kuGRz6HGnHb9w8EMGdUhvv9TarKGFUtSrE', null, null, null, '2025-02-28 23:15:51.718564 +00:00'),
        (1046, 0, '4Cnk9EPnW5ixfLZatCPJjDB1PUtcRpVVgTQukm9epump', 'DADDY TATE', 'DADDY', 6, 599643130922137, 'https://ipfs.io/ipfs/QmTY3L4rGs7Cu5aVsQSDaxcMMmwGk1AfeQRqi22vUBbUcE', null, null, null, '2025-02-28 23:15:51.718564 +00:00'),
        (1047, 0, 'AsP4pMLwSaGzWPFdKwdF6QoFDAcF6A3VqRWE8LYfpump', 'amm.pump.fun', 'amm.p.f', 6, 999687737338981, 'https://ipfs.io/ipfs/QmSqzU7fmgRFGJnmYBy9jWS92EMiH3Ah3TmGk5JTYt67Lx', null, null, null, '2025-02-28 23:15:51.718564 +00:00'),
        (1048, 0, 'BKzpnzrePYoBz5x8GoACaneAS3XodkaXfVUZMs3nRFQN', 'Pi Network', 'PI', 6, 99999999983652428, 'https://ipfs.io/ipfs/QmRk5ZvFkP95kuGRz6HGnHb9w8EMGdUhvv9TarKGFUtSrE', null, null, null, '2025-02-28 23:15:51.718564 +00:00'),
        (1049, 0, 'FrkF4GNdqKZLuzWXTk1imqMJRGK9uxCRE3PaBKZbpump', 'Metadrip', 'DRIP', 6, 99999999983652428, 'https://ipfs.io/ipfs/', null, null, null, '2025-02-28 23:15:51.718564 +00:00'),
        (1050, 0, '2sNvt9tRAW29cZgj3cVmwEGLJFfqb127GKnAiEN8iBxY', 'JobSeek AI', 'JOBSEEK', 9, 99999999983652428, 'https://ipfs.io/ipfs/', null, null, null, '2025-02-28 23:15:51.718564 +00:00');
        "#).await.unwrap();

		let pumpfun_swap_repo = solana::pumpfun::repo::SwapRepo::testing(Box::new(NeverCalledTokenInfoLoader {}));
		let jupiter_swap_repo = solana::jupiter::repo::SwapRepo::testing(Box::new(NeverCalledTokenInfoLoader {}));

		let state = State(Arc::new(StateInner {
			pool: pool.clone(),
			address_repo: AddressRepo::new(),
			token_repo: TokenRepo::testing_no_token_info(),
			pumpfun_swap_repo,
			pumpfun_current_repo: Default::default(),
			jupiter_swap_repo,
		}));

		index_block(state, block).await;

		let mut tx = pool.begin().await.unwrap();
		let count = pumpfun::count_swaps(&mut tx).await;
		assert_eq!(count, 41);

		let swaps = pumpfun::list_with_signature(&mut tx, "3QVGwMZgNUg4xoLd8eNNo6mXYNXVzro2MbKDZQ2Mz6Yogf22uswTcWZ7K5WKxGhrZccbtrV12rFVa6BGArfpFmn8").await;
		assert_eq!(swaps.len(), 0);

		let mut swaps = pumpfun::list_micro_with_signature(&mut tx, "3QVGwMZgNUg4xoLd8eNNo6mXYNXVzro2MbKDZQ2Mz6Yogf22uswTcWZ7K5WKxGhrZccbtrV12rFVa6BGArfpFmn8").await;
		assert_eq!(swaps.len(), 2);

		let swap = swaps.pop().unwrap();
		assert_eq!(swap.amount_base, "0.035724");
		assert_eq!(swap.amount_quote, "0");
		assert_eq!(swap.price, "0");
		assert!(!swap.is_buy);
		assert_eq!(swap.virtual_base_reserves, 1072373976480987);
		assert_eq!(swap.virtual_quote_reserves, 30017513238);

		let swap = swaps.pop().unwrap();
		assert_eq!(swap.amount_base, "0.035724");
		assert_eq!(swap.amount_quote, "0.000000001");
		assert_eq!(swap.price, "2.7992E-8");
		assert!(swap.is_buy);
		assert_eq!(swap.virtual_base_reserves, 1072373976445263);
		assert_eq!(swap.virtual_quote_reserves, 30017513238);

		// might be not correct - I just quickly eyeballed it
		let count = jupiter::count_swaps(&mut tx).await;
		assert_eq!(count, 27);
		let count = jupiter::count_micro_swaps(&mut tx).await;
		assert_eq!(count, 7);
		/////////////////////////

		let mut swaps = jupiter::list_with_signature(&mut tx, "74CYf6mYrv3bmAvfHfT1wQdZsSoRoUMYtN2w5fDS3CbdLSa51AZtvXD7RUHTYe5ff1TQ6H3XsaVoiebpHHB6Erm").await;
		assert_eq!(swaps.len(), 1);
		let swap = swaps.pop().unwrap();
		assert_eq!(swap.amount_base, "136.264182");
		assert_eq!(swap.amount_quote, "522.089475");
		assert_eq!(swap.price, "3.831450549492");
		assert!(swap.is_buy);
	})
		.await
}
