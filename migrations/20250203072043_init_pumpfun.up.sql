-- Copyright (c) nyanbot.com 2025.
-- This file is licensed under the AGPL-3.0-or-later.

-- This file includes portions of code from https://github.com/blockworks-foundation/traffic (AGPL 3.0).
-- Original AGPL 3 License Copyright (c) blockworks-foundation 2024.

create schema pumpfun;

create table pumpfun.trade
(
    slot                    int8 not null,
    address_id              int4 not null,
    token_pair_id           int4 not null,
    base_amount             double precision not null,
    quote_amount            double precision not null,
    price                   double precision not null,
    is_buy                  boolean not null,
    timestamp               timestamptz not null,
    virtual_base_reserves   int8 not null,
    virtual_quote_reserves  int8 not null,
    signature               text not null,

    constraint fk_wallet
        foreign key (address_id)
        references solana.address(id),

    constraint fk_token_pair
        foreign key (token_pair_id)
        references solana.token_pair(id),

    constraint unique_signature
        unique (token_pair_id, signature)

 )  partition by hash (token_pair_id);
 
 create index trade_token_pair_idx on pumpfun.trade(token_pair_id);
 
 create table pumpfun.trade_1 partition of pumpfun.trade
     for values with (modulus 8, remainder 0);
 
 create table pumpfun.trade_2 partition of pumpfun.trade
     for values with (modulus 8, remainder 1);
 
 create table pumpfun.trade_3 partition of pumpfun.trade
     for values with (modulus 8, remainder 2);
 
 create table pumpfun.trade_4 partition of pumpfun.trade
     for values with (modulus 8, remainder 3);
 
 create table pumpfun.trade_5 partition of pumpfun.trade
     for values with (modulus 8, remainder 4);
 
 create table pumpfun.trade_6 partition of pumpfun.trade
     for values with (modulus 8, remainder 5);
 
 create table pumpfun.trade_7 partition of pumpfun.trade
     for values with (modulus 8, remainder 6);

 create table pumpfun.trade_8 partition of pumpfun.trade
     for values with (modulus 8, remainder 7);