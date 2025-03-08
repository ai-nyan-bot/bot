-- Copyright (c) nyanbot.com 2025.
-- This file is licensed under the AGPL-3.0-or-later.

create table nyanbot.user
(
    id          bigserial primary key,
    telegram_id text null,
    created_at  timestamptz default (timezone('utc', now())),
    updated_at  timestamptz default (timezone('utc', now()))
);

create unique index unique_telegram_id on nyanbot.user (telegram_id) where telegram_id is not null;

create table nyanbot.auth
(
    id         bigserial primary key,
    user_id    int8 not null,
    token      text not null,
    created_at timestamptz default (timezone('utc', now())),
    updated_at timestamptz default (timezone('utc', now())),

    constraint fk_user
        foreign key (user_id)
            references nyanbot.user (id)
);

create unique index auth_unique_token_idx on nyanbot.auth (token);
