-- Copyright (c) nyanbot.com 2025.
-- This file is licensed under the AGPL-3.0-or-later.

create table jupiter.micro_swap
(
    id            bigserial       not null,
    slot          int8            not null,
    address_id    int8            not null,
    token_pair_id int8            not null,
    amount_base   numeric(36, 12) not null,
    amount_quote  numeric(36, 12) not null,
    price         numeric(36, 12) not null,
    is_buy        boolean         not null,
    timestamp     timestamptz     not null,
    signature     text            not null,
    created_at    timestamptz     not null default (timezone('utc', now())),
    updated_at    timestamptz     not null default (timezone('utc', now())),

    constraint fk_wallet foreign key (address_id) references solana.address (id),
    constraint fk_token_pair foreign key (token_pair_id) references solana.token_pair (id)
);

create table pumpfun.micro_swap
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
    created_at             timestamptz     not null default (timezone('utc', now())),
    updated_at             timestamptz     not null default (timezone('utc', now())),

    constraint fk_wallet foreign key (address_id) references solana.address (id),
    constraint fk_token_pair foreign key (token_pair_id) references solana.token_pair (id)
);

alter table pumpfun.swap drop constraint unique_signature;
alter table jupiter.swap drop constraint unique_signature;