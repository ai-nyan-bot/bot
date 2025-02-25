-- Copyright (c) nyanbot.com 2025.
-- This file is licensed under the AGPL-3.0-or-later.

create table pumpfun.curve_log
(
    id                      int4 not null,
    slot                    int8 not null,
    virtual_base_reserves   int8 not null,
    virtual_quote_reserves  int8 not null,
    progress                real not null,
    complete                bool not null,
    created_at              timestamptz default (timezone('utc', now())),

    primary key (id, slot),

    constraint fk_token_pair
        foreign key (id)
        references solana.token_pair(id)
 );

create table pumpfun.curve
(
    id                      int4 primary key,
    slot                    int8 not null,
    virtual_base_reserves   int8 not null,
    virtual_quote_reserves  int8 not null,
    progress                real not null,
    complete                bool not null,
    updated_at              timestamptz default (timezone('utc', now())),

    constraint fk_token_pair
        foreign key (id)
        references solana.token_pair(id)
 );

create trigger set_updated_at
before update on pumpfun.curve

for each row
execute function update_updated_at_column();

create function pumpfun.curve_log_trigger_fn()
returns trigger as $$
begin
    insert into pumpfun.curve_log (
        id,
        slot,
        virtual_base_reserves,
        virtual_quote_reserves,
        progress,
        complete,
        created_at
    )
    values (
        new.id,
        new.slot,
        new.virtual_base_reserves,
        new.virtual_quote_reserves,
        new.progress,
        new.complete,
        timezone('utc', now())
    )
    on conflict (id, slot) do update
    set
        virtual_base_reserves = excluded.virtual_base_reserves,
        virtual_quote_reserves = excluded.virtual_quote_reserves,
        progress = excluded.progress,
        complete = excluded.complete,
        created_at = timezone('utc', now());
    return new;
end;
$$ language plpgsql;


create trigger curve_log
after insert or update on pumpfun.curve
for each row
execute function pumpfun.curve_log_trigger_fn();