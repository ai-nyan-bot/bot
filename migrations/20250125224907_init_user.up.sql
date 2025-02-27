-- Copyright (c) nyanbot.com 2025.
-- This file is licensed under the AGPL-3.0-or-later.

create or replace function update_updated_at_column()
returns trigger as $$
begin
    NEW.updated_at = timezone('utc', now());
    return NEW;
end;
$$ language plpgsql;


create table nyanbot.user
(
    id          serial primary key,
    telegram_id text null,
    created_at  timestamptz default (timezone('utc', now())),
    updated_at  timestamptz default (timezone('utc', now()))
);

create unique index unique_telegram_id on nyanbot.user (telegram_id) where telegram_id is not null;

create trigger set_updated_at
before update on nyanbot.user
for each row
execute function update_updated_at_column();

create table nyanbot.auth
(
    id          serial  primary key,
    user_id     int4 not null,
    token       text not null,
    created_at  timestamptz default (timezone('utc', now())),
    updated_at  timestamptz default (timezone('utc', now())),

    constraint fk_user
        foreign key (user_id)
        references nyanbot.user(id)
);

create unique index auth_unique_token_idx on nyanbot.auth(token);

create trigger set_updated_at
before update on nyanbot.auth
for each row
execute function update_updated_at_column();