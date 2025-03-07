-- Copyright (c) nyanbot.com 2025.
-- This file is licensed under the AGPL-3.0-or-later.

create schema pumpfun;

create function pumpfun.update_updated_at_column() returns trigger as $$
begin
    NEW.updated_at
= timezone('utc', now());
return NEW;
end;
$$
language plpgsql;


create table pumpfun.swap
(
    id                     bigserial       not null,
    slot                   int8            not null,
    address_id             int4            not null,
    token_pair_id          int4            not null,
    amount_base            numeric(36, 12) not null,
    amount_quote           numeric(36, 12) not null,
    price                  numeric(36, 12) not null,
    is_buy                 boolean         not null,
    timestamp              timestamptz     not null,
    virtual_base_reserves  int8            not null,
    virtual_quote_reserves int8            not null,
    progress               real            not null,
    signature              text            not null,

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