-- Copyright (c) nyanbot.com 2025.
-- This file is licensed under the AGPL-3.0-or-later.


create table pumpfun.summary_1m
(
    token_pair_id               int4 primary key,

    amount_base                 numeric(36, 12),
    amount_base_change          numeric(36, 12),
    amount_base_percent         real,
    amount_base_buy             numeric(36, 12),
    amount_base_buy_change      numeric(36, 12),
    amount_base_buy_percent     real,
    amount_base_sell            numeric(36, 12),
    amount_base_sell_change     numeric(36, 12),
    amount_base_sell_percent    real,

    amount_quote                numeric(36, 12),
    amount_quote_change         numeric(36, 12),
    amount_quote_percent        real,
    amount_quote_buy            numeric(36, 12),
    amount_quote_buy_change     numeric(36, 12),
    amount_quote_buy_percent    real,
    amount_quote_sell           numeric(36, 12),
    amount_quote_sell_change    numeric(36, 12),
    amount_quote_sell_percent   real,


    curve_progress_open         real,
    curve_progress_open_change  real,

    curve_progress_high         real,
    curve_progress_high_change  real,

    curve_progress_low          real,
    curve_progress_low_change   real,

    curve_progress_close        real,
    curve_progress_close_change real,

    curve_progress_avg          real,
    curve_progress_avg_change   real,

    market_cap_open             numeric(36, 12),
    market_cap_open_usd         numeric(36, 12),
    market_cap_open_change      numeric(36, 12),
    market_cap_open_usd_change  numeric(36, 12),
    market_cap_open_percent     real,

    market_cap_high             numeric(36, 12),
    market_cap_high_usd         numeric(36, 12),
    market_cap_high_change      numeric(36, 12),
    market_cap_high_usd_change  numeric(36, 12),
    market_cap_high_percent     real,

    market_cap_low              numeric(36, 12),
    market_cap_low_usd          numeric(36, 12),
    market_cap_low_change       numeric(36, 12),
    market_cap_low_usd_change   numeric(36, 12),
    market_cap_low_percent      real,

    market_cap_close            numeric(36, 12),
    market_cap_close_usd        numeric(36, 12),
    market_cap_close_change     numeric(36, 12),
    market_cap_close_usd_change numeric(36, 12),
    market_cap_close_percent    real,

    market_cap_avg              numeric(36, 12),
    market_cap_avg_usd          numeric(36, 12),
    market_cap_avg_change       numeric(36, 12),
    market_cap_avg_usd_change   numeric(36, 12),
    market_cap_avg_percent      real,

    price_open                  numeric(36, 12),
    price_open_usd              numeric(36, 12),
    price_open_change           numeric(36, 12),
    price_open_usd_change       numeric(36, 12),
    price_open_percent          real,

    price_high                  numeric(36, 12),
    price_high_usd              numeric(36, 12),
    price_high_change           numeric(36, 12),
    price_high_usd_change       numeric(36, 12),
    price_high_percent          real,

    price_low                   numeric(36, 12),
    price_low_usd               numeric(36, 12),
    price_low_change            numeric(36, 12),
    price_low_usd_change        numeric(36, 12),
    price_low_percent           real,

    price_close                 numeric(36, 12),
    price_close_usd             numeric(36, 12),
    price_close_change          numeric(36, 12),
    price_close_usd_change      numeric(36, 12),
    price_close_percent         real,

    price_avg                   numeric(36, 12),
    price_avg_usd               numeric(36, 12),
    price_avg_change            numeric(36, 12),
    price_avg_usd_change        numeric(36, 12),
    price_avg_percent           real,

    trade                       int8,
    trade_change                numeric(36, 12),
    trade_percent               real,
    trade_buy                   int8,
    trade_buy_change            numeric(36, 12),
    trade_buy_percent           real,
    trade_sell                  int8,
    trade_sell_change           numeric(36, 12),
    trade_sell_percent          real,

    volume                      numeric(36, 12),
    volume_usd                  numeric(36, 12),
    volume_change               numeric(36, 12),
    volume_usd_change           numeric(36, 12),
    volume_percent              real,
    volume_buy                  numeric(36, 12),
    volume_buy_usd              numeric(36, 12),
    volume_buy_change           numeric(36, 12),
    volume_buy_usd_change       numeric(36, 12),
    volume_buy_percent          real,
    volume_sell                 numeric(36, 12),
    volume_sell_usd             numeric(36, 12),
    volume_sell_change          numeric(36, 12),
    volume_sell_usd_change      numeric(36, 12),
    volume_sell_percent         real,

    updated_at                  timestamptz default (timezone('utc', now())),

    constraint fk_token_pair foreign key (token_pair_id) references solana.token_pair (id)
);


