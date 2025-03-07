// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::model::Transaction;
use crate::parse::{log_and_return_parse_error, ParseError, ParseResult, Parser};
use crate::pumpfun::model::Instruction;
use base::model::{Mint, PublicKey};
use common::model::Timestamp;
use common::ByteReader;
use solana_sdk::pubkey::Pubkey;

pub struct PumpFunParser {}

impl PumpFunParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for PumpFunParser {
    fn default() -> Self {
        Self::new()
    }
}

impl Parser<Vec<Instruction>> for PumpFunParser {
    fn parse(&self, tx: &Transaction) -> ParseResult<Vec<Instruction>> {
        let mut result = vec![];

        for inner in &tx.inner_instructions {
            for instruction in &inner.instructions {
                let data = &instruction.instruction.data;
                if data.len() > 16 {
                    let reader = ByteReader::new(data);
                    reader.seek(8)?; // skip anchor method identifier
                    let disc = reader.read_range(8)?;

                    if disc == SWAP_DISCRIMINANT {
                        match parse_swap(&reader) {
                            Err(err) => {
                                return Err(log_and_return_parse_error(err, &tx.signature, "swap"))
                            }
                            Ok(instr) => result.push(instr),
                        }
                    } else if disc == CREATE_DISCRIMINANT {
                        match parse_create(&reader) {
                            Err(err) => {
                                return Err(log_and_return_parse_error(
                                    err,
                                    &tx.signature,
                                    "create",
                                ))
                            }
                            Ok(instr) => result.push(instr),
                        }
                    }
                }
            }
        }
        Ok(result)
    }
}

const CREATE_DISCRIMINANT: [u8; 8] = [27, 114, 169, 77, 222, 235, 99, 118];
const SWAP_DISCRIMINANT: [u8; 8] = [189, 219, 127, 211, 78, 230, 97, 238];

fn parse_create(reader: &ByteReader) -> ParseResult<Instruction> {
    Ok(Instruction::Create {
        name: String::from_utf8_lossy(reader.read_variable_length::<u32>()?)
            .to_string()
            .into(),
        symbol: String::from_utf8_lossy(reader.read_variable_length::<u32>()?)
            .to_string()
            .into(),
        uri: String::from_utf8_lossy(reader.read_variable_length::<u32>()?)
            .to_string()
            .into(),
        mint: Pubkey::try_from(reader.read_range(32)?).unwrap().into(),
        bonding_curve: Pubkey::try_from(reader.read_range(32)?).unwrap().into(),
        user: Pubkey::try_from(reader.read_range(32)?).unwrap().into(),
    })
}

fn parse_swap(reader: &ByteReader) -> ParseResult<Instruction> {
    Ok(Instruction::Swap {
        mint: reader
            .read_range(32)
            .map(|d| Pubkey::try_from(d).ok())?
            .map(|d| Mint::from(d))
            .ok_or(ParseError::DecodingFailed)?,
        sol_amount: reader.read_u64()?.into(),
        token_amount: reader.read_u64()?.into(),
        is_buy: reader.read_u8()? == 1,
        user: reader
            .read_range(32)
            .map(|d| Pubkey::try_from(d).ok())?
            .map(|d| PublicKey::from(d))
            .ok_or(ParseError::DecodingFailed)?,
        timestamp: Timestamp::from_epoch_second(reader.read_u64()? as i64)
            .map_err(|_| ParseError::DecodingFailed)?,
        virtual_sol_reserves: reader.read_u64()?.into(),
        virtual_token_reserves: reader.read_u64()?.into(),
    })
}

#[cfg(test)]
mod tests {
    use crate::convert::convert_transaction;
    use crate::model::Transaction;
    use crate::parse::Parser;
    use crate::pumpfun::model::Instruction;
    use crate::pumpfun::PumpFunParser;
    use lazy_static::lazy_static;
    use solana_transaction_status::EncodedTransactionWithStatusMeta;
    use std::collections::HashMap;

