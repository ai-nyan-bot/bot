// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::model::solana::Slot;
use base::repo::{AddressRepo, TokenBalanceRepo, TokenRepo};
use base::test::NeverCalledTokenInfoLoader;
use indexer::solana::block::index_block;
use indexer::solana::state::{State, StateInner};
use solana::convert::convert_block;
use sqlx::Executor;
use std::ops::Deref;
use std::sync::Arc;
use testing::run_test_with_pool_on_empty_db;

#[test_log::test(sqlx::test)]
async fn test_index_block_326027759() {
    // DnLM31nU9K4kczFoAfhbU4hrK7XaAxSJfkieUqnPpump contains invalid utf-8 characters
    run_test_with_pool_on_empty_db(|pool| async move {
		let block = serde_json::from_str(include_str!("./block_326027759.json")).unwrap();
		let block = convert_block(Slot::from(326027759), block).await.unwrap().unwrap();

		pool.acquire().await.unwrap().execute(r#"
		insert into solana.address (address) values  
		('Gb4ZDCisirfKPRxKDXqXSVwSJHv9sV415Ew3zN55UaFw');

		insert into solana.token (mint, name, symbol, decimals, supply, metadata, description, image, website, creator_id, block_id, block_time) values
		('CniPCE4b3s8gSUPhUiyMjXnytrEqUrMfSsnbBjLCpump', 'PWEASE', 'pwease', 6, 999921282.014480000000, 'https://ipfs.io/ipfs/QmRnpREFBjET3wXFRTaQAqJ7YK7jiZssZDwJFQ6uHCkEUN', null, null, null, null, null, null),
        ('DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263', 'Bonk', 'Bonk', 5, 88852740913741.447980000000, 'https://arweave.net/QPC6FYdUn-3V8ytFNuoCS85S2tHAuiDblh6u3CIZLsw', null, null, null, null, null, null),
        ('3MadWqcN9cSrULn8ikDnan9mF3znoQmBPXtVy6BfSTDB', 'GrokCoin', 'GrokCoin', 6, 999960077.638156000000, 'http://76.108.0.32:11112/json/1347452137394802739.json', null, null, null, null, null, null),
        ('DupYRDMX3g79CfWHPPCmydmFkTsfZxnQiR8Tc2V4TkKh', null, null, 6, 288401492.900000000000, null, null, null, null, null, null, null),
        ('8U8jaYGkTG1CMyzXLUdcm4DxRqrVcSkXxYHiTGYKt6o7', null, null, 0, 6240429000.000000000000, null, null, null, null, null, null, null),
        ('CPBQqugWWCcyehezHf4uSJtET2kmYNqPuhyb57H5pump', 'GREED 3.   This time we all win', 'GREED 3', 6, 999943358.950066000000, 'https://ipfs.io/ipfs/QmWVo5kQGv5M6gsovtY5gY1UgzXkg3wXLLGX52UDNXfJN2', null, null, null, null, null, null),
        ('4UdXLsCXkcat78UhykL2fq3Xm7cehdSiNifeyRcJpump', 'Barron Trump', 'BTRUMP', 6, 998379425074.236335000000, 'https://file.dexlab.space/file/08b12407909145d4b57f8df79b4686f2', null, null, null, null, null, null),
        ('61V8vBaqAGMpgDQi4JcAwo1dmBGHsyhzodcPqnEVpump', 'AI Rig Complex', 'arc', 6, 999995246.110816000000, 'https://ipfs.io/ipfs/QmZf8F4hXkAma1fUY8ZLhbxtpdi1anGAof5FqVAjrKrYhh', null, null, null, null, null, null),
        ('9BB6NFEcjBCtnNLFko2FqVQBq8HHM13kCyYcdQbgpump', 'Fartcoin', 'Fartcoin', 6, 999993232.012608000000, 'https://ipfs.io/ipfs/QmYfe8zVGHA1heej47AkBX3Nnetg2h2kqj5yymz1xyKeHb', null, null, null, null, null, null),
        ('9hWn4yftfcpsEAga1jj3XutPHqTrJ8WNjELRcbk5pyZA', 'April Fools Day', 'APRIL', 6, 994300651.472768000000, 'https://bafkreihcot7aj5mha7qvldq722n5g74bjf7ed27kehitujfsxcqfaark7a.ipfs.nftstorage.link', null, null, null, null, null, null),
        ('EpvVtrD3DneLNn9Bk9EbYGpyrkUi3nvpETnPXnbTpump', 'papipepe', 'papipepe', 6, 999610583.656616000000, 'https://ipfs.io/ipfs/QmPZ933hGykrGhxa7jyvJm9sdZMw9HRm8rseQqm7NpYULK', null, null, null, null, null, null),
        ('HrLmQRKPVd8tCWKzgBgQ681ukPQhxJqM1jmF7fENpump', 'select payment method', 'ASSPAY', 6, 999743494.096813000000, 'https://ipfs.io/ipfs/QmQZ7iPvMN3ecaPVxebKo5nmK4buQPA6F4Zxe7rXG2n1K8', null, null, null, null, null, null),
        ('9B8y8fuT3mkahGQ2Y6oyJHwZs728T7xBmyktSNbZ3ZWh', '717CapitalAI', 'CAPITALAI', 6, 1062838.584494000000, 'https://ipfs.io/ipfs/QmadKiV1CaajLbe2ZJ5sjDmPr4KrFYEgHJXYcSZdrH6ygu', null, null, null, null, null, null),
        ('WGRXSegEZiEghqbSJJAWTnvL3uZnavUPyf3seZhpump', 'PENGU X PEPE', 'PENGU', 6, 999999898.568578000000, 'https://ipfs.io/ipfs/QmbDCtw3s23FkSoD2Ju4B62xWimAxEKtTiuUkdhUXwxT92', null, null, null, null, null, null),
        ('CzVv42vRwSrZX7rk3MFm1aBBhawZHKHWWZ4xRAJgpump', 'wetard pepe', 'wepe', 6, 334201.297337000000, 'https://ipfs.io/ipfs/QmQMx2gHijnBukxkxTrMiLH64LywpSFaofuPmsQaowxjZH', null, null, null, null, null, null),
        ('8zErU24XxdtywoUzCELbd6RKcGT1hTUWX4xCjyjDpump', 'AI Trader Agent', 'AIT', 6, 999999975.499214000000, 'https://ipfs.io/ipfs/QmfFM4RACpRMzkpbPHZRioZthvGNW3U2d4A59wv3ZmvfVC', null, null, null, null, null, null),
        ('8VbR1WBaJXxCGmcK9fwUUPEfPjcMymWbj5SjXF9ppump', 'SMOG', 'SMOG', 6, 999968467.541707000000, 'https://ipfs.io/ipfs/QmRK9JRCcpardHoJLFwfHi7Jx1M52KLBQBMWdwp78H7kV9', null, null, null, null, null, null),
        ('G58GaD6ZnLkWuHtUpyQ8YM36W7qkR1cjGmvuxmVPRbPJ', 'PHIL', 'PHIL', 6, 7096566.537767000000, 'https://ipfs.io/ipfs/QmYC7eHgDeZMJnTGBipdnFNJdNVNY3ZdAmucJSUM5FmAyo', null, null, null, null, null, null),
        ('AXUrY3akSc4VqeZAz1cNyx1oxXuSCKn2PDAjPXCzyFqd', null, null, 2, 91156.080000000000, null, null, null, null, null, null, null),
        ('391eBopprrYSEVjiaFkChD29FF5k1movUtjdVYkXpump', '1st US PRESIDENTIAL SCAM COIN', 'TRUMP', 6, 998581421.982881000000, 'https://ipfs.io/ipfs/QmWtKj8PNdnLfKFsJkTHdU8rQzZ9hhMzVrzkXLJo4ESWLC', null, null, null, null, null, null),
        ('GcTiQBMLQjFPK9LLKuCBchFbP2VLaQv6YABpDF6mpump', 'Justice for Wheesung', 'Wheesung', 6, 999797824.572475000000, 'https://ipfs.io/ipfs/QmWYm27h9NceuctPRsDDkrcqPbHUoHmH6P2KTZojjbdsCv', null, null, null, null, null, null),
        ('74SBV4zDXxTRgv1pEMoECskKBkZHc2yGPnc7GYVepump', 'swarms', 'swarms', 6, 999980988.522461000000, 'https://ipfs.io/ipfs/QmTXT8EFrM7FShwENGKMPSi9Es2iXP6F1CzMDAtCJhkC6f', null, null, null, null, null, null),
        ('6VkduiPDPHC1Pnnh8Y3gu9oaz79ZoLqBmdLdtpwTSxSE', 'Tesla', 'Tesla', 9, 6599999963.630724649000, 'https://arweave.net/bjDoecw-As6mZpFjBaJoIFEceK1MhZ3CXMNcRNSyWPc', null, null, null, null, null, null),
        ('GcESTNaraLpszdBzYUzqzEPWutQkErXaZH6kqXrGmoon', 'XPi', 'XPi', 9, 999638191.148291335000, 'https://cdn.dexscreener.com/cms/tokens/metadata/tKAzWnekyL2g6cSXR9MR', null, null, null, null, null, null),
        ('aM8si3WY69icvRFem5YrMYCGSWjorSrfeF57TFApump', 'xDOGE', 'xDOGE', 6, 5182864.664750000000, 'https://arweave.net/bueSG1mKfbCo99uafVOTpYYI1KzHep0Pi2JHHUzU2Oo', null, null, null, null, null, null),
        ('CNMW9P7pNBp31hdf1ij8rBtp424JsSncMGjQPeW1nQNf', 'SpaceX', 'SPACEX', 6, 9979552396.951159000000, 'https://ipfs.io/ipfs/QmcfVHcsubgDtafDcidbazDPxUdZBmB42yZ6ZTSvR8RPxZ', null, null, null, null, null, null),
        ('BZy2VXHdSEtDjy8ku95Gx8ngxLruaDbEohx1ijvX7NhM', 'Pi Network', 'PI', 6, 99999873894.277111000000, 'https://ipfs.io/ipfs/QmRk5ZvFkP95kuGRz6HGnHb9w8EMGdUhvv9TarKGFUtSrE', null, null, null, null, null, null),
        ('HTwjtGdooLeNPVkcZFrd2D4dgWo7GVvU9wYg7tpKX9t9', 'PAWS Labs', 'PAWS', 6, 99999999800.155882000000, 'https://ipfs.io/ipfs/QmR38gRQLn8sV8TBY3C5dwdLmRkDLavZ9iCNc81QN64NzX', null, null, null, null, null, null),
        ('3VYNRKgutxvdHcd4zQhMHZacdATan3tU9V5UyQ99bqtE', 'Kanye West official', 'KANYE', 6, 9959525656.905329000000, 'https://ipfs.io/ipfs/QmNiWXUueTFsDRTFYBnhm2tovfi6R2wwTkQzQJS9i9iUyV', null, null, null, null, null, null),
        ('2p6pHvQwe239kVtHby9x7FpLFMY6X93Ku9dR1QYjAGZp', 'Barron Trump', 'BTRUMP', 6, 9962570315.395049000000, 'https://ipfs.io/ipfs/QmTJpZJbNwxGiZaEsGLjSbDxpLzMpX23cz4SCKZ3aa5DUx', null, null, null, null, null, null),
        ('8qMfPj65ZiJEExANJ9WNrgfRurB1nyu1KwvfZ2feoY8b', 'test griffain.com', 'GRIFFAIN', 6, 998995020.931388000000, 'https://ipfs.io/ipfs/QmQcurDLQem3T683Sd4fCjQ2LGonphyjDYfxQB3fqVptNH', null, null, null, null, null, null),
        ('3iQL8BFS2vE7mww4ehAqQHAsbmRNCrPxizWAT2Zfyr9y', 'Virtual Protocol', 'VIRTUAL', 9, 26939745.540791096000, 'https://ipfs.io/ipfs/bafkreifaywrymskdgtm2sw6bm5z3ou5vil5uucshhdv6o6lr67glxcv2sy', null, null, null, null, null, null),
        ('GuRTFbMFbsUjoNtZVsGJppn4r2hwpLkjp83ddj7aykbC', 'Seldon Lycurgus', 'ElonSon', 6, 9906937675.146739000000, 'https://ipfs.io/ipfs/QmRp4p7uotPMaeRc3oFz3AEGKbGVAeB6smdMpXZpo1MfZz', null, null, null, null, null, null),
        ('2PyCMbcVjMCyRRZQFxeQXRjPGjUcPHordfxxm45oyfxa', 'SPX6900', 'SPX6900', 6, 999286458.247867000000, 'https://ipfs.io/ipfs/QmcRReCpy24eS9Jng5w1ZJDrD6SWezq2HRkJRrcnfbxyJy', null, null, null, null, null, null),
        ('fueL3hBZjLLLJHiFH9cqZoozTG3XQZ53diwFPwbzNim', 'Fuel', 'FUEL', 0, 136435441525.000000000000, 'https://galaxy.staratlas.com/items/fueL3hBZjLLLJHiFH9cqZoozTG3XQZ53diwFPwbzNim', null, null, null, null, null, null),
        ('HNsCuoAe8NkfznT3JTttcFff4LbAkJiiEy8QNv69iGbY', 'Barron Trump', 'BTRUMP', 6, 998727711.476972000000, 'https://ipfs.io/ipfs/QmTJpZJbNwxGiZaEsGLjSbDxpLzMpX23cz4SCKZ3aa5DUx', null, null, null, null, null, null),
        ('hntyVP6YFm1Hg25TN9WGLqM12b8TQmcknKrdu1oxWux', 'Helium Network Token', 'HNT', 8, 179867455.923178540000, 'https://shdw-drive.genesysgo.net/6tcnBSybPG7piEDShBcrVtYJDPSvGrDbVvXmXKpzBvWP/hnt.json', null, null, null, null, null, null),
        ('6zCZ17UYZmNjsfCc87pw2RpkxMEucHX1nks22KrHGhre', null, null, 9, 608.058625645000, null, null, null, null, null, null, null),
        ('FPpmfXwnr7sneQLE92X2QNG3KruMdpFkAbugJhEVansa', null, null, 9, 2207.359106864000, null, null, null, null, null, null, null),
        ('2GCTWvAEdtnjMh7tDS9cre7en98QiHYQK16yP6hQpump', 'wanksy', 'wanksy', 6, 999803955.438615000000, 'https://ipfs.io/ipfs/QmSTN8CBGTkgBRvt1Q4FubzUWpuxfc68xdvas56d4eyTdH', null, null, null, null, null, null),
        ('EJhqXKJEncSx1HJjS5ZpKdiKGGgLiRgNPvo8JZvw5Guj', null, null, 9, 479.977489145000, null, null, null, null, null, null, null),
        ('8UrFTdmWWt8ye8KY7Jbc9yJEaQekY7XZnGV7km3urC4a', 'MANSORY AI', 'MNSAI', 6, 999224257.509609000000, 'https://ipfs.io/ipfs/Qmbh8mDL6Vbwb5uezZLNRsoGmPNovopyMoaDmqchineogy', null, null, null, null, null, null),
        ('GuNWJSV4k95FZdwhAcjdaPGGoh9cArc27yV4P54QwWdg', null, null, 0, 51465099.000000000000, null, null, null, null, null, null, null),
        ('6S6WYL1mQFmVxsf3ft5MEH8hzxJA1LcUDzgwdJDj3yxc', null, null, 6, 279443018.783900000000, null, null, null, null, null, null, null),
        ('H3GDXubBg7VxYeAXoJe1wwomtyBDZa74WHhnUrr2jxUT', null, null, 0, 284555600.000000000000, null, null, null, null, null, null, null),
        ('Ei1V6fVHunDQ7UHredBstwgwKgM1sTt4LXpZtt36kgUE', null, null, 6, 310925602.690000000000, null, null, null, null, null, null, null),
        ('ECuNBJCGxUAe2BQRxZiEKvmru3uzGBZoLanUTFsqR5Zq', null, null, 0, 23991600.000000000000, null, null, null, null, null, null, null),
        ('ApUUa8FYWCFCAaVTmhY4mpJv4qTkYCJTszQdAvUy7Jkx', null, null, 6, 286406567.660000000000, null, null, null, null, null, null, null),
        ('SDUsgfSZaDhhZ76U3ZgvtFiXsfnHbf2VrzYxjBZ5YbM', 'Survey Data Unit', 'SDU', 0, 1262686729.000000000000, 'https://galaxy.staratlas.com/items/SDUsgfSZaDhhZ76U3ZgvtFiXsfnHbf2VrzYxjBZ5YbM', null, null, null, null, null, null),
        ('foodQJAztMzX1DKpLaiounNe2BDMds5RNuPC6jsNrDG', 'Food', 'FOOD', 0, 119083962532.000000000000, 'https://galaxy.staratlas.com/items/foodQJAztMzX1DKpLaiounNe2BDMds5RNuPC6jsNrDG', null, null, null, null, null, null),
        ('B6sV248kSsj6n72osn3Wcuz87JX3RFMD7FZpgwdYGQTm', null, null, 0, 38939400.000000000000, null, null, null, null, null, null, null),
        ('5DPKMXmf9WK1C6N1MoJLWjYApiP4KR8zNf1oofevGEub', null, null, 6, 249120411.390000000000, null, null, null, null, null, null, null),
        ('AR2SmDfEbV838SXgDitHmBEdYUTxGCBs8GubSCWfpump', 'wetard twump', 'wetard', 6, 999677973.862329000000, 'https://ipfs.io/ipfs/QmemWMVkXuGW17Yu35jpsofQhcdpTq1wSGGk23nTLSLL7L', null, null, null, null, null, null),
        ('8s1tWaoroV3wdee1MkQ4BoEYFPG8RVppujtJFwTTQQZZ', 'BRIDGE', 'BRIDGE', 6, 998774982.669012000000, 'https://ipfs.io/ipfs/QmWqxSYLfKB627RAUtAzDAYg8iSCdM4u3oP51JJJ4AKhMk', null, null, null, null, null, null),
        ('D8bxLkUqwQVopFY4mn78E16wDbgy6BgjFjyMJBFnpump', 'INDIA X COINBASE', 'JEETBASE', 6, 999233762.062154000000, 'https://ipfs.io/ipfs/QmPhCm3dgNYETXRnHkbm4JHtqGmC84WhPmnQ9WGGyW4PiH', null, null, null, null, null, null),
        ('6v4tmwad1nqV5Kyt9mNtYtRxK88HG1eLZJ487T4sRcq4', 'ELON WUSK', 'WLON', 6, 999998698.984779000000, 'https://fully-would-island.quicknode-ipfs.com/ipfs/QmZKRQsrzMpKXpGrsvFPHtfAQsNyDstRBAXuBxpbkDyebV', null, null, null, null, null, null),
        ('FK5teYaPuVAQUStCoWTfY966WEx1vf8S14eNmbS9pump', 'wetard ye', 'wye', 6, 999999834.826162000000, 'https://ipfs.io/ipfs/QmQrFf8LfdPDERyrKGTVu4pp1puJSDKDp9WyrS5ZuSGTKv', null, null, null, null, null, null),
        ('2bW2fdEzuGhGFvcmxGUaHuTB7LPYWNmioacuSrweYGX8', 'Bubblemaps', 'BMT', 6, 98289229285.770407000000, 'https://ipfs.io/ipfs/Qme9J9fnZokiv3AvZzAQ3WiUTY5C22ZBEjm1EcxHH5emzH', null, null, null, null, null, null),
        ('JDME4c1i9YdCCqwG65r4u5yCTNF8rJqm2tMuntN3pump', 'dwumstick', 'dwumstick', 6, 999999998.512661000000, 'https://ipfs.io/ipfs/QmPUsLJJdHvDaPxWPUjK11n8b8vXaPHJs7D81qR4X1eFft', null, null, null, null, null, null),
        ('DBRiDgJAMsM95moTzJs7M9LnkGErpbv9v6CUR1DXnUu5', 'deBridge', 'DBR', 6, 9999987964.486038000000, 'https://cdn.debridge.foundation/dbr/metadata.json', null, null, null, null, null, null),
        ('4QPC1PDwhroryitdpGLa5kucGaVc7QLkoVkkNA1dpump', 'billy the ai fish', 'billy', 6, 999848321.434083000000, 'https://ipfs.io/ipfs/QmXuzLTbSYsozXUrY8Y6NDpanxCKzVaxkk75uVfQniF4ku', null, null, null, null, null, null),
        ('G1pcv7hEvUavFKHmespp3KsJXrrmrnpSG9qour87pump', 'SPACES MENTAL HEALTH TALK', 'SPACES', 6, 1000000000.000000000000, 'https://ipfs.io/ipfs/QmbDkFPsff3Leh11PCjpxS6MKdNzfVa4TxnBuLkYLLts8L', null, null, null, null, null, null),
        ('METAewgxyPbgwsseH8T16a39CQ5VyVxZi9zXiDPY18m', 'Metaplex Token', 'MPLX', 6, 999983154.974160000000, 'https://arweave.net/7BzVsHRrEH0ldNOCCM4_E00BiAYuJP_EQiqvcEYz3YY', null, null, null, null, null, null),
        ('FGYgFJSxZTGzaLwzUL9YZqK2yUZ8seofCwGq8BPEw4o8', null, null, 9, 2198.922607885000, null, null, null, null, null, null, null),
        ('GdAX1L7jNsMmfN3kcCijk774aE2UtARV8frTozeUWT2E', null, null, 0, 27547600.000000000000, null, null, null, null, null, null, null),
        ('J7yjhCLdftzL95kGetry8pyX4eXn4Tjjh7KoWo599Sry', null, null, 6, 170368042.310000000000, null, null, null, null, null, null, null),
        ('3fii1QntX93D5HNNjX33kBVcnW8b8HYXhNDneP6vpump', 'we’re toast', 'toast', 6, 997972262.512523000000, 'https://ipfs.io/ipfs/QmPGDnLUJMYPYk4jCnqQ2Mu2eYTZrmDK1TKAVxa3F4pztj', null, null, null, null, null, null),
        ('5xt8uuStpThtSX1PqHeiJuWtXZFpnkrrTtHDK1srXuL8', null, null, 0, 1863892000.000000000000, null, null, null, null, null, null, null),
        ('6ZfjZUd2pgmcevfL6aLTGzpTsLoNyi8pxuboDUnrVf23', null, null, 6, 180105519.000000000000, null, null, null, null, null, null, null),
        ('DnLM31nU9K4kczFoAfhbU4hrK7XaAxSJfkieUqnPpump', 'The forbidden pear', 'PEAR', 6, 1000000000.000000000000, 'https://ipfs.io/ipfs/Qmf1resBrzqymt2pPCNfFAPFFpod7WhExX1waWqxCi7ai6', null, null, null, 1, 0, '2025-03-11 10:13:53.000000 +00:00');
        "#).await.unwrap();

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

		let token = token_repo.get_by_mint(&mut tx, "DnLM31nU9K4kczFoAfhbU4hrK7XaAxSJfkieUqnPpump").await.unwrap();
		assert_eq!(token.id, 1069);
		assert_eq!(token.mint, "DnLM31nU9K4kczFoAfhbU4hrK7XaAxSJfkieUqnPpump");
		assert_eq!(token.name.unwrap(), "The forbidden pear");
		assert_eq!(token.symbol.unwrap(), "PEAR");
		assert_eq!(token.decimals, 6);
		assert_eq!(token.supply.unwrap(), 1_000_000_000);
		assert_eq!(token.block.unwrap(), 326027759);
		assert_eq!(token.block_time.unwrap().deref(), "2025-03-11 10:13:53.0 +00:00:00");
	})
		.await
}