create trigger set_updated_at
    before update
    on pumpfun.summary_1m
    for each row execute function pumpfun.update_updated_at_column();

create table pumpfun.summary_5m
(
    token_pair_id               int4 primary key,

    amount_base                 numeric(36, 12),
    amount_base_change          numeric(36, 12),
    amount_base_percent         real,
    amount_base_buy             numeric(36, 12),
    amount_base_buy_change      numeric(36, 12),
    amount_base_buy_percent     real,
    amount_base_sell            numeric(36, 12),
    amount_base_sell_change     numeric(36, 12),
    amount_base_sell_percent    real,

    amount_quote                numeric(36, 12),
    amount_quote_change         numeric(36, 12),
    amount_quote_percent        real,
    amount_quote_buy            numeric(36, 12),
    amount_quote_buy_change     numeric(36, 12),
    amount_quote_buy_percent    real,
    amount_quote_sell           numeric(36, 12),
    amount_quote_sell_change    numeric(36, 12),
    amount_quote_sell_percent   real,

    curve_progress_open         real,
    curve_progress_open_change  real,

    curve_progress_high         real,
    curve_progress_high_change  real,

    curve_progress_low          real,
    curve_progress_low_change   real,

    curve_progress_close        real,
    curve_progress_close_change real,

    curve_progress_avg          real,
    curve_progress_avg_change   real,

    market_cap_open             numeric(36, 12),
    market_cap_open_usd         numeric(36, 12),
    market_cap_open_change      numeric(36, 12),
    market_cap_open_usd_change  numeric(36, 12),
    market_cap_open_percent     real,

    market_cap_high             numeric(36, 12),
    market_cap_high_usd         numeric(36, 12),
    market_cap_high_change      numeric(36, 12),
    market_cap_high_usd_change  numeric(36, 12),
    market_cap_high_percent     real,

    market_cap_low              numeric(36, 12),
    market_cap_low_usd          numeric(36, 12),
    market_cap_low_change       numeric(36, 12),
    market_cap_low_usd_change   numeric(36, 12),
    market_cap_low_percent      real,

    market_cap_close            numeric(36, 12),
    market_cap_close_usd        numeric(36, 12),
    market_cap_close_change     numeric(36, 12),
    market_cap_close_usd_change numeric(36, 12),
    market_cap_close_percent    real,

    market_cap_avg              numeric(36, 12),
    market_cap_avg_usd          numeric(36, 12),
    market_cap_avg_change       numeric(36, 12),
    market_cap_avg_usd_change   numeric(36, 12),
    market_cap_avg_percent      real,

    price_open                  numeric(36, 12),
    price_open_usd              numeric(36, 12),
    price_open_change           numeric(36, 12),
    price_open_usd_change       numeric(36, 12),
    price_open_percent          real,

    price_high                  numeric(36, 12),
    price_high_usd              numeric(36, 12),
    price_high_change           numeric(36, 12),
    price_high_usd_change       numeric(36, 12),
    price_high_percent          real,

    price_low                   numeric(36, 12),
    price_low_usd               numeric(36, 12),
    price_low_change            numeric(36, 12),
    price_low_usd_change        numeric(36, 12),
    price_low_percent           real,

    price_close                 numeric(36, 12),
    price_close_usd             numeric(36, 12),
    price_close_change          numeric(36, 12),
    price_close_usd_change      numeric(36, 12),
    price_close_percent         real,

    price_avg                   numeric(36, 12),
    price_avg_usd               numeric(36, 12),
    price_avg_change            numeric(36, 12),
    price_avg_usd_change        numeric(36, 12),
    price_avg_percent           real,

    trade                       int8,
    trade_change                numeric(36, 12),
    trade_percent               real,
    trade_buy                   int8,
    trade_buy_change            numeric(36, 12),
    trade_buy_percent           real,
    trade_sell                  int8,
    trade_sell_change           numeric(36, 12),
    trade_sell_percent          real,

    volume                      numeric(36, 12),
    volume_usd                  numeric(36, 12),
    volume_change               numeric(36, 12),
    volume_usd_change           numeric(36, 12),
    volume_percent              real,
    volume_buy                  numeric(36, 12),
    volume_buy_usd              numeric(36, 12),
    volume_buy_change           numeric(36, 12),
    volume_buy_usd_change       numeric(36, 12),
    volume_buy_percent          real,
    volume_sell                 numeric(36, 12),
    volume_sell_usd             numeric(36, 12),
    volume_sell_change          numeric(36, 12),
    volume_sell_usd_change      numeric(36, 12),
    volume_sell_percent         real,

    updated_at                  timestamptz default (timezone('utc', now())),

    constraint fk_token_pair foreign key (token_pair_id) references solana.token_pair (id)
);


