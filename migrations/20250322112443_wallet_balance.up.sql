-- Copyright (c) nyanbot.com 2025.
-- This file is licensed under the AGPL-3.0-or-later.

create table solana.wallet_token_balance_most_recent
(
    wallet_id  int8            not null,
    token_id   int8            not null,
    balance    numeric(36, 12) not null,
    delta      numeric(36, 12) not null,
    slot       int8            not null,
    timestamp  timestamptz     not null,
    created_at timestamptz default (timezone('utc', now())),
    updated_at timestamptz default (timezone('utc', now())),
    primary key (wallet_id, token_id),
    constraint fk_address foreign key (wallet_id) references solana.address (id),
    constraint fk_token foreign key (token_id) references solana.token (id)
);

create table solana.wallet_token_balance
(
    wallet_id  int8            not null,
    token_id   int8            not null,
    balance    numeric(36, 12) not null,
    delta      numeric(36, 12) not null,
    slot       int8            not null,
    timestamp  timestamptz     not null,
    created_at timestamptz default (timezone('utc', now())),
    primary key (wallet_id, token_id, slot),
    constraint fk_address foreign key (wallet_id) references solana.address (id),
    constraint fk_token foreign key (token_id) references solana.token (id)
) partition by hash (wallet_id);

create table solana.wallet_token_balance_1 partition of solana.wallet_token_balance for values with (modulus 8, remainder 0);

create table solana.wallet_token_balance_2 partition of solana.wallet_token_balance for values with (modulus 8, remainder 1);

create table solana.wallet_token_balance_3 partition of solana.wallet_token_balance for values with (modulus 8, remainder 2);

create table solana.wallet_token_balance_4 partition of solana.wallet_token_balance for values with (modulus 8, remainder 3);

create table solana.wallet_token_balance_5 partition of solana.wallet_token_balance for values with (modulus 8, remainder 4);

create table solana.wallet_token_balance_6 partition of solana.wallet_token_balance for values with (modulus 8, remainder 5);

create table solana.wallet_token_balance_7 partition of solana.wallet_token_balance for values with (modulus 8, remainder 6);

create table solana.wallet_token_balance_8 partition of solana.wallet_token_balance for values with (modulus 8, remainder 7);


create function solana.update_wallet_token_balance_most_recent() returns trigger as $$
begin
insert into solana.wallet_token_balance_most_recent (wallet_id, token_id, balance, delta, slot, timestamp, created_at, updated_at)
values (new.wallet_id, new.token_id, new.balance, new.delta, new.slot, new.timestamp, new.created_at, now()) on conflict (wallet_id, token_id) do
update set balance = excluded.balance, delta = excluded.delta, slot = excluded.slot, timestamp = excluded.timestamp, updated_at = now();
return null;
end;
$$
language plpgsql;

create trigger trigger_update_most_recent_token_balance
    after insert
    on solana.wallet_token_balance
    for each row execute function solana.update_wallet_token_balance_most_recent();

create table solana.wallet_balance_sol_most_recent
(
    wallet_id  int8 primary key,
    balance    numeric(36, 12) not null,
    delta      numeric(36, 12) not null,
    slot       int8            not null,
    created_at timestamptz default (timezone('utc', now())),
    updated_at timestamptz default (timezone('utc', now())),
    constraint fk_address foreign key (wallet_id) references solana.address (id)
);


create table solana.wallet_balance_sol
(
    wallet_id  int8            not null,
    balance    numeric(36, 12) not null,
    delta      numeric(36, 12) not null,
    slot       int8            not null,
    timestamp  timestamptz     not null,
    created_at timestamptz default (timezone('utc', now())),
    primary key (wallet_id, slot),
    constraint fk_address foreign key (wallet_id) references solana.address (id)
) partition by hash (wallet_id);

create table solana.wallet_balance_sol_1 partition of solana.wallet_balance_sol for values with (modulus 8, remainder 0);

create table solana.wallet_balance_sol_2 partition of solana.wallet_balance_sol for values with (modulus 8, remainder 1);

create table solana.wallet_balance_sol_3 partition of solana.wallet_balance_sol for values with (modulus 8, remainder 2);

create table solana.wallet_balance_sol_4 partition of solana.wallet_balance_sol for values with (modulus 8, remainder 3);

create table solana.wallet_balance_sol_5 partition of solana.wallet_balance_sol for values with (modulus 8, remainder 4);

create table solana.wallet_balance_sol_6 partition of solana.wallet_balance_sol for values with (modulus 8, remainder 5);

create table solana.wallet_balance_sol_7 partition of solana.wallet_balance_sol for values with (modulus 8, remainder 6);

create table solana.wallet_balance_sol_8 partition of solana.wallet_balance_sol for values with (modulus 8, remainder 7);

create function solana.update_wallet_balance_sol_most_recent() returns trigger as $$
begin
insert into solana.wallet_balance_sol_most_recent (wallet_id, balance, delta, slot, created_at, updated_at)
values (new.wallet_id, new.balance, new.delta, new.slot, new.created_at, now()) on conflict (wallet_id) do
update set balance = excluded.balance, delta = excluded.delta, slot = excluded.slot, updated_at = now();
return null;
end;
$$
language plpgsql;

create trigger trigger_update_most_recent_sol_balance
    after insert
    on solana.wallet_balance_sol
    for each row execute function solana.update_wallet_balance_sol_most_recent();