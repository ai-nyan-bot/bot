// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::model::solana::Slot;
use base::model::DecimalAmount;
use base::repo::{AddressRepo, TokenBalanceRepo, TokenRepo};
use base::test::NeverCalledTokenInfoLoader;
use common::model::{BlockTimestamp, Timestamp};
use indexer::solana::block::index_block;
use indexer::solana::state::{State, StateInner};
use solana::convert::convert_block;
use sqlx::Executor;
use std::sync::Arc;
use testing::run_test_with_pool_on_empty_db;

#[test_log::test(sqlx::test)]
async fn test_index_block_317897944() {
    run_test_with_pool_on_empty_db(|pool| async move {
		let block = serde_json::from_str(include_str!("./block_317897944.json")).unwrap();
		let block = convert_block(Slot::from(317897944), block).await.unwrap().unwrap();

		let mut tx = pool.begin().await.unwrap();
		tx.execute(r#"
insert into solana.address (address) values  (
'HcvYEizKBqExpW4uJBnEqDKFtCBUKmuLpExwzcRWdbQE');

insert into solana.token (mint, name, symbol, decimals, supply, metadata, description, image, website, creator_id, block_id, block_time) values
        ('5SGGMBxHsVPmLmj7vtQjnryx34hmhW2NBQEmtPoUpump', 'Allott', 'ALLOTT', 6, 999913624.759929000000, 'https://ipfs.io/ipfs/QmdtbA5bSYcSd4GRu72A7rvRFs2V2BtMKGGp9fTUgWPvpY', null, null, null, null, null, null),
        ('27G8MtK7VtTcCHkpASjSDdkWWYfoqT6ggEuKidVJidD4', 'Jupiter Perps LP', 'JLP', 6, 383552939.759918000000, 'https://static.jup.ag/jlp/metadata.json', null, null, null, null, null, null),
        ('J2HfKQoybUgP9uLFrzLjf32fKGQikSvoqLJpErdRpump', 'Mowgli the Chug', 'mowgli', 6, 999999152.585054000000, 'https://ipfs.io/ipfs/QmVyvBMR21tNCo98EbbKYp7K3G4ShKP8B1eQqdUM9Ma3xp', null, null, null, null, null, null),
        ('14ne24juMvzbxGcXtL65aZ1QrZgSoUjaD1BBGFcppump', 'Official Elon Coin', 'ELON', 6, 38470.561101000000, 'https://arweave.net/ik5SwlY2ybezgX2vvFnRo46Xui0gN8ryIeV5xOMWIFE', null, null, null, null, null, null),
        ('Caw4nJ1iChXLkzGxCdaaQbqPiF1oSVHapvyCZ46Zpump', 'Nico.Ai', 'NICOAI', 6, 999176545.786965000000, 'https://ipfs.io/ipfs/QmUy7xQWXjg5EVew1nJYe9ypkT8ad65XkXkRNhBSMMBeYw', null, null, null, null, null, null),
        ('FeR8VBqNRSUD5NtXAj2n3j1dAHkZHfyDktKuLXD4pump', 'jelly-my-jelly', 'jellyjelly', 6, 999976454.536214000000, 'https://ipfs.io/ipfs/QmRaah2aa24T3F2hGQCf8XefSuaNFZM2wGx2W2UnsxfLxM', null, null, null, null, null, null),
        ('EfgEGG9PxLhyk1wqtqgGnwgfVC7JYic3vC9BCWLvpump', 'Gyat Coin', 'GYAT', 6, 997528855.166366000000, 'https://ipfs.io/ipfs/QmcCs4RV1PcdPxkyCHHeSNdcLu6XR2CwwXvbdg11EC8u9r', null, null, null, null, null, null),
        ('251yuzr1SEKkn9xA94KN3JGVtsrh4ahHVuudNgSHpump', 'TRUMPIUS MAXIMUS TERMINAL', '$T.M.T', 6, 1000000000.000000000000, 'https://ipfs.io/ipfs/QmcxZm4vyXGBqAKGNPoGchxq2BNHYxHSWpU99sEvG81J4Y', null, null, null, null, null, null),
        ('9Hv8f1M7EUZgAPtZz7UuPSjydnk35qU9mREWbSdapump', 'Solazard', '$SOLZ', 6, 1000000000.000000000000, 'https://ipfs.io/ipfs/QmTG2N92KTmZLDYCJn5g8T7Kqm3jwD4Cgtu7xZo8jX4AXt', null, null, null, null, null, null),
        ('jtojtomepa8beP8AuQc6eXt5FriJwfFMwQx2v2f9mCL', 'JITO', 'JTO', 9, 999999760.843563934000, 'https://metadata.jito.network/token/jto', null, null, null, null, null, null),
        ('ED5nyyWEzpPPiWimP8vYm7sD7TD3LAt3Q3gRTWHzPJBY', 'Moo Deng', 'MOODENG', 6, 989942138.598548000000, 'https://ipfs.io/ipfs/QmWVzSC1ZTFiBYFiZZ6QivGUZ9awPJwqZECSFL1UD4gitC', null, null, null, null, null, null),
        ('6NGPnEv2nAioTXyGBq4GMUickA5WuvU83bx8rQrMpump', 'Althea AI', 'ATH', 6, 999953757.057953000000, 'https://ipfs.io/ipfs/QmRFneF5to1Ju6wdGAg7Z23cy92R2zZiZoe446tBFuXoXa', null, null, null, null, null, null),
        ('bSo13r4TkiE4KumL71LsHTPpL2euBYLFx6h9HP3piy1', null, null, 9, 926946.003779276000, null, null, null, null, null, null, null),
        ('2WTrAr7vbZpg4cEhXjaYumEEQYAWr3DmoRFdBQ5Xpump', 'Solaris AI', 'SOLARIS', 6, 1000000000.000000000000, 'https://ipfs.io/ipfs/Qmb3gF2rLW2b9z4dLT7PLW3qabGqrfjbhiZptSLU7wJupB', null, null, null, null, null, null),
        ('6XYW2Q4nvn5xNWb9z19vDnAZbykk9LQ67km37q8VYsen', '$$', '$$', 6, 948781931.316303000000, 'https://ipfs.io/ipfs/QmWhoe5H6MNpR7SFYF2AV1rNikErn4UPKEGAsX5nnnBeT3', null, null, null, null, null, null),
        ('87h4Fnrdj3GJrCaQ7CDPX2m8KZALzE2WY2TX6KtQpump', 'TRUMP 100M FOLLOWERS', '100M', 6, 995444369.642507000000, 'https://ipfs.io/ipfs/QmYMToJHNmUzAYbuUBZY2FN3EqXULdRtEGG4KYnwHDteng', null, null, null, null, null, null),
        ('KENJSUYLASHUMfHyy5o4Hp2FdNqZg1AsUPhfH2kYvEP', 'test griffain.com', 'GRIFFAIN', 6, 999870511.261257000000, 'https://arweave.net/x8B0Sv4DdQB_Tupec003Mu6HyL41DLQCQHlNMPgTHuw', null, null, null, null, null, null),
        ('EdbVEzm3xLPTH1JPmd2t8iYkWRJYC45NVE8kGgiGpump', 'bag holder', 'bagholder', 6, 998949321.150186000000, 'https://ipfs.io/ipfs/Qmbfu5pbW2w7uqLexEENRTnGka6sZ4fo5ST4sWHMx819uY', null, null, null, null, null, null),
        ('6AJcP7wuLwmRYLBNbi825wgguaPsWzPBEHcHndpRpump', 'Vine Coin', 'VINE', 6, 999985757.315535000000, 'https://ipfs.io/ipfs/QmYTnHwVqLHCYL1xrwGQLY8e1YHdXVMimkLuYeHZNNyUiX', null, null, null, null, null, null),
        ('3GY2Znk6J7Hfu7VH8WnZnYdMUU1RD8Gw7rKB1iuFpump', 'College Jackie Chan', 'LiveJackie', 6, 999998506.139485000000, 'https://ipfs.io/ipfs/QmWPKXMNs6Q18j6jBfG7cw2NArSJqD8pseRwpxDzQPudPQ', null, null, null, null, null, null),
        ('AeBESHJNBV2vbtStqLdvL3Vz6bTVnktx8h9RMgubTf8L', 'Buy $100 this token to Zillion', 'ZILLION', 6, 481702158.067282000000, 'https://fully-would-island.quicknode-ipfs.com/ipfs/QmZG4jAHDaiieM34CNxmKPq5Edky6gQBgEcVD5mMtfEDaV', null, null, null, null, null, null),
        ('AH5bPbharRJh8tnsGAHHUBTSgZhJSeHEHkjZXmZ4pump', 'pepeinatux', '$INA', 6, 999881314.822113000000, 'https://ipfs.io/ipfs/QmUSiVNe8rpprjhiRnfTNESYM81QGRHJbyX79C77TLWExH', null, null, null, null, null, null),
        ('7GCihgDB8fe6KNjn2MYtkzZcRjQy3t9GHdC8uHYmW2hr', 'POPCAT', 'POPCAT', 9, 979940308.388654039000, 'https://arweave.net/IiX6OFxiM1wb8DOSidDSn_6KVHqCpwnshUzU8RU5EN8', null, null, null, null, null, null),
        ('FxidwFhfL1byzdiBBJ2HQFpUkCjFWce7eZwCK9GYwNjK', 'Ana Destierro', 'Ana', 6, 1000000000.000000000000, 'https://ipfs.io/ipfs/QmZn2Mi8zup1axTkZL5grkURjxYrfRSEfPyi59bxuxvQjT', null, null, null, null, null, null),
        ('KMNo3nJsBXfcpJTVhZcXLW7RmTwTt4GVFE7suUBo9sS', 'Kamino', 'KMNO', 6, 9999986174.788534000000, 'https://cdn.kamino.finance/kamino-metadata.json', null, null, null, null, null, null),
        ('4YK1njyeCkBuXG6phNtidJWKCbBhB659iwGkUJx98P5Z', 'Dolan Duck', 'DOLAN', 6, 98230228.344164000000, 'https://gateway.irys.xyz/bwI-DnskHKap0Rv3I3PLe4ocspBETGnDVHNCz0e2Gt8', null, null, null, null, null, null),
        ('4PwG2hQRAFPfNTKEsVaQ6D88gHcLiHdgYhhYCdHjqLrg', 'Beracha1n', 'BERA', 6, 878500501.662043000000, 'https://i.degencdn.com/ipfs/bafkreifdgo2hfekxz5nkcizky5g533ulx6rsoymktupqchlsm3ss4rmjo4', null, null, null, null, null, null),
        ('9cxTc3HExqWWwzS9QKKsagoyGcaHkfGtttFqfipFgMXj', 'NEO NAZI CULT', 'CVLT', 6, 999909971.344523000000, 'https://metadata.pumployer.fun/data/aa0b61e2-4c90-4008-8027-e7990d4f09f3.json', null, null, null, null, null, null),
        ('foodQJAztMzX1DKpLaiounNe2BDMds5RNuPC6jsNrDG', 'Food', 'FOOD', 0, 119102541758.000000000000, 'https://galaxy.staratlas.com/items/foodQJAztMzX1DKpLaiounNe2BDMds5RNuPC6jsNrDG', null, null, null, null, null, null),
        ('8UrUDAKVPtX69BkoVHWv3b3wz5zynNuJTmgpUs9aKQHN', 'United Pup Service', 'UPS', 6, 999944823.298702000000, 'https://ipfs.io/ipfs/QmdVw9LBoEwGG7nu9DDdMe2Ysk4joVU2zRSacW8VrrMKf5', null, null, null, null, null, null),
        ('7yufwWTGKyw6SH1zF3TQLqqM1ZQZC63K6q9Abp91moon', 'MooDeng', 'MOO', 6, 1000000000.000000000000, 'https://ipfs.io/ipfs/QmUzjh7X9E21T9LY8g5aidVg5Nq5wWu6W4gnpBU9pVwE1k', null, null, null, null, null, null),
        ('8ptGRQMPej6Cr2rgSfgUYkLNsrUHxYBPhBerwRWtpump', '1st SOL Foundation Dog', 'Opie', 6, 997523521.874586000000, 'https://ipfs.io/ipfs/QmX16M1PihsHCQ4GnMggA3uAFqujq9vgSvCe9fatjpZsaq', null, null, null, null, null, null),
        ('4r4acN195EJPvKb3ovCpQSvw23Y7fVtZMdu3DLj3pump', 'Felipe The Sheriff', 'FELIPE', 6, 998307322.475007000000, 'https://ipfs.io/ipfs/QmXX4WJBNcwLE3dhQhLjovP1tzEUUujQTB28nfFSqmBwhk', null, null, null, null, null, null),
        ('5fzuPMnSeGFhUFN6szN8tqL676TDE1m4YH13ZC1LKzc2', null, null, 5, 25910840486.961580000000, null, null, null, null, null, null, null),
        ('3WPep4ufaToK1aS5s8BL9inzeUrt4DYaQCiic6ZkkC1U', 'Giko Cat', 'GIKO', 9, 9999255.766662128000, 'https://bafkreic2jpq3k6jdlmse5v3o7kddlocqilr3c37zleymztartyi4xmse2m.ipfs.nftstorage.link', null, null, null, null, null, null),
        ('AypBEaLpZdeLs3d17Qcu1heyoTjNiYpJKZWRcScwQpwm', 'Official Elon Coin', 'ELON', 6, 9997550517.041637000000, 'https://ipfs.io/ipfs/QmcPYQhG47BttbVhhCt2xmvkzgQqmuS1ugj2BnQxryMCrY', null, null, null, null, null, null),
        ('A656xYK5cTyGNEFTW31piA8NRdbbXRw3ZwBENxEffSka', 'Official Elon Musket Coin', 'MUSKET', 6, 992014933.556357000000, 'https://fully-would-island.quicknode-ipfs.com/ipfs/QmW9rXfgp5Stg3zE2kosiW44nMHHmv6uf5RzZ4PCubLra5', null, null, null, null, null, null),
        ('DKu9kykSfbN5LBfFXtNNDPaX35o4Fv6vJ9FKk7pZpump', 'Ava AI', 'AVA', 6, 999887881.534791000000, 'https://ipfs.io/ipfs/QmNVY73MUCao6bcw1bRFWZDMguYgDtUCqRNN1ERW6cg2tR', null, null, null, null, null, null),
        ('H1N8BaoMhTTpbyM5EtA4zqbKtANokZnABLrVb2KyuJCL', 'Figu', 'LF', 6, 1000000000.000000000000, 'https://i.degencdn.com/ipfs/bafkreiaq7cxeilnl5l4iyrklz2ogbihzo57nspwxk2p7eznlgdrwv35eta', null, null, null, null, null, null),
        ('9WaZS7xsJW8ggSYZegWMX9Z46ACfUqALw6RgEyFnpump', 'SENDIT', '$SENDIT', 6, 1426094.069572000000, 'https://ipfs.io/ipfs/QmSdNfNAWm9kKgsBwiHTY6Y2xZCuvHGY1LYggwsZLDLxFu', null, null, null, null, null, null),
        ('2te3SSgoyHyVk92gZRwqqrtqSB6jSMPQTK6zZnUqpump', 'Robinhood', 'HOOD', 6, 938839.329798000000, 'https://ipfs.io/ipfs/QmTPQRDXQo8KTytZyh8vPs3TnKerZkMFccSES2AivM8Mse', null, null, null, null, null, null),
        ('DriFtupJYLTosbwoN8koMbEYSx54aFAVLddWsbksjwg7', 'Drift', 'DRIFT', 6, 999999854.824421000000, 'https://metadata.drift.foundation/drift.json', null, null, null, null, null, null),
        ('6EGtU6776cVwJyRexrHkhCKxRQWs91TccJfueMQhpump', 'Jeet of the United States', 'JETUS', 6, 998301283.615037000000, 'https://ipfs.io/ipfs/QmYA9H4EVg3R21Q3bS5L12eTuKLWex4H48f11Sr2aB7z2e', null, null, null, null, null, null),
        ('2iU5qDuGoBzegJhHrkTL19t6RfR2pSWq3KEuxGKvpump', 'Seal Money Spread', 'SMS', 6, 997824584.058160000000, 'https://ipfs.io/ipfs/QmXtwLd4cDsYHrLVbZk6nJwX5PaJTxgXNo9ys3qG5AasWv', null, null, null, null, null, null),
        ('MNDEFzGvMt87ueuHvVU9VcTqsAP5b3fTGPsHuuPA5ey', 'Marinade', 'MNDE', 9, 999998581.323180127000, '', null, null, null, null, null, null),
        ('6p6xgHyF7AeE6TZkSmFsko444wqoP15icUSqi2jfGiPN', 'OFFICIAL TRUMP', 'TRUMP', 6, 999999479.923616000000, 'https://arweave.net/cSCP0h2n1crjeSWE9KF-XtLciJalDNFs7Vf-Sm0NNY0', null, null, null, null, null, null),
        ('22kDnb6yiqEh39BoRJPsdw7HFXXFsHdMjoYbtooQrQj4', 'Puffer F1nance', 'PUFFER', 6, 14384287.128362000000, 'https://i.degencdn.com/ipfs/bafkreig23wskahwe6mpk23mes7t7d6c5c2ymjiquaimedaxm5k42bja6xq', null, null, null, null, null, null),
        ('HNg5PYJmtqcmzXrv6S9zP1CDKk5BgDuyFBxbvNApump', 'Alchemist AI', 'ALCH', 6, 999988163.003343000000, 'https://ipfs.io/ipfs/Qmbe4nA5bYwmGLJ6Bgcegf8tqUHBAVSNPQmT8gXYSb8rUN', null, null, null, null, null, null),
        ('9BB6NFEcjBCtnNLFko2FqVQBq8HHM13kCyYcdQbgpump', 'Fartcoin', 'Fartcoin', 6, 999993232.396856000000, 'https://ipfs.io/ipfs/QmYfe8zVGHA1heej47AkBX3Nnetg2h2kqj5yymz1xyKeHb', null, null, null, null, null, null),
        ('2JoUZ4TuuBnS2M1o9TpTfhtmKyzKdx2Ki9z5Cv75pump', 'Monica', 'MONICA', 6, 1000000000.000000000000, 'https://ipfs.io/ipfs/QmfLKij7usrNNzMUfN76YQXDa3n55ZKrd8ZPRZYeYGpngy', null, null, null, null, null, null),
        ('Gi7gEU5KkHW1Cp42ggpShDJxaBgyxqntiumZYjgA994r', 'DINGO', 'DICC', 6, 999857290.349391000000, 'https://ipfs.io/ipfs/QmXGYa4kfEJP2ARo1i3qWouiDi2nRiXKdxVWpVwNgqAzWM', null, null, null, null, null, null),
        ('CzLSujWBLFsSjncfkh59rUFqvafWcY5tzedWJSuypump', 'Goatseus Maximus', 'GOAT', 6, 999993932.201783000000, 'https://ipfs.io/ipfs/QmcGmF4tQxsQKdF3s1mSqYyg1vNqAXou32tbWztnsnEzth', null, null, null, null, null, null),
        ('h5NciPdMZ5QCB5BYETJMYBMpVx9ZuitR6HcVjyBhood', 'Robinhood', 'HOOD', 6, 44543328323.231941000000, 'https://ipfs.io/ipfs/bafkreift5u7kzayj2ul64ex4ep442naah52cxwrwuhlz5uzt3w4vbath6m', null, null, null, null, null, null),
        ('FUAfBo2jgks6gB4Z4LfZkqSZgzNucisEHqnNebaRxM1P', 'Melania Meme', 'MELANIA', 6, 999999112.509607000000, 'https://ipfs.io/ipfs/bafkreihl6322gqmdg5hkg6vcqnuxceok5kfgespmmua7zmqlx4r4myuuu4', null, null, null, null, null, null),
        ('3iHMnc58WuVhBoEfkJKkUYDAypAe9XQKA11Luo5HkPpR', 'Fu11House', 'FH', 6, 87192114.091640000000, 'https://i.degencdn.com/ipfs/bafkreieqwvlxbmox6kg3ylcayakoea3sbspie7pst3smpkuv2yikqopfhe', null, null, null, null, null, null),
        ('HdoBnUg5bK3QB1BC4Mqz14mY5VmUufjvUMQjQmvpfSmh', 'USD C', 'USDC', 6, 999999996.813545000000, 'https://i.degencdn.com/ipfs/bafkreibb2qrr6p2jrftxhf4om5l2up5fpftzyz4alcedylqwpnynvn6uha', null, null, null, null, null, null),
        ('5MuL7uj1KKszPHaJDeAXN1mZingzV9yqs7zWmNofpump', 'Monica', 'MONICA', 6, 999434148.721893000000, 'https://ipfs.io/ipfs/QmfLKij7usrNNzMUfN76YQXDa3n55ZKrd8ZPRZYeYGpngy', null, null, null, null, null, null),
        ('3n4LK6c3FHQ4UCgz1JBCng6Ff44C2xrAnR2LzZ6mvjez', 'Fred Krueger‘s Dog', 'ALASKA', 6, 49850003.795950000000, 'https://fully-would-island.quicknode-ipfs.com/ipfs/QmZrHhmmZugHAb9YqgkQA8Jz2WfqktNzJHwHmtxPYe5a9A', null, null, null, null, null, null),
        ('61V8vBaqAGMpgDQi4JcAwo1dmBGHsyhzodcPqnEVpump', 'AI Rig Complex', 'arc', 6, 999995246.110816000000, 'https://ipfs.io/ipfs/QmZf8F4hXkAma1fUY8ZLhbxtpdi1anGAof5FqVAjrKrYhh', null, null, null, null, null, null),
        ('GzAFVCBvWfdyjWxvn1QXqSSS5caSTi62t9aCC6nV3ivD', 'USD Cine', 'USDC', 6, 999999999.998749000000, 'https://i.degencdn.com/ipfs/bafkreibgws3bl2ygbobdnzlae6l5jycvdlfzuqnfzuzfepuv4ksbckxa4e', null, null, null, null, null, null),
        ('usyYNEZMGnM56u1T9813y53pF7vztDibwEPDunXwQDW', 'TROBLOX', 'BLOX', 6, 1000000000.000000000000, 'https://ipfs.io/ipfs/Qma5XKnmMcSRxdAUNxdPGT6RrLScUci5f5HLWbEWb18oiY', null, null, null, null, null, null),
        ('HNNVKPEtNmGjkNCBgNu4YcNcnSHZAVP8hCg2B5Uuawzu', 'DeepSeek', 'DeepSeek', 6, 98636721012.114079000000, 'https://ipfs.io/ipfs/Qmbf595oRK1seeY4H8CeSFk3u3fR4RhvnwkvWqd8seVxN9', null, null, null, null, null, null),
        ('2dz9KxedMT3k3k7iZoDYqYZ5n5meZqkvFH88sjb1pump', 'FEBULLARY', 'FEBULLARY', 6, 999349124.223579000000, 'https://ipfs.io/ipfs/QmPCGMzLHMHg8D4idDed6zhUrMCYe2SrYGVjg6bTfjLY7g', null, null, null, null, null, null),
        ('CvSPBtojacL6gAXjLdaiTdJWctYd22Tj2XzNrsx8W1rH', 'S0lv Pr0toc0l', 'Solv', 8, 9000000000.000000000000, 'https://i.degencdn.com/ipfs/bafkreigzztynynw25j7wougj4grgs4zirgjh5ly7mc3njs244aimq3hfba', null, null, null, null, null, null),
        ('HZ1JovNiVvGrGNiiYvEozEVgZ58xaU3RKwX8eACQBCt3', 'Pyth Network', 'PYTH', 6, 9999987850.880983000000, 'https://arweave.net/GWKBRfaCBSiDWs0hYHe_PCEEhKoq_Bl3JlpmW5MqRnE', null, null, null, null, null, null),
        ('5LaSdS71rq1KYwPnf59A8sTJU2rMokkEzfASauAynAuQ', 'SOL', 'SOL', 8, 9000000000.000000000000, 'https://i.degencdn.com/ipfs/bafkreihemzxb5wcyuxwwy5scc4khmkvofbxypszqszsl7ddc6dovx452om', null, null, null, null, null, null),
        ('6UZ3bcBtfaxWkMyeyh9Gv3Ud4CJYnH33baNJ5CSbvXz9', 'World Liberty Financial', 'WLFI', 6, 99355158370.124580000000, 'https://ipfs.io/ipfs/QmRdhC6uvHQHkz7dSjUMWFY1dnH7gMwRYFEW4Ma6JeJ7Ru', null, null, null, null, null, null),
        ('63LfDmNb3MQ8mw9MtZ2To9bEA2M71kZUUGq5tiJxcqj9', 'GIGACHAD', 'GIGA', 5, 9603910805.824210000000, 'https://bafkreiehz3jw7547ryrb5mr54rbnqmcjsqblkcoz46v7aejszqricvnbsa.ipfs.nftstorage.link', null, null, null, null, null, null),
        ('74SBV4zDXxTRgv1pEMoECskKBkZHc2yGPnc7GYVepump', 'swarms', 'swarms', 6, 999980988.522461000000, 'https://ipfs.io/ipfs/QmTXT8EFrM7FShwENGKMPSi9Es2iXP6F1CzMDAtCJhkC6f', null, null, null, null, null, null),
        ('2z1p8xCEjRzpBHjXWrx4tJnz7BFL6z7NnvbCxH7bpump', 'San Chan', 'San', 6, 941970613.817929000000, 'https://ipfs.io/ipfs/QmaWBS53yhphzpiVToxXwYQQxrBzedTKrb9utKF9qtuiRP', null, null, null, null, null, null),
        ('DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263', 'Bonk', 'Bonk', 5, 88852750624273.299380000000, 'https://arweave.net/QPC6FYdUn-3V8ytFNuoCS85S2tHAuiDblh6u3CIZLsw', null, null, null, null, null, null),
        ('AYERA68WxMvXRa3EFkjdvciYh2pjUArB8QgsQxYSpump', 'Little Birdie Bot', 'BIRD', 6, 999317063.550428000000, 'https://ipfs.io/ipfs/QmWXLWrC5qGudL7bAfN3iaVVbTBFdDgmPtWHb2Hz5BxScg', null, null, null, null, null, null),
        ('DBRiDgJAMsM95moTzJs7M9LnkGErpbv9v6CUR1DXnUu5', 'deBridge', 'DBR', 6, 9999987964.486038000000, 'https://cdn.debridge.foundation/dbr/metadata.json', null, null, null, null, null, null),
        ('8VRSLy3d5afgeE6exCPnYBM9zD7tAwCuqPtgA6v4PWmA', 'FinancialIndependenceRetireEarly', 'FIRE', 6, 53241211.948470000000, 'https://fully-would-island.quicknode-ipfs.com/ipfs/QmZjf2k5J6tVHRBmkjYWJHbv37Ew1rKR73TRsypYzdnhUK', null, null, null, null, null, null),
        ('bZLyxghvrJFBUvuDAA9qMBdcD5sJ16x4z12nR4gpump', 'Bybit', 'BYBIT', 6, 997656189.391315000000, 'https://ipfs.io/ipfs/QmRs74Y3NuUgjSMyj5GdvonBaPKgaYGM5BWqPEBizRoahN', null, null, null, null, null, null),
        ('5mbK36SZ7J19An8jFochhQS4of8g6BwUjbeCSxBSoWdp', 'michi', '$michi', 6, 555766339.228097000000, 'https://ipfs.io/ipfs/QmaFSKo4FX43NsVETn4nPfnFrXMqcGyK4mvKuCz2Pg65ji', null, null, null, null, null, null),
        ('eL5fUxj2J4CiQsmW85k5FG9DvuQjjUoBHoQBi2Kpump', 'Unicorn Fart Dust', 'UFD', 6, 999981999.782157000000, 'https://ipfs.io/ipfs/QmSwEpRo9SRsPaFfrD65drLbusGLMY6ewRevWXF4JcWKTv', null, null, null, null, null, null),
        ('B17utvBuKjiZpBrRZrkDaKWwmNGoLN1vfXFnwhRbB1eZ', null, null, 9, 14555.800233066000, null, null, null, null, null, null, null),
        ('E93CRVtdgxcyQUQfhwdo1BS5Vu65ge6Qa3dacf29zFba', 'Whale Farts', 'WF', 6, 999462123.200240000000, 'https://ipfs.io/ipfs/QmaJ89iSrU6MqXMFG1rPeW3CdycLbw3gMZxXN48tZNBTPt', null, null, null, null, null, null),
        ('BYBEfivjEgouanqm7hTceDSDhCJtdBFo8k9r3Xtfpump', 'Bennu', 'Bennu', 6, 999938195.231148000000, 'https://ipfs.io/ipfs/QmZT8smK2v5v2usxQpLo1nrumKrC98QhNQUG3dDSqakxTx', null, null, null, null, null, null),
        ('7dHbWXmci3dT8UFYWYZweBLXgycu7Y3iL6trKn1Y7ARj', 'Lido Staked SOL', 'stSOL', 9, 49361.212208120000, '', null, null, null, null, null, null),
        ('8KtBZAaNusa2QJLnLxyXQ5mq8Bo8rhYt8rrCFzYHpump', 'GANG', 'GANG', 6, 999676833.850473000000, 'https://ipfs.io/ipfs/QmRmVpuVt2SkgRZpZV3fsNHCEBsRh6CuLZXLmEJJkU9Cb6', null, null, null, null, null, null),
        ('Dfh5DzRgSvvCFDoYc2ciTkMrbDfRKybA4SoFbPmApump', 'Pippin', 'pippin', 6, 999946697.481608000000, 'https://ipfs.io/ipfs/QmWbM38Bhhcj4vQZpSyesRSotgoqicNjfaNoMmuDGtWZ2o', null, null, null, null, null, null),
        ('AKEiWadvG2UfavHUCNTACkDd6NMF7REyw5vFP2gtpump', 'Ana Destierro', 'Ana', 6, 998771100.843943000000, 'https://ipfs.io/ipfs/QmX6399K1WwSE3TMFYqm1SiGvPBijZK2eVQ3CVYuBCsJ9Z', null, null, null, null, null, null),
        ('3RpEekjLE5cdcG15YcXJUpxSepemvq2FpmMcgo342BwC', null, null, 6, 15447525.949994000000, null, null, null, null, null, null, null),
        ('8Ki8DpuWNxu9VsS3kQbarsCWMcFGWkzzA8pUPto9zBd5', 'LOCK IN', 'LOCKIN', 9, 994367543.353352425000, 'https://ipfs.io/ipfs/QmRoK51Ez4MPzYRMQAggvREoDPfGtj1fj1D5UtrixUwnAj', null, null, null, null, null, null),
        ('D7ptz7kKS165kKYXt7ZSpZxXG6vcNAoXyq73asnDpump', 'Power b', 'Power b', 6, 1000000000.000000000000, 'https://ipfs.io/ipfs/QmYmt3NQd51NTh7rrLBusxpRqGiSG6Bq6TZW3ogdmv7hWF', null, null, null, null, null, null),
        ('CQDUbQWBDy91puhdzdiXPpbzsczoSBnQAooGS31Hpump', 'FlowKitten AI', 'FLOWKITTEN', 6, 7778050.131107000000, 'https://ipfs.io/ipfs/QmZug2sk6D6eZMBELVwMQmBwwwxGkoKHej13znp3DTeMxW', null, null, null, null, null, null),
        ('9Vjm63MkpKuiuH91wwgYNuC9uZS3xDvTULu5LCMfpump', 'Lacy', 'Lacy', 6, 1000000000.000000000000, 'https://ipfs.io/ipfs/QmQwBDBLaGvRbkL3qQUm5WqjWMvpMqLrcsYi72NDtqVmFU', null, null, null, null, null, null),
        ('7vfCXTUXx5WJV5JADk17DUJ4ksgau7utNKj4b963voxs', 'Wrapped Ether (Wormhole)', 'WETH', 8, 91455.845881900000, '', null, null, null, null, null, null),
        ('SDUsgfSZaDhhZ76U3ZgvtFiXsfnHbf2VrzYxjBZ5YbM', 'Survey Data Unit', 'SDU', 0, 1262729229.000000000000, 'https://galaxy.staratlas.com/items/SDUsgfSZaDhhZ76U3ZgvtFiXsfnHbf2VrzYxjBZ5YbM', null, null, null, null, null, null),
        ('JUPyiwrYJFskUPiHa7hkeR8VUtAeFoSYbKedZNsDvCN', 'Jupiter', 'JUP', 6, 6999978225.972046000000, 'https://static.jup.ag/jup/metadata.json', null, null, null, null, null, null),
        ('2Kk16bkuFH8dsd117feYaqPjBYrF8NC5GCM2VMyKpump', 'Coinbase', 'COINBASE', 6, 997807609.687651000000, 'https://ipfs.io/ipfs/QmSfXzypDGxoPrfXHTGZvFkgu4qZGrckejtWRrdAMXToyV', null, null, null, null, null, null),
        ('3zQ1XAcbSejFZNdbTBGvFGQatvViYbcwgXZ5pQ3KRRaw', 'Aiccelerate', 'AICC', 9, 1099999346.990371017000, 'https://ipfs.io/ipfs/bafkreicsxgufxzke7jdaap6g4bdsuyfruei6ejyi23s67lro2oehnawpse', null, null, null, null, null, null),
        ('FZN7QZ8ZUUAxMPfxYEYkH3cXUASzH8EqA6B4tyCL8f1j', null, null, 9, 248484.995735112000, null, null, null, null, null, null, null),
        ('G3TpcmEy28TbzbyjL7TQy5noZbXrFBzJ5Vw5PqhTpump', 'ТURTlS', 'ТURTlS', 6, 1000000000.000000000000, 'https://ipfs.io/ipfs/QmTBYaHRY47odx3pvXJH7hdSbpNErYhT74dz1W1CS3Likf', null, null, null, 1, 317897944, '2025-02-02 02:23:39.000000 +00:00');
        "#).await.unwrap();

		tx.commit().await.unwrap();

		let token_repo = TokenRepo::testing_no_token_info();
		let pumpfun_swap_repo = solana::pumpfun::repo::SwapRepo::testing(Box::new(NeverCalledTokenInfoLoader {}));
		let jupiter_swap_repo = solana::jupiter::repo::SwapRepo::testing(Box::new(NeverCalledTokenInfoLoader {}));

		let state = State(Arc::new(StateInner {
			token_repo: token_repo.clone(),
			address_repo: AddressRepo::new(),
			token_balance_repo: TokenBalanceRepo::new(),
			pool: pool.clone(),
			pumpfun_swap_repo,
			pumpfun_current_repo: Default::default(),
			jupiter_swap_repo,
		}));

		index_block(state, block).await;

		let mut tx = pool.begin().await.unwrap();
		let minted_token = token_repo.get_by_mint(&mut tx, "G3TpcmEy28TbzbyjL7TQy5noZbXrFBzJ5Vw5PqhTpump").await.unwrap();
		assert_eq!(minted_token.id, 1095);
		assert_eq!(minted_token.mint, "G3TpcmEy28TbzbyjL7TQy5noZbXrFBzJ5Vw5PqhTpump");
		assert_eq!(minted_token.symbol.unwrap(), "ТURTlS");
		assert_eq!(minted_token.decimals, 6);
		assert_eq!(minted_token.supply.unwrap(), DecimalAmount::from(1_000_000_000i64));
		assert_eq!(minted_token.creator.unwrap(), 1);
		assert_eq!(minted_token.block.unwrap(), 317897944);
		assert_eq!(minted_token.block_time.unwrap(), BlockTimestamp(Timestamp::from_epoch_second(1738463019).unwrap()));
	})
		.await
}