create trigger set_updated_at
    before update
    on pumpfun.summary_5m
    for each row execute function pumpfun.update_updated_at_column();

create table pumpfun.summary_15m
(
    token_pair_id               int4 primary key,

    amount_base                 numeric(36, 12),
    amount_base_change          numeric(36, 12),
    amount_base_percent         real,
    amount_base_buy             numeric(36, 12),
    amount_base_buy_change      numeric(36, 12),
    amount_base_buy_percent     real,
    amount_base_sell            numeric(36, 12),
    amount_base_sell_change     numeric(36, 12),
    amount_base_sell_percent    real,

    amount_quote                numeric(36, 12),
    amount_quote_change         numeric(36, 12),
    amount_quote_percent        real,
    amount_quote_buy            numeric(36, 12),
    amount_quote_buy_change     numeric(36, 12),
    amount_quote_buy_percent    real,
    amount_quote_sell           numeric(36, 12),
    amount_quote_sell_change    numeric(36, 12),
    amount_quote_sell_percent   real,

    curve_progress_open         real,
    curve_progress_open_change  real,

    curve_progress_high         real,
    curve_progress_high_change  real,

    curve_progress_low          real,
    curve_progress_low_change   real,

    curve_progress_close        real,
    curve_progress_close_change real,

    curve_progress_avg          real,
    curve_progress_avg_change   real,

    market_cap_open             numeric(36, 12),
    market_cap_open_usd         numeric(36, 12),
    market_cap_open_change      numeric(36, 12),
    market_cap_open_usd_change  numeric(36, 12),
    market_cap_open_percent     real,

    market_cap_high             numeric(36, 12),
    market_cap_high_usd         numeric(36, 12),
    market_cap_high_change      numeric(36, 12),
    market_cap_high_usd_change  numeric(36, 12),
    market_cap_high_percent     real,

    market_cap_low              numeric(36, 12),
    market_cap_low_usd          numeric(36, 12),
    market_cap_low_change       numeric(36, 12),
    market_cap_low_usd_change   numeric(36, 12),
    market_cap_low_percent      real,

    market_cap_close            numeric(36, 12),
    market_cap_close_usd        numeric(36, 12),
    market_cap_close_change     numeric(36, 12),
    market_cap_close_usd_change numeric(36, 12),
    market_cap_close_percent    real,

    market_cap_avg              numeric(36, 12),
    market_cap_avg_usd          numeric(36, 12),
    market_cap_avg_change       numeric(36, 12),
    market_cap_avg_usd_change   numeric(36, 12),
    market_cap_avg_percent      real,

    price_open                  numeric(36, 12),
    price_open_usd              numeric(36, 12),
    price_open_change           numeric(36, 12),
    price_open_usd_change       numeric(36, 12),
    price_open_percent          real,

    price_high                  numeric(36, 12),
    price_high_usd              numeric(36, 12),
    price_high_change           numeric(36, 12),
    price_high_usd_change       numeric(36, 12),
    price_high_percent          real,

    price_low                   numeric(36, 12),
    price_low_usd               numeric(36, 12),
    price_low_change            numeric(36, 12),
    price_low_usd_change        numeric(36, 12),
    price_low_percent           real,

    price_close                 numeric(36, 12),
    price_close_usd             numeric(36, 12),
    price_close_change          numeric(36, 12),
    price_close_usd_change      numeric(36, 12),
    price_close_percent         real,

    price_avg                   numeric(36, 12),
    price_avg_usd               numeric(36, 12),
    price_avg_change            numeric(36, 12),
    price_avg_usd_change        numeric(36, 12),
    price_avg_percent           real,

    trade                       int8,
    trade_change                numeric(36, 12),
    trade_percent               real,
    trade_buy                   int8,
    trade_buy_change            numeric(36, 12),
    trade_buy_percent           real,
    trade_sell                  int8,
    trade_sell_change           numeric(36, 12),
    trade_sell_percent          real,

    volume                      numeric(36, 12),
    volume_usd                  numeric(36, 12),
    volume_change               numeric(36, 12),
    volume_usd_change           numeric(36, 12),
    volume_percent              real,
    volume_buy                  numeric(36, 12),
    volume_buy_usd              numeric(36, 12),
    volume_buy_change           numeric(36, 12),
    volume_buy_usd_change       numeric(36, 12),
    volume_buy_percent          real,
    volume_sell                 numeric(36, 12),
    volume_sell_usd             numeric(36, 12),
    volume_sell_change          numeric(36, 12),
    volume_sell_usd_change      numeric(36, 12),
    volume_sell_percent         real,

    updated_at                  timestamptz default (timezone('utc', now())),

    constraint fk_token_pair foreign key (token_pair_id) references solana.token_pair (id)
);


