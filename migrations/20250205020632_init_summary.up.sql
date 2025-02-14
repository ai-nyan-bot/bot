-- Copyright (c) nyanbot.com 2025.
-- This file is licensed under the AGPL-3.0-or-later.

-- This file includes portions of code from https://github.com/blockworks-foundation/traffic (AGPL 3.0).
-- Original AGPL 3 License Copyright (c) blockworks-foundation 2024.
create table pumpfun.summary_1m (
    token_pair_id int primary key,

    high double precision not null,
    high_change double precision not null,
    high_usd double precision not null,
    high_usd_change double precision not null,
    
    low double precision not null,
    low_change double precision not null,
    low_usd double precision not null,
    low_usd_change double precision not null,

    avg double precision not null,
    avg_change double precision not null,
    avg_usd double precision not null,
    avg_usd_change double precision not null,

    amount double precision not null,
    amount_change double precision not null,
    buy_amount double precision not null,
    buy_amount_change double precision not null,
    sell_amount double precision not null,
    sell_amount_change double precision not null,

    trades int,
    trades_change double precision not null,
    buy_trades int,
    buy_trades_change double precision not null,
    sell_trades int,
    sell_trades_change double precision not null,

    volume double precision not null,
    volume_change double precision not null,
    volume_usd double precision not null,
    volume_usd_change double precision not null,

    buy_volume double precision not null,
    buy_volume_change double precision not null,
    buy_volume_usd double precision not null,
    buy_volume_usd_change double precision not null,

    sell_volume double precision not null,
    sell_volume_change double precision not null,
    sell_volume_usd double precision not null,
    sell_volume_usd_change double precision not null,

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
    token_pair_id int primary key,

    high double precision not null,
    high_change double precision not null,
    high_usd double precision not null,
    high_usd_change double precision not null,
    
    low double precision not null,
    low_change double precision not null,
    low_usd double precision not null,
    low_usd_change double precision not null,

    avg double precision not null,
    avg_change double precision not null,
    avg_usd double precision not null,
    avg_usd_change double precision not null,

    amount double precision not null,
    amount_change double precision not null,
    buy_amount double precision not null,
    buy_amount_change double precision not null,
    sell_amount double precision not null,
    sell_amount_change double precision not null,

    trades int,
    trades_change double precision not null,
    buy_trades int,
    buy_trades_change double precision not null,
    sell_trades int,
    sell_trades_change double precision not null,

    volume double precision not null,
    volume_change double precision not null,
    volume_usd double precision not null,
    volume_usd_change double precision not null,

    buy_volume double precision not null,
    buy_volume_change double precision not null,
    buy_volume_usd double precision not null,
    buy_volume_usd_change double precision not null,

    sell_volume double precision not null,
    sell_volume_change double precision not null,
    sell_volume_usd double precision not null,
    sell_volume_usd_change double precision not null,

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
    token_pair_id int primary key,

    high double precision not null,
    high_change double precision not null,
    high_usd double precision not null,
    high_usd_change double precision not null,
    
    low double precision not null,
    low_change double precision not null,
    low_usd double precision not null,
    low_usd_change double precision not null,

    avg double precision not null,
    avg_change double precision not null,
    avg_usd double precision not null,
    avg_usd_change double precision not null,

    amount double precision not null,
    amount_change double precision not null,
    buy_amount double precision not null,
    buy_amount_change double precision not null,
    sell_amount double precision not null,
    sell_amount_change double precision not null,

    trades int,
    trades_change double precision not null,
    buy_trades int,
    buy_trades_change double precision not null,
    sell_trades int,
    sell_trades_change double precision not null,

    volume double precision not null,
    volume_change double precision not null,
    volume_usd double precision not null,
    volume_usd_change double precision not null,

    buy_volume double precision not null,
    buy_volume_change double precision not null,
    buy_volume_usd double precision not null,
    buy_volume_usd_change double precision not null,

    sell_volume double precision not null,
    sell_volume_change double precision not null,
    sell_volume_usd double precision not null,
    sell_volume_usd_change double precision not null,

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
    token_pair_id int primary key,

    high double precision not null,
    high_change double precision not null,
    high_usd double precision not null,
    high_usd_change double precision not null,
    
    low double precision not null,
    low_change double precision not null,
    low_usd double precision not null,
    low_usd_change double precision not null,

    avg double precision not null,
    avg_change double precision not null,
    avg_usd double precision not null,
    avg_usd_change double precision not null,

    amount double precision not null,
    amount_change double precision not null,
    buy_amount double precision not null,
    buy_amount_change double precision not null,
    sell_amount double precision not null,
    sell_amount_change double precision not null,

    trades int,
    trades_change double precision not null,
    buy_trades int,
    buy_trades_change double precision not null,
    sell_trades int,
    sell_trades_change double precision not null,

    volume double precision not null,
    volume_change double precision not null,
    volume_usd double precision not null,
    volume_usd_change double precision not null,

    buy_volume double precision not null,
    buy_volume_change double precision not null,
    buy_volume_usd double precision not null,
    buy_volume_usd_change double precision not null,

    sell_volume double precision not null,
    sell_volume_change double precision not null,
    sell_volume_usd double precision not null,
    sell_volume_usd_change double precision not null,

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
    token_pair_id int primary key,

    high double precision not null,
    high_change double precision not null,
    high_usd double precision not null,
    high_usd_change double precision not null,
    
    low double precision not null,
    low_change double precision not null,
    low_usd double precision not null,
    low_usd_change double precision not null,

    avg double precision not null,
    avg_change double precision not null,
    avg_usd double precision not null,
    avg_usd_change double precision not null,

    amount double precision not null,
    amount_change double precision not null,
    buy_amount double precision not null,
    buy_amount_change double precision not null,
    sell_amount double precision not null,
    sell_amount_change double precision not null,

    trades int,
    trades_change double precision not null,
    buy_trades int,
    buy_trades_change double precision not null,
    sell_trades int,
    sell_trades_change double precision not null,

    volume double precision not null,
    volume_change double precision not null,
    volume_usd double precision not null,
    volume_usd_change double precision not null,

    buy_volume double precision not null,
    buy_volume_change double precision not null,
    buy_volume_usd double precision not null,
    buy_volume_usd_change double precision not null,

    sell_volume double precision not null,
    sell_volume_change double precision not null,
    sell_volume_usd double precision not null,
    sell_volume_usd_change double precision not null,

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
    token_pair_id int primary key,

    high double precision not null,
    high_change double precision not null,
    high_usd double precision not null,
    high_usd_change double precision not null,
    
    low double precision not null,
    low_change double precision not null,
    low_usd double precision not null,
    low_usd_change double precision not null,

    avg double precision not null,
    avg_change double precision not null,
    avg_usd double precision not null,
    avg_usd_change double precision not null,

    amount double precision not null,
    amount_change double precision not null,
    buy_amount double precision not null,
    buy_amount_change double precision not null,
    sell_amount double precision not null,
    sell_amount_change double precision not null,

    trades int,
    trades_change double precision not null,
    buy_trades int,
    buy_trades_change double precision not null,
    sell_trades int,
    sell_trades_change double precision not null,

    volume double precision not null,
    volume_change double precision not null,
    volume_usd double precision not null,
    volume_usd_change double precision not null,

    buy_volume double precision not null,
    buy_volume_change double precision not null,
    buy_volume_usd double precision not null,
    buy_volume_usd_change double precision not null,

    sell_volume double precision not null,
    sell_volume_change double precision not null,
    sell_volume_usd double precision not null,
    sell_volume_usd_change double precision not null,

    updated_at  timestamptz default (timezone('utc', now())),

    constraint fk_token_pair
        foreign key (token_pair_id)
        references solana.token_pair(id)
);


create trigger set_updated_at
before update on pumpfun.summary_1d
for each row
execute function update_updated_at_column();