-- Copyright (c) nyanbot.com 2025.
-- This file is licensed under the AGPL-3.0-or-later.

create table solana.address
(
    id      serial primary key,
    address text not null
);

create unique index address_unique_address_idx on solana.address(address);