create trigger set_updated_at
    before update
    on pumpfun.summary_15m
    for each row execute function pumpfun.update_updated_at_column();

create table pumpfun.summary_1h
(
    token_pair_id               int4 primary key,

    amount_base                 numeric(36, 12),
    amount_base_change          numeric(36, 12),
    amount_base_percent         real,
    amount_base_buy             numeric(36, 12),
    amount_base_buy_change      numeric(36, 12),
    amount_base_buy_percent     real,
    amount_base_sell            numeric(36, 12),
    amount_base_sell_change     numeric(36, 12),
    amount_base_sell_percent    real,

    amount_quote                numeric(36, 12),
    amount_quote_change         numeric(36, 12),
    amount_quote_percent        real,
    amount_quote_buy            numeric(36, 12),
    amount_quote_buy_change     numeric(36, 12),
    amount_quote_buy_percent    real,
    amount_quote_sell           numeric(36, 12),
    amount_quote_sell_change    numeric(36, 12),
    amount_quote_sell_percent   real,

    curve_progress_open         real,
    curve_progress_open_change  real,

    curve_progress_high         real,
    curve_progress_high_change  real,

    curve_progress_low          real,
    curve_progress_low_change   real,

    curve_progress_close        real,
    curve_progress_close_change real,

    curve_progress_avg          real,
    curve_progress_avg_change   real,

    market_cap_open             numeric(36, 12),
    market_cap_open_usd         numeric(36, 12),
    market_cap_open_change      numeric(36, 12),
    market_cap_open_usd_change  numeric(36, 12),
    market_cap_open_percent     real,

    market_cap_high             numeric(36, 12),
    market_cap_high_usd         numeric(36, 12),
    market_cap_high_change      numeric(36, 12),
    market_cap_high_usd_change  numeric(36, 12),
    market_cap_high_percent     real,

    market_cap_low              numeric(36, 12),
    market_cap_low_usd          numeric(36, 12),
    market_cap_low_change       numeric(36, 12),
    market_cap_low_usd_change   numeric(36, 12),
    market_cap_low_percent      real,

    market_cap_close            numeric(36, 12),
    market_cap_close_usd        numeric(36, 12),
    market_cap_close_change     numeric(36, 12),
    market_cap_close_usd_change numeric(36, 12),
    market_cap_close_percent    real,

    market_cap_avg              numeric(36, 12),
    market_cap_avg_usd          numeric(36, 12),
    market_cap_avg_change       numeric(36, 12),
    market_cap_avg_usd_change   numeric(36, 12),
    market_cap_avg_percent      real,

    price_open                  numeric(36, 12),
    price_open_usd              numeric(36, 12),
    price_open_change           numeric(36, 12),
    price_open_usd_change       numeric(36, 12),
    price_open_percent          real,

    price_high                  numeric(36, 12),
    price_high_usd              numeric(36, 12),
    price_high_change           numeric(36, 12),
    price_high_usd_change       numeric(36, 12),
    price_high_percent          real,

    price_low                   numeric(36, 12),
    price_low_usd               numeric(36, 12),
    price_low_change            numeric(36, 12),
    price_low_usd_change        numeric(36, 12),
    price_low_percent           real,

    price_close                 numeric(36, 12),
    price_close_usd             numeric(36, 12),
    price_close_change          numeric(36, 12),
    price_close_usd_change      numeric(36, 12),
    price_close_percent         real,

    price_avg                   numeric(36, 12),
    price_avg_usd               numeric(36, 12),
    price_avg_change            numeric(36, 12),
    price_avg_usd_change        numeric(36, 12),
    price_avg_percent           real,

    trade                       int8,
    trade_change                numeric(36, 12),
    trade_percent               real,
    trade_buy                   int8,
    trade_buy_change            numeric(36, 12),
    trade_buy_percent           real,
    trade_sell                  int8,
    trade_sell_change           numeric(36, 12),
    trade_sell_percent          real,

    volume                      numeric(36, 12),
    volume_usd                  numeric(36, 12),
    volume_change               numeric(36, 12),
    volume_usd_change           numeric(36, 12),
    volume_percent              real,
    volume_buy                  numeric(36, 12),
    volume_buy_usd              numeric(36, 12),
    volume_buy_change           numeric(36, 12),
    volume_buy_usd_change       numeric(36, 12),
    volume_buy_percent          real,
    volume_sell                 numeric(36, 12),
    volume_sell_usd             numeric(36, 12),
    volume_sell_change          numeric(36, 12),
    volume_sell_usd_change      numeric(36, 12),
    volume_sell_percent         real,

    updated_at                  timestamptz default (timezone('utc', now())),

    constraint fk_token_pair foreign key (token_pair_id) references solana.token_pair (id)
);


