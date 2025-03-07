-- Copyright (c) nyanbot.com 2025.
-- This file is licensed under the AGPL-3.0-or-later.

create table solana.token
(
    id          serial primary key,
    version     int2 not null default (0),
    mint        text not null,
    name        text,
    symbol      text,
    decimals    int2 not null,
    supply      numeric(36, 12),
    metadata    text,
    description text,
    image       text,
    website     text,
    updated_at  timestamptz   default (timezone('utc', now()))
);

create trigger set_updated_at
    before update
    on solana.token
    for each row execute function solana.update_updated_at_column();


insert into solana.token (id, mint, name, symbol, decimals)
values (1, 'So11111111111111111111111111111111111111112', 'Wrapped SOL', 'WSOL', 9),
       (2, 'Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB', 'USDT', 'USDT', 6),
       (3, 'EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v', 'USD Coin', 'USDC', 6),
       (4, 'J1toso1uCk3RLmjorhTtrVwY9HJ7X8V9yYac6Y7kGCPn', 'Jito Staked SOL', 'JitoSOL', 9),
       (5, 'jupSoLaHXQiZZTSfEWMTRRgpnyFm8f6sZdosWBjx93v', 'Jupiter Staked SOL', 'JupSOL', 9),
       (6, 'mSoLzYCxHdYgdzU16g5QSh3i5K3z3KZK7ytfqcJm7So', 'Marinade staked SOL', 'mSOL', 9),
       (7, 'BNso1VUJnh4zcfpZa6986Ea66P6TCp59hvtNJ8b1X85', 'Binance Staked SOL', 'BNSOL', 9);

alter sequence solana.token_id_seq restart with 1000;


create unique index token_unique_mint_idx on solana.token (mint);

create table solana.token_pair
(
    id       serial primary key,
    base_id  int4 not null,
    quote_id int4 not null,

    constraint fk_base foreign key (base_id) references solana.token (id),

    constraint fk_quote foreign key (quote_id) references solana.token (id)
);

create unique index token_pair_unique_idx on solana.token_pair (base_id, quote_id);

insert into solana.token_pair (id, base_id, quote_id)
values (1, 1, 2), -- WSOL/USDT
       (2, 1, 3), -- WSOL/USDC
       (3, 3, 2); -- USDC/USDT

alter sequence solana.token_pair_id_seq restart with 1000;