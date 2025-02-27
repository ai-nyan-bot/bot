-- Copyright (c) nyanbot.com 2025.
-- This file is licensed under the AGPL-3.0-or-later.

create table nyanbot.notification (
    id serial   primary key,
    user_id     int4 not null,
    type        int2 not null,
    channel     int2 not null,
    payload     json not null,
    created_at  timestamptz default (timezone('utc', now())),

    constraint fk_user
        foreign key (user_id)
        references nyanbot.user(id)
);