create trigger set_updated_at
    before update
    on pumpfun.summary_1h
    for each row execute function pumpfun.update_updated_at_column();

create table pumpfun.summary_6h
(
    token_pair_id               int4 primary key,

    amount_base                 numeric(36, 12),
    amount_base_change          numeric(36, 12),
    amount_base_percent         real,
    amount_base_buy             numeric(36, 12),
    amount_base_buy_change      numeric(36, 12),
    amount_base_buy_percent     real,
    amount_base_sell            numeric(36, 12),
    amount_base_sell_change     numeric(36, 12),
    amount_base_sell_percent    real,

    amount_quote                numeric(36, 12),
    amount_quote_change         numeric(36, 12),
    amount_quote_percent        real,
    amount_quote_buy            numeric(36, 12),
    amount_quote_buy_change     numeric(36, 12),
    amount_quote_buy_percent    real,
    amount_quote_sell           numeric(36, 12),
    amount_quote_sell_change    numeric(36, 12),
    amount_quote_sell_percent   real,

    curve_progress_open         real,
    curve_progress_open_change  real,

    curve_progress_high         real,
    curve_progress_high_change  real,

    curve_progress_low          real,
    curve_progress_low_change   real,

    curve_progress_close        real,
    curve_progress_close_change real,

    curve_progress_avg          real,
    curve_progress_avg_change   real,

    market_cap_open             numeric(36, 12),
    market_cap_open_usd         numeric(36, 12),
    market_cap_open_change      numeric(36, 12),
    market_cap_open_usd_change  numeric(36, 12),
    market_cap_open_percent     real,

    market_cap_high             numeric(36, 12),
    market_cap_high_usd         numeric(36, 12),
    market_cap_high_change      numeric(36, 12),
    market_cap_high_usd_change  numeric(36, 12),
    market_cap_high_percent     real,

    market_cap_low              numeric(36, 12),
    market_cap_low_usd          numeric(36, 12),
    market_cap_low_change       numeric(36, 12),
    market_cap_low_usd_change   numeric(36, 12),
    market_cap_low_percent      real,

    market_cap_close            numeric(36, 12),
    market_cap_close_usd        numeric(36, 12),
    market_cap_close_change     numeric(36, 12),
    market_cap_close_usd_change numeric(36, 12),
    market_cap_close_percent    real,

    market_cap_avg              numeric(36, 12),
    market_cap_avg_usd          numeric(36, 12),
    market_cap_avg_change       numeric(36, 12),
    market_cap_avg_usd_change   numeric(36, 12),
    market_cap_avg_percent      real,

    price_open                  numeric(36, 12),
    price_open_usd              numeric(36, 12),
    price_open_change           numeric(36, 12),
    price_open_usd_change       numeric(36, 12),
    price_open_percent          real,

    price_high                  numeric(36, 12),
    price_high_usd              numeric(36, 12),
    price_high_change           numeric(36, 12),
    price_high_usd_change       numeric(36, 12),
    price_high_percent          real,

    price_low                   numeric(36, 12),
    price_low_usd               numeric(36, 12),
    price_low_change            numeric(36, 12),
    price_low_usd_change        numeric(36, 12),
    price_low_percent           real,

    price_close                 numeric(36, 12),
    price_close_usd             numeric(36, 12),
    price_close_change          numeric(36, 12),
    price_close_usd_change      numeric(36, 12),
    price_close_percent         real,

    price_avg                   numeric(36, 12),
    price_avg_usd               numeric(36, 12),
    price_avg_change            numeric(36, 12),
    price_avg_usd_change        numeric(36, 12),
    price_avg_percent           real,

    trade                       int8,
    trade_change                numeric(36, 12),
    trade_percent               real,
    trade_buy                   int8,
    trade_buy_change            numeric(36, 12),
    trade_buy_percent           real,
    trade_sell                  int8,
    trade_sell_change           numeric(36, 12),
    trade_sell_percent          real,

    volume                      numeric(36, 12),
    volume_usd                  numeric(36, 12),
    volume_change               numeric(36, 12),
    volume_usd_change           numeric(36, 12),
    volume_percent              real,
    volume_buy                  numeric(36, 12),
    volume_buy_usd              numeric(36, 12),
    volume_buy_change           numeric(36, 12),
    volume_buy_usd_change       numeric(36, 12),
    volume_buy_percent          real,
    volume_sell                 numeric(36, 12),
    volume_sell_usd             numeric(36, 12),
    volume_sell_change          numeric(36, 12),
    volume_sell_usd_change      numeric(36, 12),
    volume_sell_percent         real,

    updated_at                  timestamptz default (timezone('utc', now())),

    constraint fk_token_pair foreign key (token_pair_id) references solana.token_pair (id)
);