    #[test]
    fn test_create() {
        let test_instance = PumpFunParser::new();
        let tx = transaction("iurqm3R8LYed6Y5fJcozXRZFg54LSoeVaabi3zk1arA1ULtUSjV66gBUidfiduVrJpFB8C7z2CaCucvk8DuFEcK");

        let mut result = test_instance.parse(&tx).unwrap();
        assert_eq!(result.len(), 2);

        let Instruction::Swap {
            mint,
            sol_amount,
            token_amount,
            is_buy,
            user,
            timestamp,
            virtual_sol_reserves,
            virtual_token_reserves,
        } = result.pop().unwrap()
        else {
            panic!()
        };
        assert_eq!(mint, "G3TpcmEy28TbzbyjL7TQy5noZbXrFBzJ5Vw5PqhTpump");
        assert_eq!(sol_amount, 800000147);
        assert_eq!(token_amount, 27870134831168);
        assert!(is_buy);
        assert_eq!(user, "HcvYEizKBqExpW4uJBnEqDKFtCBUKmuLpExwzcRWdbQE");
        assert_eq!(timestamp.to_string(), "2025-02-02T02:23:39Z");
        assert_eq!(virtual_sol_reserves, 30800000147);
        assert_eq!(virtual_token_reserves, 1045129865168832);

        let Instruction::Create {
            name,
            symbol,
            uri,
            mint,
            bonding_curve,
            user,
        } = result.pop().unwrap()
        else {
            panic!()
        };
        assert_eq!(name, "ТURTlS");
        assert_eq!(symbol, "ТURTlS");
        assert_eq!(
            uri,
            "https://ipfs.io/ipfs/QmTBYaHRY47odx3pvXJH7hdSbpNErYhT74dz1W1CS3Likf"
        );
        assert_eq!(mint, "G3TpcmEy28TbzbyjL7TQy5noZbXrFBzJ5Vw5PqhTpump");
        assert_eq!(
            bonding_curve,
            "B8wno3ipF3v1p59SHCZsUfjXwpv98bL7aJHXbo382nA6"
        );
        assert_eq!(user, "HcvYEizKBqExpW4uJBnEqDKFtCBUKmuLpExwzcRWdbQE");
    }

    #[test]
    fn test_swap_sell() {
        let test_instance = PumpFunParser::new();
        let tx = transaction("2RqhBZykXDPG6qt5fDRJujKuZL9yqAXxQuMkJ4JC9u9fAJEe774dBVNi4E8UpAbdWB47GpBm1avug1a6VGNN3Ujv");

        let mut result = test_instance.parse(&tx).unwrap();
        assert_eq!(result.len(), 1);

        let Instruction::Swap {
            mint,
            sol_amount,
            token_amount,
            is_buy,
            user,
            timestamp,
            virtual_sol_reserves,
            virtual_token_reserves,
        } = result.pop().unwrap()
        else {
            panic!()
        };
        assert_eq!(mint, "5iA1jhWN6kJRZaASzNZ4K7vvwiR3XykUiJKiAgWSpump");
        assert_eq!(sol_amount, 212700228);
        assert_eq!(token_amount, 967931120978);
        assert!(!is_buy);
        assert_eq!(user, "7PQ3nyAJHXiFQd5c8HgRBMYLF748MQKgq3uYfTuFioHX");
        assert_eq!(timestamp.to_string(), "2025-02-01T07:31:30Z");
        assert_eq!(virtual_sol_reserves, 83998799473);
        assert_eq!(virtual_token_reserves, 383219762808254);
    }

    #[test]
    fn test_swap_with_multiple_transfers() {
        let test_instance = PumpFunParser::new();
        let tx = transaction("AUvnh9oF5xEAG2f2ccZ8vbaKNcHwQf4CcD5Bas8HuAw8qXd26B5XxJkJYLmjVYyyVzxmwUp9AzZeDZqm4JRYE7X");

        let mut result = test_instance.parse(&tx).unwrap();
        assert_eq!(result.len(), 1);

        let Instruction::Swap {
            mint,
            sol_amount,
            token_amount,
            is_buy,
            user,
            timestamp,
            virtual_sol_reserves,
            virtual_token_reserves,
        } = result.pop().unwrap()
        else {
            panic!()
        };

        assert_eq!(mint, "CdX4emxnT1y1cZqDcTrmFBDPhEnmyWooAKSwJoqbpump");
        assert_eq!(sol_amount, 300000053);
        assert_eq!(token_amount, 1040025448826);
        assert!(is_buy);
        assert_eq!(user, "EtHx7bqNL5MRs2oVwyxtEXWDQ5L257EZkAcGVzfjDYuB");
        assert_eq!(timestamp.to_string(), "2025-02-01T18:15:23Z");
        assert_eq!(virtual_sol_reserves, 96510644837);
        assert_eq!(virtual_token_reserves, 333538337869300);
    }

