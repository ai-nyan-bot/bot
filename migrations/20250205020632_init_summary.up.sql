-- Copyright (c) nyanbot.com 2025.
-- This file is licensed under the AGPL-3.0-or-later.

-- This file includes portions of code from https://github.com/blockworks-foundation/traffic (AGPL 3.0).
-- Original AGPL 3 License Copyright (c) blockworks-foundation 2024.
create table pumpfun.summary_1m (
    token_pair_id int4 primary key,

    trades int4,
    trades_change int4,
    trades_change_percent real,
    trades_buy int4,
    trades_buy_change int4,
    trades_buy_change_percent real,
    trades_sell int4,
    trades_sell_change int4,
    trades_sell_change_percent real,

    updated_at  timestamptz default (timezone('utc', now())),

    constraint fk_token_pair
        foreign key (token_pair_id)
        references solana.token_pair(id)
);


create trigger set_updated_at
before update on pumpfun.summary_1m
for each row
execute function update_updated_at_column();

create table pumpfun.summary_5m (
    token_pair_id int4 primary key,

    trades int4,
    trades_change int4,
    trades_change_percent real,
    trades_buy int4,
    trades_buy_change int4,
    trades_buy_change_percent real,
    trades_sell int4,
    trades_sell_change int4,
    trades_sell_change_percent real,

    updated_at  timestamptz default (timezone('utc', now())),

    constraint fk_token_pair
        foreign key (token_pair_id)
        references solana.token_pair(id)
);


create trigger set_updated_at
before update on pumpfun.summary_5m
for each row
execute function update_updated_at_column();

create table pumpfun.summary_15m (
    token_pair_id int4 primary key,

    trades int4,
    trades_change int4,
    trades_change_percent real,
    trades_buy int4,
    trades_buy_change int4,
    trades_buy_change_percent real,
    trades_sell int4,
    trades_sell_change int4,
    trades_sell_change_percent real,

    updated_at  timestamptz default (timezone('utc', now())),

    constraint fk_token_pair
        foreign key (token_pair_id)
        references solana.token_pair(id)
);


create trigger set_updated_at
before update on pumpfun.summary_15m
for each row
execute function update_updated_at_column();

create table pumpfun.summary_1h (
    token_pair_id int4 primary key,

    trades int4,
    trades_change int4,
    trades_change_percent real,
    trades_buy int4,
    trades_buy_change int4,
    trades_buy_change_percent real,
    trades_sell int4,
    trades_sell_change int4,
    trades_sell_change_percent real,

    updated_at  timestamptz default (timezone('utc', now())),

    constraint fk_token_pair
        foreign key (token_pair_id)
        references solana.token_pair(id)
);


create trigger set_updated_at
before update on pumpfun.summary_1h
for each row
execute function update_updated_at_column();

create table pumpfun.summary_4h (
    token_pair_id int4 primary key,

    trades int4,
    trades_change int4,
    trades_change_percent real,
    trades_buy int4,
    trades_buy_change int4,
    trades_buy_change_percent real,
    trades_sell int4,
    trades_sell_change int4,
    trades_sell_change_percent real,

    updated_at  timestamptz default (timezone('utc', now())),

    constraint fk_token_pair
        foreign key (token_pair_id)
        references solana.token_pair(id)
);


create trigger set_updated_at
before update on pumpfun.summary_4h
for each row
execute function update_updated_at_column();

create table pumpfun.summary_1d (
    token_pair_id int4 primary key,

    trades int4,
    trades_change int4,
    trades_change_percent real,
    trades_buy int4,
    trades_buy_change int4,
    trades_buy_change_percent real,
    trades_sell int4,
    trades_sell_change int4,
    trades_sell_change_percent real,

    updated_at  timestamptz default (timezone('utc', now())),

    constraint fk_token_pair
        foreign key (token_pair_id)
        references solana.token_pair(id)
);


create trigger set_updated_at
before update on pumpfun.summary_1d
for each row
execute function update_updated_at_column();