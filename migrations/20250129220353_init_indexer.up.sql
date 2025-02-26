-- Copyright (c) nyanbot.com 2025.
-- This file is licensed under the AGPL-3.0-or-later.

create table solana.indexer
(
    id          smallint primary key,
    slot        bigint not null,
    updated_at  timestamptz default (timezone('utc', now()))
);

create trigger set_updated_at
before update on solana.indexer
for each row
execute function update_updated_at_column();