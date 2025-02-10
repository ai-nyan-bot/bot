-- Copyright (c) nyanbot.com 2025.
-- This file is licensed under the AGPL-3.0-or-later.

create table solana.strategy
(
    id          serial primary key,
    version     smallint not null,
    name        text not null,
    user_id     int not null,
    sequence    json not null,
    created_at  timestamptz default (timezone('utc', now())),
    updated_at  timestamptz default (timezone('utc', now())),

    constraint fk_user
        foreign key (user_id)
        references nyanbot.user(id)
);

create trigger set_updated_at
before update on solana.strategy
for each row
execute function update_updated_at_column();