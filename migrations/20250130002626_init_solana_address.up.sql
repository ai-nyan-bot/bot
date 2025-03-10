-- Copyright (c) nyanbot.com 2025.
-- This file is licensed under the AGPL-3.0-or-later.

create table solana.address
(
    id         bigserial primary key,
    address    text not null,

    created_at timestamptz default (timezone('utc', now())),
    updated_at timestamptz default (timezone('utc', now()))
);

create unique index address_unique_address_idx on solana.address (address);