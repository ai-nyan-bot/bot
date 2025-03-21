-- Copyright (c) nyanbot.com 2025.
-- This file is licensed under the AGPL-3.0-or-later.

create table solana.sol_price_1m
(
    timestamp  timestamp with time zone not null,
    usd        real                     not null,
    created_at timestamptz not null default (timezone('utc', now())),
    updated_at timestamptz not null default (timezone('utc', now())),
    primary key (timestamp)
);

create table solana.sol_price_5m
(
    timestamp  timestamp with time zone not null,
    usd        real                     not null,
    created_at timestamptz not null default (timezone('utc', now())),
    updated_at timestamptz not null default (timezone('utc', now())),
    primary key (timestamp)
);

create table solana.sol_price_15m
(
    timestamp  timestamp with time zone not null,
    usd        real                     not null,
    created_at timestamptz not null default (timezone('utc', now())),
    updated_at timestamptz not null default (timezone('utc', now())),
    primary key (timestamp)
);

create table solana.sol_price_1h
(
    timestamp  timestamp with time zone not null,
    usd        real                     not null,
    created_at timestamptz not null default (timezone('utc', now())),
    updated_at timestamptz not null default (timezone('utc', now())),
    primary key (timestamp)
);

create table solana.sol_price_6h
(
    timestamp  timestamp with time zone not null,
    usd        real                     not null,
    created_at timestamptz not null default (timezone('utc', now())),
    updated_at timestamptz not null default (timezone('utc', now())),
    primary key (timestamp)
);

create table solana.sol_price_1d
(
    timestamp  timestamp with time zone not null,
    usd        real                     not null,
    created_at timestamptz not null default (timezone('utc', now())),
    updated_at timestamptz not null default (timezone('utc', now())),
    primary key (timestamp)
);