    lazy_static! {
        static ref transactions: HashMap<String, String>  = HashMap::from([
            ("2RqhBZykXDPG6qt5fDRJujKuZL9yqAXxQuMkJ4JC9u9fAJEe774dBVNi4E8UpAbdWB47GpBm1avug1a6VGNN3Ujv".to_string(), r#"{"blockTime":1738395090,"meta":{"computeUnitsConsumed":35888,"err":null,"fee":234049,"innerInstructions":[{"index":2,"instructions":[{"accounts":[4,3,0],"data":"3TJqkRW2LjsV","programIdIndex":11,"stackHeight":2},{"accounts":[12],"data":"2K7nL28PxCW8ejnyCeuMpbWJwVv2AdENBThdFVahYH4gHwS8ffKuawgsQZCcVo7ATBYE6DBXodvTi6JfnuBjoVRmMGgz12aCFPeAvgLHbraRDLCNvwkHvJmEgHoWhu4QrsDRToN4F7Vc4dkaMvtptZEkDkpoTKXs8hog8RyvhjaLGFHpMPhjfvSS3gtw","programIdIndex":6,"stackHeight":2}]}],"loadedAddresses":{"readonly":[],"writable":[]},"logMessages":["Program ComputeBudget111111111111111111111111111111 invoke [1]","Program ComputeBudget111111111111111111111111111111 success","Program ComputeBudget111111111111111111111111111111 invoke [1]","Program ComputeBudget111111111111111111111111111111 success","Program 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P invoke [1]","Program log: Instruction: Sell","Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [2]","Program log: Instruction: Transfer","Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 4645 of 14442 compute units","Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success","Program 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P invoke [2]","Program 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P consumed 2132 of 6131 compute units","Program 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P success","Program data: vdt/007mYe5F+cBmWEXkSxdITRvl3MspkqmLFsEA/P1WUE2p8W2nv0SMrQwAAAAAUs0wXeEAAAAAXuL1wVTmRkTr0jSqCE7uhvTbBRbqh6TAeX0ESLdyfH7SzZ1nAAAAAHH2t44TAAAAvkXVTYlcAQBxSpSSDAAAAL6twgH4XQAA","Program 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P consumed 35588 of 37382 compute units","Program 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P success"],"postBalances":[871586799,90246585773504,54000031393,2039280,2039280,1,1141440,246642259,1461600,1,731913600,934087680,122100014],"postTokenBalances":[{"accountIndex":3,"mint":"5iA1jhWN6kJRZaASzNZ4K7vvwiR3XykUiJKiAgWSpump","owner":"84jMmWjvPzAo9hrQD7A1VTMCVHyT6oJpN1znKqBB5TLE","programId":"TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA","uiTokenAmount":{"amount":"310219762808254","decimals":6,"uiAmount":310219762.808254,"uiAmountString":"310219762.808254"}},{"accountIndex":4,"mint":"5iA1jhWN6kJRZaASzNZ4K7vvwiR3XykUiJKiAgWSpump","owner":"7PQ3nyAJHXiFQd5c8HgRBMYLF748MQKgq3uYfTuFioHX","programId":"TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA","uiTokenAmount":{"amount":"0","decimals":6,"uiAmount":null,"uiAmountString":"0"}}],"preBalances":[661247622,90246583646502,54212731621,2039280,2039280,1,1141440,246642259,1461600,1,731913600,934087680,122100014],"preTokenBalances":[{"accountIndex":3,"mint":"5iA1jhWN6kJRZaASzNZ4K7vvwiR3XykUiJKiAgWSpump","owner":"84jMmWjvPzAo9hrQD7A1VTMCVHyT6oJpN1znKqBB5TLE","programId":"TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA","uiTokenAmount":{"amount":"309251831687276","decimals":6,"uiAmount":309251831.687276,"uiAmountString":"309251831.687276"}},{"accountIndex":4,"mint":"5iA1jhWN6kJRZaASzNZ4K7vvwiR3XykUiJKiAgWSpump","owner":"7PQ3nyAJHXiFQd5c8HgRBMYLF748MQKgq3uYfTuFioHX","programId":"TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA","uiTokenAmount":{"amount":"967931120978","decimals":6,"uiAmount":967931.120978,"uiAmountString":"967931.120978"}}],"rewards":[],"status":{"Ok":null}},"slot":317728984,"transaction":["AUdwCvIf2iaf1/ebrZhkK1E/I9oNeLD7j7feQuzpDt7rrs2OfJgBUPlsb/MXMlcQpLRwQgKYyMJyzCBPPTH/dAWAAQAIDV7i9cFU5kZE69I0qghO7ob02wUW6oekwHl9BEi3cnx+rRHmpPwpRKT6glG++BVCbhv7KMa2ZGZ3YHxq2fVmpkZo9nIApxrY/zAHsw3MmRtcQDkFUSXzYCz2PuY7TDyVg8oRvWQG5j8TPzNLh9V4PEG5aY7u1gAE/HrKpjo/eXbNfjCfZPDyiD7i83GMnBloszD3oh8TXxyVLSmwHkkYriADBkZv5SEXMv/srbpyw5vnvIzlu8X3EmssQ5s6QAAAAAFW4PaTZlrPRNsVaL8XW6pRicuX9dL/O2VdK7b9bRiwOoZeae4PVIDKvPZjV+TcLxjVjUXB6nSJ+zcj2Xk8cqZF+cBmWEXkSxdITRvl3MspkqmLFsEA/P1WUE2p8W2nvwAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAjJclj04kifG7PRApFI4NgwtaE5na/xCEBI572Nvp+FkG3fbh12Whk9nL4UbO63msHLSF7V9bN5E6jPWFfv8AqazxNusB/BxOiD0jyLWESrWaN/Zq3VfF6aw7U+BZ01xkvqGvo6WfRCJSyZdHO1LdI1/rwIw0QqwWOd4a2/GYabUDBQAFAjKTAAAFAAkD7r9cAAAAAAAGDAcBCAIDBAAJCgsMBhgz5oWkAX+DrVLNMF3hAAAAeJlNDAAAAAAA","base64"],"version":0}"#.to_string()),
            ("AUvnh9oF5xEAG2f2ccZ8vbaKNcHwQf4CcD5Bas8HuAw8qXd26B5XxJkJYLmjVYyyVzxmwUp9AzZeDZqm4JRYE7X".to_string(),  r#"{"blockTime":1738433723,"meta":{"computeUnitsConsumed":41729,"err":null,"fee":490000,"innerInstructions":[{"index":2,"instructions":[{"accounts":[4,3,2],"data":"3a2G86DXss9y","programIdIndex":10,"stackHeight":2},{"accounts":[1,2],"data":"3Bxs49q36ceXVnET","programIdIndex":6,"stackHeight":2},{"accounts":[1,5],"data":"3Bxs4Z6oyhaczjLK","programIdIndex":6,"stackHeight":2},{"accounts":[13],"data":"2K7nL28PxCW8ejnyCeuMpbXPqwK3esjCTUDEUvs6eq6M2PEFdBQQh2RiGwrHMtMy2Fnvgc17PN1y7338y73eFF2eBALTUWyGw8rqjx9mGg1DeiFJergEJW96b1isCRwE5BibJ9qUdXUBMRGk9ni4QL998GqG78JQXZwXWrPHtc5u6na1fBrx8cvzbsSF","programIdIndex":7,"stackHeight":2}]}],"loadedAddresses":{"readonly":[],"writable":[]},"logMessages":["Program ComputeBudget111111111111111111111111111111 invoke [1]","Program ComputeBudget111111111111111111111111111111 success","Program ComputeBudget111111111111111111111111111111 invoke [1]","Program ComputeBudget111111111111111111111111111111 success","Program 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P invoke [1]","Program log: Instruction: Buy","Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [2]","Program log: Instruction: Transfer","Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 4645 of 95177 compute units","Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success","Program 11111111111111111111111111111111 invoke [2]","Program 11111111111111111111111111111111 success","Program 11111111111111111111111111111111 invoke [2]","Program 11111111111111111111111111111111 success","Program 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P invoke [2]","Program 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P consumed 2132 of 82607 compute units","Program 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P success","Program data: vdt/007mYe6sy3UlsHyaIu95wcR/CMrplERVXJTQXDcih/eV8DyO7zWj4REAAAAAevFYJvIAAAABzku3jVnwMN9yCa+NKtiCb2H+lK+t03K+3OywzQ6bGq67ZJ5nAAAAAGWSe3gWAAAA9M0l8lkvAQBl5ld8DwAAAPQ1E6bIMAAA","Program 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P consumed 41429 of 119700 compute units","Program 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P success"],"postBalances":[14645986952,2087440573,66511876757,2039280,2039280,96110923119084,1,1141440,1,1009200,934087680,246642259,1461600,122100014],"postTokenBalances":[{"accountIndex":3,"mint":"CdX4emxnT1y1cZqDcTrmFBDPhEnmyWooAKSwJoqbpump","owner":"EtHx7bqNL5MRs2oVwyxtEXWDQ5L257EZkAcGVzfjDYuB","programId":"TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA","uiTokenAmount":{"amount":"1474207452502","decimals":6,"uiAmount":1474207.452502,"uiAmountString":"1474207.452502"}},{"accountIndex":4,"mint":"CdX4emxnT1y1cZqDcTrmFBDPhEnmyWooAKSwJoqbpump","owner":"GN2G6AZ48LoRdJkQDbVpooBiYmrA75QDDTeZqYVuQWF","programId":"TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA","uiTokenAmount":{"amount":"260538337869300","decimals":6,"uiAmount":260538337.8693,"uiAmountString":"260538337.8693"}}],"preBalances":[14646476952,2390440626,66211876704,2039280,2039280,96110920119084,1,1141440,1,1009200,934087680,246642259,1461600,122100014],"preTokenBalances":[{"accountIndex":3,"mint":"CdX4emxnT1y1cZqDcTrmFBDPhEnmyWooAKSwJoqbpump","owner":"EtHx7bqNL5MRs2oVwyxtEXWDQ5L257EZkAcGVzfjDYuB","programId":"TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA","uiTokenAmount":{"amount":"434182003676","decimals":6,"uiAmount":434182.003676,"uiAmountString":"434182.003676"}},{"accountIndex":4,"mint":"CdX4emxnT1y1cZqDcTrmFBDPhEnmyWooAKSwJoqbpump","owner":"GN2G6AZ48LoRdJkQDbVpooBiYmrA75QDDTeZqYVuQWF","programId":"TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA","uiTokenAmount":{"amount":"261578363318126","decimals":6,"uiAmount":261578363.318126,"uiAmountString":"261578363.318126"}}],"rewards":[],"status":{"Ok":null}},"slot":317825228,"transaction":["AggtLohV6seSnC7t7dblVHyjVYDDYOJ8Vpn6X0vO3VR7GEIixTDrNL/NTzSesrDQOdSPi7Nv2YAPYrHx956xSQaQWIxVCvVSv0rEuoTlgnkp3c9x7PFWIw7ghyerNI4949CaHhmZrCHENPX8xlbbAOelchlAK8jdhrWf2y7uodMEgAIACA7c2mlfcEwRWJFuCGW4XLdNN1FBBjS9MRoU366j/xgnDc5Lt41Z8DDfcgmvjSrYgm9h/pSvrdNyvtzssM0OmxquA+97QiuRpMagJwJP8+WI5OKPYBKu4JrIc9x9J1u4FhxVGEj6g+11qv1/mr5P+L48uo5N3fs71myVy6lp6Ol3aHEnyWGlXf6PhVDkrhQHJlGbFokSjUm5DHj7NKm7dMBqrRHmpPwpRKT6glG++BVCbhv7KMa2ZGZ3YHxq2fVmpkYAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAFW4PaTZlrPRNsVaL8XW6pRicuX9dL/O2VdK7b9bRiwAwZGb+UhFzL/7K26csOb57yM5bvF9xJrLEObOkAAAAAGp9UXGSxcUSGMyUw9SvF/WNruCJuh/UTj29mKAAAAAAbd9uHXZaGT2cvhRs7reawctIXtX1s3kTqM9YV+/wCpOoZeae4PVIDKvPZjV+TcLxjVjUXB6nSJ+zcj2Xk8cqasy3UlsHyaIu95wcR/CMrplERVXJTQXDcih/eV8DyO76zxNusB/BxOiD0jyLWESrWaN/Zq3VfF6aw7U+BZ01xkLheJuPh7ujHgweSBQM+HgI4uzCVkHyP7E654p9Paf6YDCAAFAsDUAQAIAAkDAAk9AAAAAAAHDAsFDAIEAwEGCgkNBxhmBj0SAdrr6nrxWCbyAAAAwITGEgAAAAAA","base64"],"version":0}"#.to_string()),
            ("iurqm3R8LYed6Y5fJcozXRZFg54LSoeVaabi3zk1arA1ULtUSjV66gBUidfiduVrJpFB8C7z2CaCucvk8DuFEcK".to_string(),  r#"{"blockTime":1738463019,"meta":{"computeUnitsConsumed":198292,"err":null,"fee":1885000,"innerInstructions":[{"index":2,"instructions":[{"accounts":[0,1],"data":"11114XtYk9gGfZoo968fyjNUYQJKf9gdmkGoaoBpzFv4vyaSMBn3VKxZdv7mZLzoyX5YNC","programIdIndex":8,"stackHeight":2},{"accounts":[1],"data":"2zt6UCCHp66bJGRS4G7bTsjdxFh6FQ9sBEyRfGyPQKxYisAw","programIdIndex":13,"stackHeight":2},{"accounts":[0,2],"data":"11112npZeiggj74jpdjnyaoXKQznZSuR59vRrSV7vxdg9g2eRJ4seDwRwaQLDUqBNSEnrB","programIdIndex":8,"stackHeight":2},{"accounts":[0,7,2,1,8,13],"data":"1","programIdIndex":16,"stackHeight":2},{"accounts":[1],"data":"84eT","programIdIndex":13,"stackHeight":3},{"accounts":[0,7],"data":"11119os1e9qSs2u7TsThXqkBSRVFxhmYaFKFZ1waB2X7armDmvK3p5GmLdUxYdg3h7QSrL","programIdIndex":8,"stackHeight":3},{"accounts":[7],"data":"P","programIdIndex":13,"stackHeight":3},{"accounts":[7,1],"data":"6XGeTjFFcXuBr5ZExNMmCxeAvSiVxYLbpHDTP6NiwKvb4","programIdIndex":13,"stackHeight":3},{"accounts":[6,1,12,0,12,8],"data":"6fbbw1whoYveHX9VWKuSUqDgvWCEhUcsCPuArPCvaYiTVhdcCfqWgCNbPYdrggwd6WCEdhaLKHPsQQtkEbpzoeW3jrY2mRBKxytDi1YRk3Yns66MJahCDpovuPTqSnaGT5FrsJi5Bu","programIdIndex":14,"stackHeight":2},{"accounts":[0,6],"data":"3Bxs4EM3hQgDpNyd","programIdIndex":8,"stackHeight":3},{"accounts":[6],"data":"9krTDGKLJBg7SB59","programIdIndex":8,"stackHeight":3},{"accounts":[6],"data":"SYXsBkG6yKW2wWDcW8EDHR6D3P82bKxJGPpM65DD8nHqBfMP","programIdIndex":8,"stackHeight":3},{"accounts":[1,7,12],"data":"6ApXSNCamGdm","programIdIndex":13,"stackHeight":2},{"accounts":[1,12],"data":"31tb","programIdIndex":13,"stackHeight":2},{"accounts":[17],"data":"kxKFkfTgrrXZfAxrKGyWE1EeCib9hPncRn9M8CmEomAvQhwupk5Ra69KLuQsx2hfNKRdVHQX3Rtgr2e397cmKeY1BAitVFQMx9Hyy7HV5SA8977TAQMvaGArM4nkDPWRTezXmJvwBHJwLLYtfnQExg9n7dpAyVJoKrppjwk9n9smzjGGD8aUYwtc436RKyyzA35L5wagWTHw2mCd1p6kx1SDSd9T4CzjcLPxvze9wNnPYbmGVNEunaEckSQAjnvJuDCVkY8REEEC65uPBTBg4xLA","programIdIndex":9,"stackHeight":2}]},{"index":3,"instructions":[{"accounts":[1],"data":"84eT","programIdIndex":13,"stackHeight":2},{"accounts":[0,3],"data":"11119os1e9qSs2u7TsThXqkBSRVFxhmYaFKFZ1waB2X7armDmvK3p5GmLdUxYdg3h7QSrL","programIdIndex":8,"stackHeight":2},{"accounts":[3],"data":"P","programIdIndex":13,"stackHeight":2},{"accounts":[3,1],"data":"6dkdDAvX7UgWo5zARPLyaKBkeP5mHjXB3XA8oVC7KvjqC","programIdIndex":13,"stackHeight":2}]},{"index":4,"instructions":[{"accounts":[7,3,2],"data":"3QFcdgz55ykb","programIdIndex":13,"stackHeight":2},{"accounts":[0,2],"data":"3Bxs4RT5DbtktCVM","programIdIndex":8,"stackHeight":2},{"accounts":[0,4],"data":"3Bxs4134XtgF3WGw","programIdIndex":8,"stackHeight":2},{"accounts":[17],"data":"2K7nL28PxCW8ejnyCeuMpbXvsGoFBJnL22Y9yjXzvH4yuq9Liaq1yo9b3RhAWXVMs8no3BSh84PVe5YU4xgWqBWZba2KqWARctEjmBwphTrAVMbai6octxSMY7Ls6yKL2866a4pZVSXHsLHUsfVLUFjcvEe35E1zzV1m5gJBc8o2f42YCsHec96S2YZD","programIdIndex":9,"stackHeight":2}]}],"loadedAddresses":{"readonly":[],"writable":[]},"logMessages":["Program ComputeBudget111111111111111111111111111111 invoke [1]","Program ComputeBudget111111111111111111111111111111 success","Program ComputeBudget111111111111111111111111111111 invoke [1]","Program ComputeBudget111111111111111111111111111111 success","Program 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P invoke [1]","Program log: Instruction: Create","Program 11111111111111111111111111111111 invoke [2]","Program 11111111111111111111111111111111 success","Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [2]","Program log: Instruction: InitializeMint2","Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 2780 of 287749 compute units","Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success","Program 11111111111111111111111111111111 invoke [2]","Program 11111111111111111111111111111111 success","Program ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL invoke [2]","Program log: Create","Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [3]","Program log: Instruction: GetAccountDataSize","Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 1595 of 259235 compute units","Program return: TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA pQAAAAAAAAA=","Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success","Program 11111111111111111111111111111111 invoke [3]","Program 11111111111111111111111111111111 success","Program log: Initialize the associated token account","Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [3]","Program log: Instruction: InitializeImmutableOwner","Program log: Please upgrade to SPL Token 2022 for immutable owner support","Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 1405 of 252622 compute units","Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success","Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [3]","Program log: Instruction: InitializeAccount3","Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 4214 of 248738 compute units","Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success","Program ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL consumed 21990 of 266210 compute units","Program ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL success","Program metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s invoke [2]","Program log: IX: Create Metadata Accounts v3","Program 11111111111111111111111111111111 invoke [3]","Program 11111111111111111111111111111111 success","Program log: Allocate space for the account","Program 11111111111111111111111111111111 invoke [3]","Program 11111111111111111111111111111111 success","Program log: Assign the account to the owning program","Program 11111111111111111111111111111111 invoke [3]","Program 11111111111111111111111111111111 success","Program metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s consumed 37746 of 227439 compute units","Program metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s success","Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [2]","Program log: Instruction: MintTo","Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 4492 of 186568 compute units","Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success","Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [2]","Program log: Instruction: SetAuthority","Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 2911 of 179499 compute units","Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success","Program 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P invoke [2]","Program 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P consumed 2132 of 172357 compute units","Program 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P success","Program data: G3KpTd7rY3YHAAAA0KJVUlRsUwcAAADQolVSVGxTQwAAAGh0dHBzOi8vaXBmcy5pby9pcGZzL1FtVEJZYUhSWTQ3b2R4M3B2WEpIN2hkU2JwTkVyWWhUNzRkejFXMUNTM0xpa2bfgMrGP1o7h8C/LXMEZK7RWZAMtz40c9iKRSYCu6Bqb5ad4rsztjSSMXiW77vTh6TLOURovhh5Bu63ixIuj8C79u7a+n8LKJM22BkDUIezApHrPmHltk1q5uxqJl5gTLs=","Program 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P consumed 131905 of 299700 compute units","Program 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P success","Program ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL invoke [1]","Program log: Create","Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [2]","Program log: Instruction: GetAccountDataSize","Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 1569 of 159285 compute units","Program return: TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA pQAAAAAAAAA=","Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success","Program 11111111111111111111111111111111 invoke [2]","Program 11111111111111111111111111111111 success","Program log: Initialize the associated token account","Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [2]","Program log: Instruction: InitializeImmutableOwner","Program log: Please upgrade to SPL Token 2022 for immutable owner support","Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 1405 of 152698 compute units","Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success","Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [2]","Program log: Instruction: InitializeAccount3","Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 4188 of 148818 compute units","Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success","Program ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL consumed 23482 of 167795 compute units","Program ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL success","Program 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P invoke [1]","Program log: Instruction: Buy","Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [2]","Program log: Instruction: Transfer","Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 4645 of 119793 compute units","Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success","Program 11111111111111111111111111111111 invoke [2]","Program 11111111111111111111111111111111 success","Program 11111111111111111111111111111111 invoke [2]","Program 11111111111111111111111111111111 success","Program 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P invoke [2]","Program 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P consumed 2132 of 107223 compute units","Program 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P success","Program data: vdt/007mYe7fgMrGP1o7h8C/LXMEZK7RWZAMtz40c9iKRSYCu6Bqb5MIry8AAAAAQIh8BVkZAAAB9u7a+n8LKJM22BkDUIezApHrPmHltk1q5uxqJl5gTLsr155nAAAAAJO00isHAAAAwIdbQoq2AwCTCK8vAAAAAMDvSPb4twIA","Program 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P consumed 41426 of 144313 compute units","Program 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P success","Program 11111111111111111111111111111111 invoke [1]","Program 11111111111111111111111111111111 success","Program HQ2UUt18uJqKaQFJhgV9zaTdQxUZjNrsKFgoEDquBkcx invoke [1]","Program log: Powered by bloXroute Trader Api","Program HQ2UUt18uJqKaQFJhgV9zaTdQxUZjNrsKFgoEDquBkcx consumed 1029 of 102737 compute units","Program HQ2UUt18uJqKaQFJhgV9zaTdQxUZjNrsKFgoEDquBkcx success"],"postBalances":[6854365668,1461600,801232067,2039280,101571579974333,211396152932,15115600,2039280,1,1141440,1,1009200,440484922,934087680,1141440,246642259,731913600,122100014,1141440],"postTokenBalances":[{"accountIndex":3,"mint":"G3TpcmEy28TbzbyjL7TQy5noZbXrFBzJ5Vw5PqhTpump","owner":"HcvYEizKBqExpW4uJBnEqDKFtCBUKmuLpExwzcRWdbQE","programId":"TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA","uiTokenAmount":{"amount":"27870134831168","decimals":6,"uiAmount":27870134.831168,"uiAmountString":"27870134.831168"}},{"accountIndex":7,"mint":"G3TpcmEy28TbzbyjL7TQy5noZbXrFBzJ5Vw5PqhTpump","owner":"B8wno3ipF3v1p59SHCZsUfjXwpv98bL7aJHXbo382nA6","programId":"TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA","uiTokenAmount":{"amount":"972129865168832","decimals":6,"uiAmount":972129865.168832,"uiAmountString":"972129865.168832"}}],"preBalances":[7691138496,0,0,0,101571571974332,211391152932,0,0,1,1141440,1,1009200,440484922,934087680,1141440,246642259,731913600,122100014,1141440],"preTokenBalances":[],"rewards":[],"status":{"Ok":null}},"slot":317897944,"transaction":["AiQkjL2G9kYD5zbz+1hMtr1ubX9ayjqn/RGeSk9tbfYy82x6y4U8ec9nC31jTbMkjFd3DspNxkaTWYBzUaITdgS0iP5W1qPad0zYSrKcHcPhm6TakQok27cGC3ibwKGQN1QnA/hC+1wDqx1Y+QKaQ5SoeDF5JQmXtowHm+WzPrkBAgALE/bu2vp/CyiTNtgZA1CHswKR6z5h5bZNaubsaiZeYEy734DKxj9aO4fAvy1zBGSu0VmQDLc+NHPYikUmArugam+WneK7M7Y0kjF4lu+704ekyzlEaL4YeQbut4sSLo/Au6J0PiusRmt1EEJgvK5e4gpxPNod5rG8XD9NdXH/toxqrRHmpPwpRKT6glG++BVCbhv7KMa2ZGZ3YHxq2fVmpkb1OG8SvZdVn4bkbroTeHso+NPCQmjmufEDZn4foFUom/VdZt8PCzncOpHprK50jN3lP0fZxZxK5wIH7Qb6nsMj/zttAizFK+eLNC2an6iVCXCjvys0NFzJgHcM6K8XesUAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAFW4PaTZlrPRNsVaL8XW6pRicuX9dL/O2VdK7b9bRiwAwZGb+UhFzL/7K26csOb57yM5bvF9xJrLEObOkAAAAAGp9UXGSxcUSGMyUw9SvF/WNruCJuh/UTj29mKAAAAAAbFwc5jjSVn0mRosF65UdGijcxuEjSCtcZ1FJdw5ivyBt324ddloZPZy+FGzut5rBy0he1fWzeROoz1hX7/AKkLcGWx49F8RTidUn9rBMPNWLhscxqg/bVJttG8A/gpRjqGXmnuD1SAyrz2Y1fk3C8Y1Y1Fwep0ifs3I9l5PHKmjJclj04kifG7PRApFI4NgwtaE5na/xCEBI572Nvp+Fms8TbrAfwcTog9I8i1hEq1mjf2at1XxemsO1PgWdNcZPOhBA/YJb+FjB3SXIbz9mWbwz0Kuj54mmKlLK7JmfeBrB1/t/8jK+Oods/SME7RJowzYz66hMqXgRPxRK/qHIQHCgAJAxBeXwAAAAAACgAFAuCTBAAJDgEMAgcPDgYACA0QCxEJZRgeyCgFHAd3BwAAANCiVVJUbFMHAAAA0KJVUlRsU0MAAABodHRwczovL2lwZnMuaW8vaXBmcy9RbVRCWWFIUlk0N29keDNwdlhKSDdoZFNicE5FclloVDc0ZHoxVzFDUzNMaWtmEAcAAwABCA0LAAkMDwQBAgcDAAgNCxEJGGYGPRIB2uvqQIh8BVkZAAAAvHM0AAAAAAgCAAUMAgAAAEBLTAAAAAAAEgEAH1Bvd2VyZWQgYnkgYmxvWHJvdXRlIFRyYWRlciBBcGk=","base64"],"version":"legacy"}"#.to_string()),
        ]);
    }

    pub(crate) fn transaction(signature: impl AsRef<str>) -> Transaction {
        let json = transactions
            .get(signature.as_ref())
            .unwrap_or_else(|| panic!("Transaction not found: {}", signature.as_ref()));
        let tx: EncodedTransactionWithStatusMeta = serde_json::from_str(json).unwrap();
        convert_transaction(tx)
    }
}
