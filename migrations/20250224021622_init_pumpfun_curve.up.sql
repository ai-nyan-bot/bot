-- Copyright (c) nyanbot.com 2025.
-- This file is licensed under the AGPL-3.0-or-later.

create table pumpfun.curve
(
    token_pair_id           int4 not null,
    virtual_base_reserves   int8 not null,
    virtual_quote_reserves  int8 not null,
    real_base_reserves      int8 not null,
    real_quote_reserves     int8 not null,
    progress                real not null,
    complete                bool not null,

    constraint fk_token_pair
        foreign key (token_pair_id)
        references solana.token_pair(id)
 );
