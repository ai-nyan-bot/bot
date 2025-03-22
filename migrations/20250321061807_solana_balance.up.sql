-- Copyright (c) nyanbot.com 2025.
-- This file is licensed under the AGPL-3.0-or-later.

create table solana.balance_token_most_recent
(
    address_id int8            not null,
    token_id   int8            not null,
    balance    numeric(36, 12) not null,
    delta      numeric(36, 12) not null,
    slot       int8            not null,
    timestamp  timestamptz     not null,
    created_at timestamptz default (timezone('utc', now())),
    updated_at timestamptz default (timezone('utc', now())),
    primary key (address_id, token_id),
    constraint fk_address foreign key (address_id) references solana.address (id),
    constraint fk_token foreign key (token_id) references solana.token (id)
);

create table solana.balance_token
(
    address_id int8            not null,
    token_id   int8            not null,
    balance    numeric(36, 12) not null,
    delta      numeric(36, 12) not null,
    slot       int8            not null,
    timestamp  timestamptz     not null,
    created_at timestamptz default (timezone('utc', now())),
    primary key (address_id, token_id, slot),
    constraint fk_address foreign key (address_id) references solana.address (id),
    constraint fk_token foreign key (token_id) references solana.token (id)
) partition by hash (address_id);

create table solana.balance_token_1 partition of solana.balance_token for values with (modulus 8, remainder 0);

create table solana.balance_token_2 partition of solana.balance_token for values with (modulus 8, remainder 1);

create table solana.balance_token_3 partition of solana.balance_token for values with (modulus 8, remainder 2);

create table solana.balance_token_4 partition of solana.balance_token for values with (modulus 8, remainder 3);

create table solana.balance_token_5 partition of solana.balance_token for values with (modulus 8, remainder 4);

create table solana.balance_token_6 partition of solana.balance_token for values with (modulus 8, remainder 5);

create table solana.balance_token_7 partition of solana.balance_token for values with (modulus 8, remainder 6);

create table solana.balance_token_8 partition of solana.balance_token for values with (modulus 8, remainder 7);


create function solana.update_balance_token_most_recent() returns trigger as $$
begin
insert into solana.balance_token_most_recent (address_id, token_id, balance, delta, slot, timestamp, created_at, updated_at)
values (new.address_id, new.token_id, new.balance, new.delta, new.slot, new.timestamp, new.created_at, now()) on conflict (address_id, token_id) do
update set balance = excluded.balance, delta = excluded.delta, slot = excluded.slot, timestamp = excluded.timestamp,  updated_at = now();
return null;
end;
$$
language plpgsql;

create trigger trigger_update_most_recent_token_balance
    after insert
    on solana.balance_token
    for each row execute function solana.update_balance_token_most_recent();

create table solana.balance_sol_most_recent
(
    address_id int8 primary key,
    balance    numeric(36, 12) not null,
    delta      numeric(36, 12) not null,
    slot       int8            not null,
    created_at timestamptz default (timezone('utc', now())),
    updated_at timestamptz default (timezone('utc', now())),
    constraint fk_address foreign key (address_id) references solana.address (id)
);


create table solana.balance_sol
(
    address_id int8            not null,
    balance    numeric(36, 12) not null,
    delta      numeric(36, 12) not null,
    slot       int8            not null,
    timestamp  timestamptz     not null,
    created_at timestamptz default (timezone('utc', now())),
    primary key (address_id, slot),
    constraint fk_address foreign key (address_id) references solana.address (id)
) partition by hash (address_id);

create table solana.balance_sol_1 partition of solana.balance_sol for values with (modulus 8, remainder 0);

create table solana.balance_sol_2 partition of solana.balance_sol for values with (modulus 8, remainder 1);

create table solana.balance_sol_3 partition of solana.balance_sol for values with (modulus 8, remainder 2);

create table solana.balance_sol_4 partition of solana.balance_sol for values with (modulus 8, remainder 3);

create table solana.balance_sol_5 partition of solana.balance_sol for values with (modulus 8, remainder 4);

create table solana.balance_sol_6 partition of solana.balance_sol for values with (modulus 8, remainder 5);

create table solana.balance_sol_7 partition of solana.balance_sol for values with (modulus 8, remainder 6);

create table solana.balance_sol_8 partition of solana.balance_sol for values with (modulus 8, remainder 7);

create function solana.update_balance_sol_most_recent() returns trigger as $$
begin
insert into solana.balance_sol_most_recent (address_id, balance, delta, slot, created_at, updated_at)
values (new.address_id, new.balance, new.delta, new.slot, new.created_at, now()) on conflict (address_id) do
update set balance = excluded.balance, delta = excluded.delta, slot = excluded.slot, updated_at = now();
return null;
end;
$$
language plpgsql;

create trigger trigger_update_most_recent_sol_balance
    after insert
    on solana.balance_sol
    for each row execute function solana.update_balance_sol_most_recent();