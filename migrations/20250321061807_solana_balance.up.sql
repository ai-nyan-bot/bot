-- Copyright (c) nyanbot.com 2025.
-- This file is licensed under the AGPL-3.0-or-later.

create table solana.token_balance
(
    address_id int8            not null,
    token_id   int8            not null,
    balance    numeric(36, 12) not null,
    slot       int8            not null,
    timestamp  timestamptz     not null,
    created_at timestamptz default (timezone('utc', now())),
    updated_at timestamptz default (timezone('utc', now())),
    primary key (token_id, address_id),
    constraint fk_address foreign key (address_id) references solana.address (id),
    constraint fk_token foreign key (token_id) references solana.token (id)
) partition by hash (token_id);

create table solana.token_balance_1 partition of solana.token_balance for values with (modulus 8, remainder 0);

create table solana.token_balance_2 partition of solana.token_balance for values with (modulus 8, remainder 1);

create table solana.token_balance_3 partition of solana.token_balance for values with (modulus 8, remainder 2);

create table solana.token_balance_4 partition of solana.token_balance for values with (modulus 8, remainder 3);

create table solana.token_balance_5 partition of solana.token_balance for values with (modulus 8, remainder 4);

create table solana.token_balance_6 partition of solana.token_balance for values with (modulus 8, remainder 5);

create table solana.token_balance_7 partition of solana.token_balance for values with (modulus 8, remainder 6);

create table solana.token_balance_8 partition of solana.token_balance for values with (modulus 8, remainder 7);