create trigger set_updated_at
    before update
    on pumpfun.summary_6h
    for each row execute function pumpfun.update_updated_at_column();

create table pumpfun.summary_1d
(
    token_pair_id               int4 primary key,

    amount_base                 numeric(36, 12),
    amount_base_change          numeric(36, 12),
    amount_base_percent         real,
    amount_base_buy             numeric(36, 12),
    amount_base_buy_change      numeric(36, 12),
    amount_base_buy_percent     real,
    amount_base_sell            numeric(36, 12),
    amount_base_sell_change     numeric(36, 12),
    amount_base_sell_percent    real,

    amount_quote                numeric(36, 12),
    amount_quote_change         numeric(36, 12),
    amount_quote_percent        real,
    amount_quote_buy            numeric(36, 12),
    amount_quote_buy_change     numeric(36, 12),
    amount_quote_buy_percent    real,
    amount_quote_sell           numeric(36, 12),
    amount_quote_sell_change    numeric(36, 12),
    amount_quote_sell_percent   real,

    curve_progress_open         real,
    curve_progress_open_change  real,

    curve_progress_high         real,
    curve_progress_high_change  real,

    curve_progress_low          real,
    curve_progress_low_change   real,

    curve_progress_close        real,
    curve_progress_close_change real,

    curve_progress_avg          real,
    curve_progress_avg_change   real,

    market_cap_open             numeric(36, 12),
    market_cap_open_usd         numeric(36, 12),
    market_cap_open_change      numeric(36, 12),
    market_cap_open_usd_change  numeric(36, 12),
    market_cap_open_percent     real,

    market_cap_high             numeric(36, 12),
    market_cap_high_usd         numeric(36, 12),
    market_cap_high_change      numeric(36, 12),
    market_cap_high_usd_change  numeric(36, 12),
    market_cap_high_percent     real,

    market_cap_low              numeric(36, 12),
    market_cap_low_usd          numeric(36, 12),
    market_cap_low_change       numeric(36, 12),
    market_cap_low_usd_change   numeric(36, 12),
    market_cap_low_percent      real,

    market_cap_close            numeric(36, 12),
    market_cap_close_usd        numeric(36, 12),
    market_cap_close_change     numeric(36, 12),
    market_cap_close_usd_change numeric(36, 12),
    market_cap_close_percent    real,

    market_cap_avg              numeric(36, 12),
    market_cap_avg_usd          numeric(36, 12),
    market_cap_avg_change       numeric(36, 12),
    market_cap_avg_usd_change   numeric(36, 12),
    market_cap_avg_percent      real,

    price_open                  numeric(36, 12),
    price_open_usd              numeric(36, 12),
    price_open_change           numeric(36, 12),
    price_open_usd_change       numeric(36, 12),
    price_open_percent          real,

    price_high                  numeric(36, 12),
    price_high_usd              numeric(36, 12),
    price_high_change           numeric(36, 12),
    price_high_usd_change       numeric(36, 12),
    price_high_percent          real,

    price_low                   numeric(36, 12),
    price_low_usd               numeric(36, 12),
    price_low_change            numeric(36, 12),
    price_low_usd_change        numeric(36, 12),
    price_low_percent           real,

    price_close                 numeric(36, 12),
    price_close_usd             numeric(36, 12),
    price_close_change          numeric(36, 12),
    price_close_usd_change      numeric(36, 12),
    price_close_percent         real,

    price_avg                   numeric(36, 12),
    price_avg_usd               numeric(36, 12),
    price_avg_change            numeric(36, 12),
    price_avg_usd_change        numeric(36, 12),
    price_avg_percent           real,

    trade                       int8,
    trade_change                numeric(36, 12),
    trade_percent               real,
    trade_buy                   int8,
    trade_buy_change            numeric(36, 12),
    trade_buy_percent           real,
    trade_sell                  int8,
    trade_sell_change           numeric(36, 12),
    trade_sell_percent          real,

    volume                      numeric(36, 12),
    volume_usd                  numeric(36, 12),
    volume_change               numeric(36, 12),
    volume_usd_change           numeric(36, 12),
    volume_percent              real,
    volume_buy                  numeric(36, 12),
    volume_buy_usd              numeric(36, 12),
    volume_buy_change           numeric(36, 12),
    volume_buy_usd_change       numeric(36, 12),
    volume_buy_percent          real,
    volume_sell                 numeric(36, 12),
    volume_sell_usd             numeric(36, 12),
    volume_sell_change          numeric(36, 12),
    volume_sell_usd_change      numeric(36, 12),
    volume_sell_percent         real,

    updated_at                  timestamptz default (timezone('utc', now())),

    constraint fk_token_pair foreign key (token_pair_id) references solana.token_pair (id)
);


create trigger set_updated_at
    before update
    on pumpfun.summary_1d
    for each row execute function pumpfun.update_updated_at_column();