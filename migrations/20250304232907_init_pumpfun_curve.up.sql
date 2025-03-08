-- Copyright (c) nyanbot.com 2025.
-- This file is licensed under the AGPL-3.0-or-later.

create table pumpfun.curve
(
    id                     int8 primary key,
    slot                   int8 not null,
    virtual_base_reserves  int8 not null,
    virtual_quote_reserves int8 not null,
    progress               real not null,
    complete               bool not null,
    created_at             timestamptz not null default (timezone('utc', now())),
    updated_at             timestamptz not null default (timezone('utc', now())),

    constraint fk_token_pair foreign key (id) references solana.token_pair (id)
);

create index curve_updated_at_idx on pumpfun.curve (updated_at desc);