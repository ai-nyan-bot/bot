// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::repo::{AddressRepo, TokenRepo};
use base::test::NeverCalledTokenInfoLoader;
use indexer::solana::block::index_block;
use indexer::solana::state::{State, StateInner};
use solana::convert::convert_block;
use solana::model::Slot;
use solana::repo::BalanceRepo;
use sqlx::Executor;
use std::sync::Arc;
use testing::{jupiter, pumpfun, run_test_with_pool_on_empty_db};

#[test_log::test(sqlx::test)]
async fn test_index_block_323481688() {
    run_test_with_pool_on_empty_db(|pool| async move {
		let block = serde_json::from_str(include_str!("./block_323481688.json")).unwrap();
		let block = convert_block(Slot::from(323481688), block).await.unwrap().unwrap();

		pool.acquire().await.unwrap().execute(r#"
insert into solana.token (mint, name, symbol, decimals, supply, metadata, description, image, website, creator_id, block_id, block_time) values
		('9Tji5x1zQaVMC8QxRJEhY3D3a3iBvWufRpcKGrg7pump', 'Tencent Hunyuan LLM', 'TurboS', 6, 999310433.604877000000, 'https://ipfs.io/ipfs/QmVnR3ZmKXdNNY5E68NzHnyMTB79ZbnhiTeZ4brxh6VxeN', null, null, null, null, null, null),
        ('2M1JuYot3qZsCbFpBzn2BB7Eh3ws3a85xZsFjz6Hpump', 'DirtyJeeto', 'DirtyJeeto', 6, 997731806.845019000000, 'https://ipfs.io/ipfs/QmS8G7q1MFstRBzcBbB9Ymn2qt5JhTfk9eVCaZTfadabRy', null, null, null, null, null, null),
        ('27G8MtK7VtTcCHkpASjSDdkWWYfoqT6ggEuKidVJidD4', 'Jupiter Perps LP', 'JLP', 6, 383543004.683347000000, 'https://static.jup.ag/jlp/metadata.json', null, null, null, null, null, null),
        ('6Ujm6bJEqj1CSNs3EfqVQFinvywsCymuP1isAB8spump', 'Elon Doge', 'ELONDOGE', 6, 999847064.496227000000, 'https://ipfs.io/ipfs/QmZvCKWc6n7VtAtuRaMx9XLasuZg6s6JcWnfL25yTVeGfx', null, null, null, null, null, null),
        ('4qSocbwLebAZNUH7CM2C7eEP7ta2KAZNoprM8g96hL5t', 'Tooncinator', 'TOON', 6, 81392760612.713132000000, 'https://olive-defensive-anaconda-707.mypinata.cloud/ipfs/bafkreiax2gzuj3ez53gnre4cyc3747456h2rks7knrs7muic4qe37wch54', null, null, null, null, null, null),
        ('6p6xgHyF7AeE6TZkSmFsko444wqoP15icUSqi2jfGiPN', 'OFFICIAL TRUMP', 'TRUMP', 6, 999999479.923616000000, 'https://arweave.net/cSCP0h2n1crjeSWE9KF-XtLciJalDNFs7Vf-Sm0NNY0', null, null, null, null, null, null),
        ('61V8vBaqAGMpgDQi4JcAwo1dmBGHsyhzodcPqnEVpump', 'AI Rig Complex', 'arc', 6, 999995246.110816000000, 'https://ipfs.io/ipfs/QmZf8F4hXkAma1fUY8ZLhbxtpdi1anGAof5FqVAjrKrYhh', null, null, null, null, null, null),
        ('1Qf8gESP4i6CFNWerUSDdLKJ9U1LpqTYvjJ2MM4pain', 'PAIN', 'PAIN', 6, 9999981.387429000000, 'https://arweave.net/t_fIZZYbDIy2eZ8xYFjh-jHEIpT9EtQe7UxkEJZCOMg', null, null, null, null, null, null),
        ('Ds9FdU48nD34dgtfeSLtqvc5L9LoPig7cwb7kcxbmoon', 'PumpfunDAO', 'PDAO', 6, 999956458.049839000000, 'https://ipfs.io/ipfs/QmWqgn9eSyHVbHc6CEKYrwnyiw7snZGUcgUiYGsu643kmm', null, null, null, null, null, null),
        ('CQvadZTR8vikRqqwyhvYV8YpdfCRjUCGyQwCuY4rxBQt', 'Conan', 'Conan', 6, 822787746.552672000000, 'https://gateway.irys.xyz/uBnVNjKlyO14uGrFgoZQUH5tRxjpKrPOIZEB2OMQ3J4', null, null, null, null, null, null),
        ('HZ1JovNiVvGrGNiiYvEozEVgZ58xaU3RKwX8eACQBCt3', 'Pyth Network', 'PYTH', 6, 9999987850.880983000000, 'https://arweave.net/GWKBRfaCBSiDWs0hYHe_PCEEhKoq_Bl3JlpmW5MqRnE', null, null, null, null, null, null),
        ('HNg5PYJmtqcmzXrv6S9zP1CDKk5BgDuyFBxbvNApump', 'Alchemist AI', 'ALCH', 6, 999988163.003343000000, 'https://ipfs.io/ipfs/Qmbe4nA5bYwmGLJ6Bgcegf8tqUHBAVSNPQmT8gXYSb8rUN', null, null, null, null, null, null),
        ('B9r1YcdKPg6AmTb4RaFJtucjocozVKJ3gpAcRp7PQ6kE', 'Solana Money Glitch', 'SMG', 6, 783155315.045567000000, 'https://i.degencdn.com/ipfs/bafkreie274bqaureliol2o6gbbnpoqzknpn6yvkfqb6x6dsdklzmxf53au', null, null, null, null, null, null),
        ('GUdXKe5B35QBcqa8VrPQqn8LCQrK2X1aecCQSJs3EVrS', 'Storm Money', 'STORM', 6, 974233676.811218000000, 'https://r32wrcv3bsahny4eg22o6p2hqafitodbyq4yhb6mak3re4tigf4a.arweave.net/jvVoirsMgHbjhDa07z9HgAqJuGHEOYOHzAK3EnJoMXg', null, null, null, null, null, null),
        ('h5NciPdMZ5QCB5BYETJMYBMpVx9ZuitR6HcVjyBhood', 'Robinhood', 'HOOD', 6, 44543328323.231941000000, 'https://ipfs.io/ipfs/bafkreift5u7kzayj2ul64ex4ep442naah52cxwrwuhlz5uzt3w4vbath6m', null, null, null, null, null, null),
        ('5UQAYbGFuVP9qVmj6dbWzVQUc3hj8ZsWMrmfnhBWpump', 'Duolingo New Mascot', 'ZIG', 6, 999558685.057026000000, 'https://ipfs.io/ipfs/QmYtrsBR7kjRCW1pbwrHrQ7JRVQbDYA7y4475YBYZzDqi9', null, null, null, null, null, null),
        ('tqrRt3AsuGa3aYYSvABMDrPJEWVzpAnh3p6LxXGpump', 'ponzi dot market', 'ponzi', 6, 992444454.136469000000, 'https://ipfs.io/ipfs/QmXpz274QhG9oxZxWkfXTafcZrbXPhN9NFENkfWRMbLR5B', null, null, null, null, null, null),
        ('GMi5eschjr13PtXesDK6TCtbjxq2PnEa7z11MkRf95Mb', 'UЅDС', 'UЅDС', 6, 999999999.999820000000, 'https://i.degencdn.com/ipfs/bafkreifmxh7kpiws5g4fixzqpgmozn4rtbapc2kimbvmskzn6vvmjhwttu', null, null, null, null, null, null),
        ('JhKY6d3c84QG93fdjfSm7FsZkYd7H5zisQmtJpEPUUg', 'Minik Toad', 'MINIK', 6, 998364042.508687000000, 'https://arweave.net/UULdJDDgQUejpNd6qL7f3dIA7gLk4emP1FYhXNuUBn8', null, null, null, null, null, null),
        ('9M2FAqirwVZ2x8TbsFbBkMu7AAPUyQ2DSUgEvjewpump', 'a banana on a pig', 'bananapig', 6, 1000000000.000000000000, 'https://ipfs.io/ipfs/QmUEtcwRihndyUF7U7vV8HDf3egD9ftBitu6rWyURx6G6y', null, null, null, null, null, null),
        ('AfUu4bMzvfKCevoRk1vbuySwmGEgyixvhL5A3Qrqpump', 'Popstein', 'POPSTEIN', 6, 999999999.993469000000, 'https://ipfs.io/ipfs/QmNMozK8KwAwGxchGaCGA1imTdQ5HbfPbVWapof7WmtuMC', null, null, null, null, null, null),
        ('7cUSXCVjMDDxEZVjvBTkj8yFZZGWcVf34V2WRrjapump', 'bread dog', 'bd', 6, 999999999.996894000000, 'https://ipfs.io/ipfs/QmaYgjv5V9jfdtkxk997zDFLuuWHYzvKmZJ4GqHqxegh4s', null, null, null, null, null, null),
        ('CHsPiBpxwdFudgEaQ8FYKb4hBHGWNEC2atGR4vvZ6Ber', 'LUME', 'LUME', 6, 100000000.000000000000, 'https://gateway.irys.xyz/3yBv5w5r_7syndYEZbS3joVGcvjtFQ-Ay25VpLeLZdc', null, null, null, null, null, null),
        ('698UyMjzw2KedzBxeR8PuBUUa5Tn4ZjUNfHFLrkFpump', 'growing shrek', 'gSHREK', 6, 999999999.990711000000, 'https://ipfs.io/ipfs/QmSxhmdDX1p9um2oLPoDVkMFZbDiidYqjECMXdgKATobX7', null, null, null, null, null, null),
        ('3ij4jeLgooZuBU5zLbgddR4dvtQd9AHGhuHxcYZkpump', 'IQDNV', 'IQDNV', 6, 1000000000.000000000000, 'https://ipfs.io/ipfs/Qmbj5Mjjcci3BiihTgQt6D3Z98ijtWL3adPXf3pbntnHec', null, null, null, null, null, null),
        ('2UywZrUdyqs5vDchy7fKQJKau2RVyuzBev2XKGPDSiX1', null, null, 6, 3632949123.117685000000, null, null, null, null, null, null, null),
        ('AtpUocL94CzYR1tZouFpo76QeGsUMH7kSqicaTNy7Lvz', null, null, 9, 593.298603029000, null, null, null, null, null, null, null),
        ('AMGA3Uv58u6MBhgkvenCDr8wBEwRUFEFTM1ccUukvs64', 'RED PEPPER', 'PEPPER', 6, 999900367.974174000000, 'https://gateway.irys.xyz/S-wojxrB3jzUlZg3gXHNkytk0bzk4y-U1Vnp3GbaB1U', null, null, null, null, null, null),
        ('6pKHwNCpzgZuC9o5FzvCZkYSUGfQddhUYtMyDbEVpump', 'Deer Seized by US Government', 'Baby', 6, 999931471.001370000000, 'https://ipfs.io/ipfs/QmWqX5HK33CcDSxQiF4e8jVRoo2wKuQ8TBrReAsfYgbULJ', null, null, null, null, null, null),
        ('FZ5Zeb15vikNZVDNnvgdFhzJVY1oekXQXVfydvt7pump', 'Las Vegas Sphere', 'ORBI', 6, 999356841.180159000000, 'https://ipfs.io/ipfs/Qmd4T6kfkrhfhJN4cuSdwoVrZ1aNJrad3D3h1j1iuYzi6t', null, null, null, null, null, null),
        ('DriFtupJYLTosbwoN8koMbEYSx54aFAVLddWsbksjwg7', 'Drift', 'DRIFT', 6, 999999854.824421000000, 'https://metadata.drift.foundation/drift.json', null, null, null, null, null, null),
        ('2ZLdhFsrkAtdn9Kud4SZvqchQFvn5jVCHUdJ83vumKyR', null, null, 0, 10141300.000000000000, null, null, null, null, null, null, null),
        ('BKt2FdgBahn77joeawhNidswFxfgasPYCHWghRL4AKBR', null, null, 6, 1307303813.470000000000, null, null, null, null, null, null, null),
        ('DufQbmMJrmZTtcWafRmvwLykkJVx6q2fgYk51MFYpump', 'Orbi''s Sister', 'karoliny', 6, 1000000000.000000000000, 'https://ipfs.io/ipfs/QmWpQe68MjN1HxVzu6x5U8eUuj34B7hhuk42myuH8gJDQR', null, null, null, null, null, null),
        ('PNCHtJUtaMhgwke4LXExrWXRJDpq51b2tRPdY6DWNLv', 'Ponchiqs', 'PONCH', 9, 399999999.182015666000, 'https://jade-persistent-rat-575.mypinata.cloud/ipfs/bafkreifagwlchgxsrlot3zp6lwmemhtwdujktd66zjrhavtgxvcvp42tsi', null, null, null, null, null, null),
        ('2v1VUMtmRKy81CpNgmViGVLkA1rrepNY7xk53Wtepump', 'OneRuble', 'Ruble', 6, 1000000000.000000000000, 'https://ipfs.io/ipfs/QmRgShEuYBbuHGB7bmxfjq9GLaZGBt91CUScPCuwDXiffH', null, null, null, null, null, null),
        ('CkTyKbKbcpckb7RzQLx7fgvPEpTi28UpSRetkQJdpump', 'First Shrek Currency', 'SHREKLE', 6, 1000000000.000000000000, 'https://ipfs.io/ipfs/QmdAtTbiTw5qzzEiyqVeKKvqjK4nf8PDXSTbbn5Gmpnw3w', null, null, null, null, null, null),
        ('4u4ZDoyHEDwmJBzxBdLnbzbkfBuE4jgXk6RSUobdpump', 'orbi as emoji', '🤨', 6, 1000000000.000000000000, 'https://ipfs.io/ipfs/QmRHQkSuzC4YC1g9N1b7N3JhDAzKihSnEwKZRSjXVLABj2', null, null, null, null, null, null),
        ('EGGqybynBUFyVeSZQ2L4nkqyWMSQeSR86H5oiVPjpump', 'Zenithia_AI', 'Zai', 6, 999999999.990531000000, 'https://ipfs.io/ipfs/QmNWrea6CgZVnTsrhc1h1E2JH13opXisCw81Zx85YySXaC', null, null, null, null, null, null),
        ('DeRdQAfjnG7KW9av9jy88TiUhN8yxoohGmi7J92Dpump', 'Ravelix Science AI', 'Ravelis', 6, 1000000000.000000000000, 'https://ipfs.io/ipfs/QmczfMMnn8CbfaNSixCLfg9YtUVxp45KGiZER6FjjiyajV', null, null, null, null, null, null),
        ('6NL1wXkByhr8GLDQrU7huAUBgdDBTEPgL6vqMWJjpump', 'Meme Family', 'Family', 6, 1000000000.000000000000, 'https://ipfs.io/ipfs/QmZTctnVuoTBWJi8ULsWfqzUk9LvX3uKjfLf6hke6u5itH', null, null, null, null, null, null),
        ('G9KhCMmYWvb3nnzGEEkywKiLVaNCC2Dk9hJvDZefpump', 'Zendaya as Shrek''s Daughter', 'Zhrek 5', 6, 1000000000.000000000000, 'https://ipfs.io/ipfs/QmNV63djdk79Gz3BX6RzBNWLkVaLFRcEP7EpASCfeDCDag', null, null, null, null, null, null),
        ('CFXBFXDeCe1BmNRuSfmF4i6oiS13w9dp1JnkZojNpump', 'Good boy', 'GBOY', 6, 1000000000.000000000000, 'https://ipfs.io/ipfs/QmZpdxzn72v5vvFffSJaGaXvBGwmJWK6cuYnNFcrJLv71V', null, null, null, null, null, null),
        ('CHnmtGxv5G6VBbixMwUuwj9qARYvXXLmdvjQSZSvpump', 'Doge Father', 'DogeFather', 6, 1000000000.000000000000, 'https://ipfs.io/ipfs/QmSp3rKFg2kr8id21yRsACTtMchcNPV2TvmumimTq2se8v', null, null, null, null, null, null),
        ('DN678Y6fXjjwBMPddwmvqeroqKDZopgjf27cHScopump', '2026 London Olympics Mascot', 'MELTAN', 6, 1000000000.000000000000, 'https://ipfs.io/ipfs/QmcpUsJVNURJvEg3VYSQXyVB7B7HSai1E5ZjiS4FgHjrxU', null, null, null, null, null, null),
        ('5m5ax8WHd9Jhm9tWpU6Ncehke3FSjp23vZ1kGmq3pump', 'Summer', 'SU', 6, 1000000000.000000000000, 'https://ipfs.io/ipfs/QmWN8Vag1GFmFiWRenpTDW6oGUmjcj43wF32TRLdjb5iwb', null, null, null, null, null, null),
        ('5V7sVrJPszHssASArL8sPUFgnKSxP9qxTMxWKEiCEJyn', 'Open Ai Services', 'OAS', 6, 1000000000.000000000000, 'https://ipfs.io/ipfs/QmR1TmMMAeCNm3keUNUFF9K2nySwEkrHynPFy3zMRSkejV', null, null, null, null, null, null),
        ('63DZkAzoDXmzGzn9esoWSYpMLo4YB9oPHXreHKwuu4HA', null, null, 0, 12774.000000000000, null, null, null, null, null, null, null),
        ('DhMH8oRQoAAb6poHVsvCqq3NCMj6aKUH2tGQG5Lo4bCg', null, null, 6, 1368655061.545400000000, null, null, null, null, null, null, null),
        ('A685eW8k4GpZ7vsZN1gWQMJc5oJ3kjuCHheRhDQwaag4', null, null, 0, 45535520000.000000000000, null, null, null, null, null, null, null),
        ('GV6ExHe5u6ZZdPaiRy4z5nUr3WBC74kd7iKoyeQnisy6', null, null, 6, 203447050.900000000000, null, null, null, null, null, null, null),
        ('MASS9GqtJz6ABisAxcUn3FeR4phMqH1XfG6LPKJePog', 'Biomass', 'BIOMASS', 0, 168549282731.000000000000, 'https://galaxy.staratlas.com/items/MASS9GqtJz6ABisAxcUn3FeR4phMqH1XfG6LPKJePog', null, null, null, null, null, null),
        ('foodQJAztMzX1DKpLaiounNe2BDMds5RNuPC6jsNrDG', 'Food', 'FOOD', 0, 119097198821.000000000000, 'https://galaxy.staratlas.com/items/foodQJAztMzX1DKpLaiounNe2BDMds5RNuPC6jsNrDG', null, null, null, null, null, null),
        ('ammoK8AkX2wnebQb35cDAZtTkvsXQbi82cGeTnUvvfK', 'Ammunition', 'AMMO', 0, 128714710698.000000000000, 'https://galaxy.staratlas.com/items/ammoK8AkX2wnebQb35cDAZtTkvsXQbi82cGeTnUvvfK', null, null, null, null, null, null),
        ('fueL3hBZjLLLJHiFH9cqZoozTG3XQZ53diwFPwbzNim', 'Fuel', 'FUEL', 0, 136443059798.000000000000, 'https://galaxy.staratlas.com/items/fueL3hBZjLLLJHiFH9cqZoozTG3XQZ53diwFPwbzNim', null, null, null, null, null, null),
        ('2GR7E9zYMD9AnDJMTa8U31HcLvPS5upZaRBGWi5bpump', 'FOR YOU DADDY', 'ANSEM', 6, 999939574.630827000000, 'https://ipfs.io/ipfs/QmdZDXKtuDqND9BMQJ6foqdoZLtRYcwuao71kHS9wDtxJY', null, null, null, null, null, null),
        ('6S6WYL1mQFmVxsf3ft5MEH8hzxJA1LcUDzgwdJDj3yxc', null, null, 6, 279171950.433900000000, null, null, null, null, null, null, null),
        ('GuNWJSV4k95FZdwhAcjdaPGGoh9cArc27yV4P54QwWdg', null, null, 0, 51464799.000000000000, null, null, null, null, null, null, null),
        ('ygRrmycNpMzEsqLYRpDMS9J1SHaFMG4Rm3tjkyVGVnc', null, null, 6, 26106087.300000000000, null, null, null, null, null, null, null),
        ('EV1UdC9dSz7a66hqYW5TkVe6JihSAyfEwVLwYzy1cGXz', null, null, 0, 192674000.000000000000, null, null, null, null, null, null, null),
        ('H6Jy27JyMXSB2xZymykCE4e2oFBXMxAUPW7PHwP8t4Yo', 'EVA COIN', 'EVA', 9, 985567948.278519491000, 'https://ipfs.io/ipfs/Qmcsr2TPCMfZRzSzcj4uTkRVNUvHh3ocBFCizD9yaiHX7f', null, null, null, null, null, null),
        ('Kruj63Qx9EQX9QzukLCBgx5g9AGW69gPDsSK25FRZAi', 'EnKryptedAI', 'KRAI', 6, 1000199999.221895000000, 'https://ipfs.io/ipfs/QmXi2c5yYHFvhohPa7hnkKFbEyfDquhWHARfk78TvuZgby', null, null, null, null, null, null),
        ('6Ce4DFTuRAjF16WsAjirvGeDT4sUryjRCNA6UgAfpump', 'Shrek Fart Dust', 'SFD', 6, 999999427.354008000000, 'https://ipfs.io/ipfs/QmRqLWy3QmR4p6EcvkRW2UgnWFZtJo8A8bVHMG8Cu1BknN', null, null, null, null, null, null),
        ('FrkF4GNdqKZLuzWXTk1imqMJRGK9uxCRE3PaBKZbpump', 'Metadrip', 'DRIP', 6, 999969015.358347000000, 'https://ipfs.io/ipfs/QmUAUR4wVFutwYFv6gXQgNyivRYTaoAT9KzerWRTAVKxiJ', null, null, null, null, null, null),
        ('2sNvt9tRAW29cZgj3cVmwEGLJFfqb127GKnAiEN8iBxY', 'JobSeek AI', 'JOBSEEK', 9, 99898383.933233136000, 'https://ipfs.io/ipfs/QmcaXmEGGRB6JbKR82WXBQrswHFzcx1BBve1WzquMVnHWP', null, null, null, null, null, null),
        ('ycZSyeBNgLK4rYopu9svNRQeyQvDihCRKBwSY28pump', 'SHRETARD', 'SHRETARD', 6, 999999999.990368000000, 'https://ipfs.io/ipfs/QmSEHNAcG3qXCWqHZzCuKgDwdgMjL2faWtkMJRJW5JJFcU', null, null, null, null, null, null),
        ('3rA3F3ZrTcy7QwQubQyvS9ceZ6q1HuRQ4bkEFyN46Mv9', 'MEMHASH', 'MEMHASH', 9, 993425310.769177951000, 'https://ipfs.io/ipfs/QmQ58MuaiaTo8x6sxUXmTT8twqVTr5mhgxja8RxAU6fDi4', null, null, null, null, null, null),
        ('EfeZEB8nBn6ExS7XBjknyXAyew5FbtWzhKcTTMbVpump', 'Epstein''s Black Book', 'UNREDACTED', 6, 995298687.210847000000, 'https://ipfs.io/ipfs/QmTu1y2whLreDXZiQ64ZVen6M6vmvxAntFuDct1iEyjiHG', null, null, null, null, null, null),
        ('9PWaS9T6c8sGZGv7zu1JKUUYGZS492P8iPhDoGNapump', 'Charizrad', 'CHARiZARD', 6, 1000000000.000000000000, 'https://ipfs.io/ipfs/QmPmvsF7DDSkowmNbjSAK6EBKDwGGZ76r6fAEAX9rsNfp2', null, null, null, null, null, null),
        ('BoMbSn3KcWsUe1dgz5ddJrRaM6v44fpeARNA9t7Dpump', 'XPi', 'XPi', 6, 999662967.162157000000, 'https://ipfs.io/ipfs/QmfJoHbvrEfe39oUsa8vUqtxYhfDMPjrYGNKJU9LJ8bSTo', null, null, null, null, null, null),
        ('HYDR4EPHJcDPcaLYUcNCtrXUdt1PnaN4MvE655pevBYp', 'Hydrogen', 'HYG', 0, 705790071652.000000000000, 'https://galaxy.staratlas.com/items/HYDR4EPHJcDPcaLYUcNCtrXUdt1PnaN4MvE655pevBYp', null, null, null, null, null, null),
        ('9BB6NFEcjBCtnNLFko2FqVQBq8HHM13kCyYcdQbgpump', 'Fartcoin', 'Fartcoin', 6, 999993232.012608000000, 'https://ipfs.io/ipfs/QmYfe8zVGHA1heej47AkBX3Nnetg2h2kqj5yymz1xyKeHb', null, null, null, null, null, null),
        ('DheLJ1VDUqBzmi7LSvXvsCrRu2VQ5m1FZz2uBhuuaSLb', 'JOON WIF HAT', 'JWH', 6, 998523819.631159000000, 'https://ipfs.io/ipfs/QmRc6dy9mtZtpKSRgnw972X3eDrJUcfam8tTHSQYBbFjUr', null, null, null, null, null, null),
        ('K72GZwe5MmX7WtqGRrP4wKYQkFEfhkbpkH6fnGGpump', 'Andrew Tate', 'TATE', 6, 998053720.596068000000, 'https://ipfs.io/ipfs/QmWxeP9rvaYp2xbJri9mXa9H741G8TQsu2N3zB5JTynwBF', null, null, null, null, null, null),
        ('FeR8VBqNRSUD5NtXAj2n3j1dAHkZHfyDktKuLXD4pump', 'jelly-my-jelly', 'jellyjelly', 6, 999976454.536214000000, 'https://ipfs.io/ipfs/QmRaah2aa24T3F2hGQCf8XefSuaNFZM2wGx2W2UnsxfLxM', null, null, null, null, null, null),
        ('A8C3xuqscfmyLrte3VmTqrAq8kgMASius9AFNANwpump', 'FWOG', 'FWOG', 6, 975621766.422049000000, 'https://ipfs.io/ipfs/QmVAvr3r1q2NrFHsY5fvrkJCdBniGM326U2pAvfpvgkwDR', null, null, null, null, null, null),
        ('3vM8sKL2AgptCZ3hA8hE2c8JwnDVyRqWGtyAspLppump', 'cauliflower', 'Cauli', 6, 998942003.888951000000, 'https://ipfs.io/ipfs/QmYdhox5FAfEEPniyJWGZR9pSawGmwy46KHN7PK3wSfVcL', null, null, null, null, null, null),
        ('9PR7nCP9DpcUotnDPVLUBUZKu5WAYkwrCUx9wDnSpump', 'Comedian', 'Ban', 6, 999861589.589653000000, 'https://ipfs.io/ipfs/QmdQb1U6dmKSL5os8LmFwh9hDQ7TcmDathFp1C3SdwgPzj', null, null, null, null, null, null),
        ('5oXbnF6JLwEbEc8gt91aTWyjeEAYVG21WdqK2KxeA2MR', null, null, 0, 601366000.000000000000, null, null, null, null, null, null, null),
        ('8rq8Z7hpk1mbaP61KremihYfBLyJ4hCZ4xBhBUCBC8bP', null, null, 6, 66580151.000000000000, null, null, null, null, null, null, null),
        ('5dCMQpaQdkX8u3kWvDKGsiC1fZiQritZGFSRZbd4pump', 'FIRST MEMECOIN SUICIDE', 'DANIEL1', 6, 999981199.395058000000, 'https://ipfs.io/ipfs/QmNR3zFu4Uw44W4W2tNn1rQuRKcWqSaY36v2ALwm7xhJTT', null, null, null, null, null, null),
        ('hTRDn7zE5tDHRnjj6Qms2WG1zEGv9ii6AiwfgbFpump', 'Deep AI', 'DEEP', 6, 998091669.176402000000, 'https://ipfs.io/ipfs/QmciSSYEVfCUW1oguwPR5B9oLS3vpoQRh6dLzrpTpKGfLT', null, null, null, null, null, null),
        ('EnpGPwV8YLGM6Eqjy1ASXfA8MY7FhtSet44HivTnpump', 'Just a Broke Guy', 'brokeguy', 6, 998742909.547031000000, 'https://ipfs.io/ipfs/QmYtEgySdKTyKtJqJsWD3iD9igQzhGzuYqNK9DpXrNr9FP', null, null, null, null, null, null),
        ('EJhqXKJEncSx1HJjS5ZpKdiKGGgLiRgNPvo8JZvw5Guj', null, null, 9, 479.976680520000, null, null, null, null, null, null, null),
        ('6zCZ17UYZmNjsfCc87pw2RpkxMEucHX1nks22KrHGhre', null, null, 9, 608.056073350000, null, null, null, null, null, null, null),
        ('4vZawEAwyxjuk9MbZrurGcPa2dpa92KnZWEPQfHvkCBk', 'beo the dancing hippo', 'beo', 6, 999430579.607338000000, 'https://ipfs.io/ipfs/QmUrVP9DT4xspWGeXYgfXeB97nPwqutorsgvTzUNsWfeGU', null, null, null, null, null, null),
        ('HheWD4czgzwiYvhV3YJmwY2r6Db1xv1i1pbEsnydRBQo', 'KanyeChain', 'KAYNE', 6, -8758417785465.660104000000, 'https://ipfs.io/ipfs/QmULpx4vYMyBVFHPQAWndnnH1eg8vsj8DzuqPSBeNMVjhN', null, null, null, null, null, null),
        ('8y56V29H3zNARbMrb33Nhg8ufCX1NfmXhkkfsy8Cpump', 'smolpain', 'pain', 6, 1000000000.000000000000, 'https://ipfs.io/ipfs/QmQyG6KPq86K51EmEE9jmGc5nV2gamojArxhWeUkYZvxmF', null, null, null, null, null, null),
        ('CHNjvfj21DBWNy2B8cSpKxG386T2KxNZDDUVUycPzTkP', 'JAKE', 'JAKE', 6, 4408438.926337000000, 'https://arweave.net/MGoAf6RLZIEuy7-SLYtBbOx90ETMC2mPmokSPgsq2c8', null, null, null, null, null, null),
        ('CbNV9RFVMXjBDRWnQVz4JUxifVRAdJgvL9jx1Ayhpump', 'I''m Shrekt', 'ShRekt', 6, 1000000000.000000000000, 'https://ipfs.io/ipfs/QmNu1n1qUDBzjoZya3oFV4dU6prUT4j2Z785cgDBX4a46G', null, null, null, null, null, null),
        ('CEJ6i8m4pgQWdN96a4WNorSNi8ypujCbuj3w69ytX9hb', 'Stella', 'Stella', 6, 1329911.651644000000, 'https://ipfs.io/ipfs/QmcHWKkJo8SFhFMQn5kvBhZRUf6ed2JTGaE4y4TzKxZ1St', null, null, null, null, null, null),
        ('9YsmPSnfHjqt6PxvNWkrdmMStDJJ3LLawnoEFJDqQAaH', 'Wrapped Pepe', 'WPEPE', 6, 985243588501.325037000000, 'https://gateway.irys.xyz/J4uagy1XmPBY5z8snva04ke1zGB3yHmqcfBjCBfbLGA', null, null, null, null, null, null),
        ('5gaE6hC3bn5WDxkdH9EZqbYAq2q2djpJzHAHFtUPmoon', 'Cainam Ventures', 'CAINAM', 9, 999778187.848373438000, 'https://cdn.dexscreener.com/cms/tokens/metadata/wieXokQAYhAEvVWusCpU', null, null, null, null, null, null),
        ('Eso4UACFScW5HqZSDyUFjbuBjVuJc55Rjat38NcfeuWi', 'OnlyFemes', 'OnlyFemes', 6, 6708966.688653000000, 'https://ipfs.io/ipfs/QmQFYfhBXHw6P8H3uaTrUKPuHKByJYxPX6puwK6iUAc8ut', null, null, null, null, null, null),
        ('6AJcP7wuLwmRYLBNbi825wgguaPsWzPBEHcHndpRpump', 'Vine Coin', 'VINE', 6, 999985757.315535000000, 'https://ipfs.io/ipfs/QmYTnHwVqLHCYL1xrwGQLY8e1YHdXVMimkLuYeHZNNyUiX', null, null, null, null, null, null),
        ('As3DHyZ7yHZeHTQsZUyKYXiz54iqDRq2RM4j5hc7RMEM', 'MemePal', 'MEM', 6, 999996495.008346000000, 'https://gateway.irys.xyz/e9cO8_esFMbTzIrKqVWpk736SImaFn19jbRyID02vYE', null, null, null, null, null, null),
        ('GbUNSqRw8v9ga5WN8d3UaXZQG7jcW626HjzAg6MH42eo', 'Official Elon Coin', 'ELON', 6, 9806252086.353749000000, 'https://ipfs.io/ipfs/QmcPYQhG47BttbVhhCt2xmvkzgQqmuS1ugj2BnQxryMCrY', null, null, null, null, null, null),
        ('FWfaKzeg7ZnYJHE8th1c1j6Ap8FTWtMicDeBXS1Hpump', 'Minik Toad', 'Minik', 6, 998834745.002919000000, 'https://ipfs.io/ipfs/QmU3SugX9AhhWgu3H71tgjVXwYqyHUfjF8VGD17WyN7uao', null, null, null, null, null, null),
        ('Fu4mWfg42KJRfzeFLup59CbFd1oNfHMku7d1C6N5pump', 'smol pikachu', 'smolpika', 6, 998170653.293612000000, 'https://ipfs.io/ipfs/QmWkp4KnEu69sj33rKD4Q9BqBhJaUVH98tn3ZFVFZYn3PS', null, null, null, null, null, null),
        ('CZNuju83NPJxNTPtW5V79W1L92qtE1B4H3aqWhb4psQM', 'Infinite Trump Rewards', 'ITR', 6, 999466598.276083000000, 'https://ipfs.io/ipfs/QmfXYFFF9McoHDAzvrD11iWSzchBsTj61sEH8yyDv5aHmZ', null, null, null, null, null, null),
        ('3vD5SD6VUKaqGbEz2BvVDo4fh9JwzoA2v6io9iJ2pump', 'BLOCKCOIN', '$BLOCK', 6, 999606197.534495000000, 'https://ipfs.io/ipfs/QmVDtjsdfZQySQwoCTKbPoEf1MHwZAF7VCsRXRjxLPbzas', null, null, null, null, null, null),
        ('3iQL8BFS2vE7mww4ehAqQHAsbmRNCrPxizWAT2Zfyr9y', 'Virtual Protocol', 'VIRTUAL', 9, 26939745.540791096000, 'https://ipfs.io/ipfs/bafkreifaywrymskdgtm2sw6bm5z3ou5vil5uucshhdv6o6lr67glxcv2sy', null, null, null, null, null, null),
        ('9PtYbja953Lt2ySn2LSR9dVGjAUp53KqgGgNipRWmDoj', 'Gigashrek', 'Gigashrek', 6, 1000000997.729684000000, 'https://ipfs.io/ipfs/QmP6EDVi9A9BfMqavLRe5UmejhtyeHDYaSrawvCx9FTKj3', null, null, null, null, null, null),
        ('7AjmgdaSb2MZiY3sAhJVHahxVX1LNRfLzUf9fyBxpump', 'Meme War General', 'MWG', 6, 999480854.429037000000, 'https://ipfs.io/ipfs/QmXkyFG2X2rjauv6QywW2WwXGJMYBtJXLwcJuek1QVLtcC', null, null, null, null, null, null),
        ('Gjw4VmFEe7g2S2HFe8BXXNbHXtBAauYU34BgQZPL22Ja', 'Pi Network', 'PI', 6, 99982322232.920149000000, 'https://ipfs.io/ipfs/QmRk5ZvFkP95kuGRz6HGnHb9w8EMGdUhvv9TarKGFUtSrE', null, null, null, null, null, null),
        ('DGrR3xiuWC6wgDkEQPZfm9ozHLXzmd6BDdU1mEMoknVK', 'Pi.Network', 'PI', 6, 99999999510.213965000000, 'https://ipfs.io/ipfs/QmR8QXj9mHirpXQ9HaGoqLfcopd2TkmbJtZARycsnmPkEA', null, null, null, null, null, null),
        ('4Cnk9EPnW5ixfLZatCPJjDB1PUtcRpVVgTQukm9epump', 'DADDY TATE', 'DADDY', 6, 599641140.259355000000, 'https://ipfs.io/ipfs/QmTY3L4rGs7Cu5aVsQSDaxcMMmwGk1AfeQRqi22vUBbUcE', null, null, null, null, null, null),
        ('EGFu6FyT9i5DEGkQpqGoiPBqGmbpe9GtEzX7rkzSSk9x', 'SHREK', 'SHREK', 6, 9066525843.060528000000, 'https://ipfs.io/ipfs/Qme4hEy281yXiNxjCA51S5Y6eWtXh6BoNaGLtvdSAQkiRm', null, null, null, null, null, null),
        ('AsP4pMLwSaGzWPFdKwdF6QoFDAcF6A3VqRWE8LYfpump', 'amm.pump.fun', 'amm.p.f', 6, 999687620.344026000000, 'https://ipfs.io/ipfs/QmSqzU7fmgRFGJnmYBy9jWS92EMiH3Ah3TmGk5JTYt67Lx', null, null, null, null, null, null),
        ('6BMGm3QxxSAviadBdPpHrosiSKPKH5snvkB6zTgMs9wo', 'Official Elon Coin', 'ELON', 6, 9498805537.403285000000, 'https://ipfs.io/ipfs/QmcPYQhG47BttbVhhCt2xmvkzgQqmuS1ugj2BnQxryMCrY', null, null, null, null, null, null),
        ('CP25ohjCqkQ25pjT1MPBKG7XC6srPsPabotcYe6uDBK9', 'scam here', 'SCAM', 8, 302505472.725932060000, 'https://raw.githubusercontent.com/partymonster999/probable-palm-tree/main/w7zd9l2KNUt3stc.json', null, null, null, null, null, null),
        ('HrdFQzGi7xNRmGAH4Duk1LZ8iSDbS7Vf4kqEcVN7pump', 'MANSORY', 'MNSRY', 6, 9987282066.250141000000, 'https://ipfs.io/ipfs/QmcJgfsVNvXzwTsYLCAe1mfGHMPrbng3BESn4tKKGw65B5', null, null, null, null, null, null),
        ('BKzpnzrePYoBz5x8GoACaneAS3XodkaXfVUZMs3nRFQN', 'Pi Network', 'PI', 6, 99999999966.773698000000, 'https://ipfs.io/ipfs/QmRk5ZvFkP95kuGRz6HGnHb9w8EMGdUhvv9TarKGFUtSrE', null, null, null, null, null, null),
        ('5KCJen1YymJuuLHTGRwtfE3bfxGBaHQRvKKKQsKnCPXJ', 'Ashley St. Claire', 'ASHLEY', 6, 989968721.337658000000, 'https://ipfs.io/ipfs/QmdvHBiUdGjZknm1pu5z6GwnELNNjLRoq9DSsHYvE1phFH', null, null, null, null, null, null),
        ('AySCdQWkSrcGhkpwQqn9XYdTGbrTV4UNzie6gFuBMRop', 'Farting Unicorn By Elon Musk', 'FU', 6, 9999278927.158145000000, 'https://ipfs.io/ipfs/QmSJKx7fJBr6V8JNbe8mpT2XTbtcP3xsknoN4ZwiLuXch4', null, null, null, null, null, null),
        ('FUAfBo2jgks6gB4Z4LfZkqSZgzNucisEHqnNebaRxM1P', 'Melania Meme', 'MELANIA', 6, 999999112.509607000000, 'https://ipfs.io/ipfs/bafkreihl6322gqmdg5hkg6vcqnuxceok5kfgespmmua7zmqlx4r4myuuu4', null, null, null, null, null, null),
        ('46TQ3S5QLzenuoqDHCkS4C8TmtCZGiNUnbY6fn1vbuMR', 'DEFAI', 'DEFAI', 6, 999622465.140812000000, 'https://ipfs.io/ipfs/Qmb1uzki2Lsuw7Noupag116fnJpJJv4gfUh5VMP1fuhKdg', null, null, null, null, null, null),
        ('KENJSUYLASHUMfHyy5o4Hp2FdNqZg1AsUPhfH2kYvEP', 'test griffain.com', 'GRIFFAIN', 6, 999870511.261257000000, 'https://arweave.net/x8B0Sv4DdQB_Tupec003Mu6HyL41DLQCQHlNMPgTHuw', null, null, null, null, null, null),
        ('74SBV4zDXxTRgv1pEMoECskKBkZHc2yGPnc7GYVepump', 'swarms', 'swarms', 6, 999980988.522461000000, 'https://ipfs.io/ipfs/QmTXT8EFrM7FShwENGKMPSi9Es2iXP6F1CzMDAtCJhkC6f', null, null, null, null, null, null),
        ('2Stzi7XE3btUQXaauTVB9brPAtPmGnrEDSJmp3w5VY2j', null, null, 0, 489820.000000000000, null, null, null, null, null, null, null),
        ('5Ehp2LtTRmjug39GphXhFEeguz7hGeg41N1U49wU8Kov', null, null, 6, 815147127.808000000000, null, null, null, null, null, null, null);
        "#).await.unwrap();

		let pumpfun_swap_repo = solana::pumpfun::repo::SwapRepo::testing(Box::new(NeverCalledTokenInfoLoader {}));
		let jupiter_swap_repo = solana::jupiter::repo::SwapRepo::testing(Box::new(NeverCalledTokenInfoLoader {}));

		let state = State(Arc::new(StateInner {
			pool: pool.clone(),
			address_repo: AddressRepo::new(),
			token_repo: TokenRepo::testing_no_token_info(),
			balance_repo: BalanceRepo::new(),
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
