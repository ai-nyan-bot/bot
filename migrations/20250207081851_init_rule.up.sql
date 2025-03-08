-- Copyright (c) nyanbot.com 2025.
-- This file is licensed under the AGPL-3.0-or-later.

create table solana.rule
(
    id         bigserial primary key,
    version    int2  not null,
    name       text  not null,
    user_id    int8  not null,
    sequence   jsonb not null,
    created_at timestamptz default (timezone('utc', now())),
    updated_at timestamptz default (timezone('utc', now())),

    constraint fk_user
        foreign key (user_id)
            references nyanbot.user (id)
            on delete cascade
);

create
or replace function solana.increment_rule_version()
returns trigger as $$
begin
    new.version
:= new.version + 1;
return new;
end;
$$
language plpgsql;

create trigger trigger_increment_version
    before update
    on solana.rule
    for each row
    when (old.* is distinct from new.*)
execute function solana.increment_rule_version();
