-- Copyright (c) nyanbot.com 2025.
-- This file is licensed under the AGPL-3.0-or-later.

create table nyanbot.wallet
(
    id                 bigserial primary key,
    user_id            int8 not null,
    solana_public_key  text not null,
    solana_private_key text not null,
    created_at         timestamptz default (timezone('utc', now())),
    updated_at         timestamptz default (timezone('utc', now())),

    constraint fk_user
        foreign key (user_id)
            references nyanbot.user (id)
);

create unique index unique_user_id on nyanbot.wallet (user_id);
create unique index unique_solana_solana_public_key on nyanbot.wallet (solana_public_key);
create unique index unique_solana_solana_private_key on nyanbot.wallet (solana_private_key);
