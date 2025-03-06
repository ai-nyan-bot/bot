-- Copyright (c) nyanbot.com 2025.
-- This file is licensed under the AGPL-3.0-or-later.

create table pumpfun.curve
(
    id                     int4 primary key,
    slot                   int8 not null,
    virtual_base_reserves  int8 not null,
    virtual_quote_reserves int8 not null,
    progress               real not null,
    complete               bool not null,
    updated_at             timestamptz default (timezone('utc', now())),

    constraint fk_token_pair foreign key (id) references solana.token_pair (id)
);

create trigger set_updated_at
    before update
    on pumpfun.curve
    for each row execute function pumpfun.update_updated_at_column();

create index curve_updated_at_idx on pumpfun.curve (updated_at desc);