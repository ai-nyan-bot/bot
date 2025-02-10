#############################################################################################

There are 3 trades - indexer only finds 2

Overview
Block
318124628
Timestamp
6 hrs ago
February 03, 2025 03:40:16 +UTC
Block Hash
5J7cC8sxmWYpxQKvARXFHrYZ9qxuL3qmyn2C8VMMQGCh
Epoch
736
Leader
validator.com
Reward
0.0104 SOL ($2.0510)
SOL price: $197.09
Transactions
Total 1,946 transactions
Previous Block Hash
D3ig518XJV6N4FPZoWC1dCShevJiJCgN1b9hy4EBrz1H
Exclude Vote Program
Signature

Block

Time
Instructions
By
Value (SOL)

Fee (SOL)

Programs
4ZpWPGRKmR4ChymvfWXabC4jbch6ryee9UJEQUbyfdmo2rMYyd9mYjt9AkRsz94p8ZqYwpTDTtYuaQHnLXxRvzys
318124628
6 hrs ago

transfer
76HD3VPnPX...HvHHf2gETM
price
0.1462

price
0.0001705

image
6M
image
aqB6ye9KU3V63GUdtwtFm6cmsaFQNdqJFKvQhMqQNnvh5YDRnjMYKEQkZ4EedVpzNRgygSDTjWkjzQFV5N8jfGf
318124628
6 hrs ago

buy
Bxf3YiWHXA...M5gbV1H42H
price
0.000005

price
0.000005

image
TJ
image
5XLXaW5iFYHt2BFYi15h2TaK7rYWkXLU1TkEYAmEDyWowJZD1yJ4x27KzLdjp9ZUkZAL9z4a6tWtcASwiZdG5mL2
318124628
6 hrs ago

buy
Bxf3YiWHXA...M5gbV1H42H
price
0.000005

price
0.000005

image
TJ
image
57g7GeNn9j8J819XXd6eAqGThZsdwDHwNNVxMe6oJwAZwPYgN9G5R7co4B5SuyjpBCaf2nfNXmDJQuV98E6rGzpN
318124628
6 hrs ago

sell
9ibdHqD6g2...ukjNbpHVGo
price
0.01425

price
0.01

image
AF
4P
4FufGr26C7XNSMwqt8y51LYvf8UhgdDmmZicfQoRiGTiD549772Q54Q7GPeZ5EmnshMY4Sdb2Vph78cXiyJe8Bzo
318124628
6 hrs ago

buy
FcEQM1QgxY...dR728hL6mW
price
0.02281

price
0.000005

image
image
image
2RGrgv9Ar8X6wcb1N7pRUxnXiVm4zcgqNJVydwQnjHPxWSLnEsiXNwBJ4yTQmCxxkbvqVTjYefQ32Q1zCBjbJCG8
318124628
6 hrs ago

sell
8S6P5yVd5g...sJC3Jj36LN
price
0.00000833

price
0.00000833

image
image
image
mKN4zvi3HHB1fzBSF5g8fgvVVfHVkUzUXvUg7TKS3NJQ1wGTHDMa2n7tFYQJkfno37iJyunGf9rM3QLM2WGSKQR
318124628
6 hrs ago

sell
8S6P5yVd5g...sJC3Jj36LN
price
0.00000833

price
0.00000833

image
image
image

#############################################################################################

pda not found

2025-02-06T10:52:37.214001Z TRACE web3::solana::rpc::block: get block for 318839359    
2025-02-06T10:52:37.669286Z DEBUG web3::solana::token_info::rpc: Load token info: SLNDpmoWTVADgEdndyvWzroNL7zSi1dF9PC3xHGtPwp    
2025-02-06T10:52:37.879337Z ERROR web3::solana::token_info::rpc: pda not found: SLNDpmoWTVADgEdndyvWzroNL7zSi1dF9PC3xHGtPwp    
thread 'tokio-runtime-worker' panicked at bin/indexer/src/solana/jupiter/trade.rs:9:77:
called `Result::unwrap()` on an `Err` value: NotFound
stack backtrace:
2025-02-06T10:52:37.890737Z DEBUG web3::solana::stream::slot: 318839518

#############################################################################################
