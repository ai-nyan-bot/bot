-- Copyright (c) nyanbot.com 2025.
-- This file is licensed under the AGPL-3.0-or-later.

create table nyanbot.notification
(
    id         bigserial primary key,
    user_id    int8 not null,
    type       int2 not null,
    channel    int2 not null,
    payload    json not null,
    created_at timestamptz default (timezone('utc', now())),
    updated_at timestamptz default (timezone('utc', now())),

    constraint fk_user foreign key (user_id) references nyanbot.user (id)
);

create table nyanbot.notification_sent
(
    id         int8 primary key,
    user_id    int8 not null,
    type       int2 not null,
    channel    int2 not null,
    payload    json not null,
    created_at timestamptz default (timezone('utc', now()))
);

create function nyanbot.store_notification_sent() returns trigger as $$
begin
insert into nyanbot.notification_sent (id, user_id, type, channel, payload, created_at)
values (old.id, old.user_id, old.type, old.channel, old.payload, timezone('utc', now()));
return old;
end;
$$
language plpgsql;

create trigger trigger_store_notification_sent
    before delete
    on nyanbot.notification
    for each row execute function nyanbot.store_notification_sent();
