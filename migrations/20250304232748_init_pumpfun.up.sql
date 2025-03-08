-- Copyright (c) nyanbot.com 2025.
-- This file is licensed under the AGPL-3.0-or-later.

create schema pumpfun;

create table pumpfun.swap
(
    id                     bigserial       not null,
    slot                   int8            not null,
    address_id             int8            not null,
    token_pair_id          int8            not null,
    amount_base            numeric(36, 12) not null,
    amount_quote           numeric(36, 12) not null,
    price                  numeric(36, 12) not null,
    is_buy                 boolean         not null,
    timestamp              timestamptz     not null,
    virtual_base_reserves  int8            not null,
    virtual_quote_reserves int8            not null,
    progress               real            not null,
    signature              text            not null,
    created_at             timestamptz not null default (timezone('utc', now())),
    updated_at             timestamptz not null default (timezone('utc', now())),

    constraint fk_wallet foreign key (address_id) references solana.address (id),

    constraint fk_token_pair foreign key (token_pair_id) references solana.token_pair (id),

    constraint unique_signature unique (token_pair_id, signature)

) partition by hash (token_pair_id);

create index swap_token_pair_idx on pumpfun.swap (token_pair_id);

create table pumpfun.swap_1 partition of pumpfun.swap for values with (modulus 8, remainder 0);

create table pumpfun.swap_2 partition of pumpfun.swap for values with (modulus 8, remainder 1);

create table pumpfun.swap_3 partition of pumpfun.swap for values with (modulus 8, remainder 2);

create table pumpfun.swap_4 partition of pumpfun.swap for values with (modulus 8, remainder 3);

create table pumpfun.swap_5 partition of pumpfun.swap for values with (modulus 8, remainder 4);

create table pumpfun.swap_6 partition of pumpfun.swap for values with (modulus 8, remainder 5);

create table pumpfun.swap_7 partition of pumpfun.swap for values with (modulus 8, remainder 6);

create table pumpfun.swap_8 partition of pumpfun.swap for values with (modulus 8, remainder 7);

create index swap_1_timestamp ON pumpfun.swap_1 (timestamp desc);
create index swap_2_timestamp ON pumpfun.swap_2 (timestamp desc);
create index swap_3_timestamp ON pumpfun.swap_3 (timestamp desc);
create index swap_4_timestamp ON pumpfun.swap_4 (timestamp desc);
create index swap_5_timestamp ON pumpfun.swap_5 (timestamp desc);
create index swap_6_timestamp ON pumpfun.swap_6 (timestamp desc);
create index swap_7_timestamp ON pumpfun.swap_7 (timestamp desc);
create index swap_8_timestamp ON pumpfun.swap_8 (timestamp desc);