-- Copyright (c) nyanbot.com 2025.
-- This file is licensed under the AGPL-3.0-or-later.


create table pumpfun.summary_1m
(
    token_pair_id                  int4 primary key,

    amount                         double precision,
    amount_change                  double precision,
    amount_change_percent          real,
    amount_buy                     double precision,
    amount_buy_change              double precision,
    amount_buy_change_percent      real,
    amount_sell                    double precision,
    amount_sell_change             double precision,
    amount_sell_change_percent     real,

    curve_progress_avg             real,
    curve_progress_avg_change      real,

    curve_progress_high            real,
    curve_progress_high_change     real,

    curve_progress_low             real,
    curve_progress_low_change      real,

    holder_avg                     int4,
    holder_avg_change              int4,
    holder_avg_change_percent      real,

    holder_high                    int4,
    holder_high_change             int4,
    holder_high_change_percent     real,

    holder_low                     int4,
    holder_low_change              int4,
    holder_low_change_percent      real,

    liquidity_avg                  double precision,
    liquidity_avg_usd              double precision,
    liquidity_avg_change           double precision,
    liquidity_avg_usd_change       double precision,
    liquidity_avg_change_percent   real,

    liquidity_high                 double precision,
    liquidity_high_usd             double precision,
    liquidity_high_change          double precision,
    liquidity_high_usd_change      double precision,
    liquidity_high_change_percent  real,

    liquidity_low                  double precision,
    liquidity_low_usd              double precision,
    liquidity_low_change           double precision,
    liquidity_low_usd_change       double precision,
    liquidity_low_change_percent   real,

    market_cap_avg                 double precision,
    market_cap_avg_usd             double precision,
    market_cap_avg_change          double precision,
    market_cap_avg_usd_change      double precision,
    market_cap_avg_change_percent  real,

    market_cap_high                double precision,
    market_cap_high_usd            double precision,
    market_cap_high_change         double precision,
    market_cap_high_usd_change     double precision,
    market_cap_high_change_percent real,

    market_cap_low                 double precision,
    market_cap_low_usd             double precision,
    market_cap_low_change          double precision,
    market_cap_low_usd_change      double precision,
    market_cap_low_change_percent  real,

    price_avg                      double precision,
    price_avg_usd                  double precision,
    price_avg_change               double precision,
    price_avg_usd_change           double precision,
    price_avg_change_percent       real,

    price_high                     double precision,
    price_high_usd                 double precision,
    price_high_change              double precision,
    price_high_usd_change          double precision,
    price_high_change_percent      real,

    price_low                      double precision,
    price_low_usd                  double precision,
    price_low_change               double precision,
    price_low_usd_change           double precision,
    price_low_change_percent       real,

    trades                         int4,
    trades_change                  int4,
    trades_change_percent          real,
    trades_buy                     int4,
    trades_buy_change              int4,
    trades_buy_change_percent      real,
    trades_sell                    int4,
    trades_sell_change             int4,
    trades_sell_change_percent     real,

    volume                         double precision,
    volume_usd                     double precision,
    volume_change                  double precision,
    volume_usd_change              double precision,
    volume_change_percent          real,
    volume_buy                     double precision,
    volume_buy_usd                 double precision,
    volume_buy_change              double precision,
    volume_buy_usd_change          double precision,
    volume_buy_change_percent      real,
    volume_sell                    double precision,
    volume_sell_usd                double precision,
    volume_sell_change             double precision,
    volume_sell_usd_change         double precision,
    volume_sell_change_percent     real,

    updated_at                     timestamptz default (timezone('utc', now())),

    constraint fk_token_pair foreign key (token_pair_id) references solana.token_pair (id)
);


create trigger set_updated_at
    before update
    on pumpfun.summary_1m
    for each row execute function pumpfun.update_updated_at_column();

create table pumpfun.summary_5m
(
    token_pair_id                  int4 primary key,

    amount                         double precision,
    amount_change                  double precision,
    amount_change_percent          real,
    amount_buy                     double precision,
    amount_buy_change              double precision,
    amount_buy_change_percent      real,
    amount_sell                    double precision,
    amount_sell_change             double precision,
    amount_sell_change_percent     real,

    curve_progress_avg             real,
    curve_progress_avg_change      real,

    curve_progress_high            real,
    curve_progress_high_change     real,

    curve_progress_low             real,
    curve_progress_low_change      real,

    holder_avg                     int4,
    holder_avg_change              int4,
    holder_avg_change_percent      real,

    holder_high                    int4,
    holder_high_change             int4,
    holder_high_change_percent     real,

    holder_low                     int4,
    holder_low_change              int4,
    holder_low_change_percent      real,

    liquidity_avg                  double precision,
    liquidity_avg_usd              double precision,
    liquidity_avg_change           double precision,
    liquidity_avg_usd_change       double precision,
    liquidity_avg_change_percent   real,

    liquidity_high                 double precision,
    liquidity_high_usd             double precision,
    liquidity_high_change          double precision,
    liquidity_high_usd_change      double precision,
    liquidity_high_change_percent  real,

    liquidity_low                  double precision,
    liquidity_low_usd              double precision,
    liquidity_low_change           double precision,
    liquidity_low_usd_change       double precision,
    liquidity_low_change_percent   real,

    market_cap_avg                 double precision,
    market_cap_avg_usd             double precision,
    market_cap_avg_change          double precision,
    market_cap_avg_usd_change      double precision,
    market_cap_avg_change_percent  real,

    market_cap_high                double precision,
    market_cap_high_usd            double precision,
    market_cap_high_change         double precision,
    market_cap_high_usd_change     double precision,
    market_cap_high_change_percent real,

    market_cap_low                 double precision,
    market_cap_low_usd             double precision,
    market_cap_low_change          double precision,
    market_cap_low_usd_change      double precision,
    market_cap_low_change_percent  real,

    price_avg                      double precision,
    price_avg_usd                  double precision,
    price_avg_change               double precision,
    price_avg_usd_change           double precision,
    price_avg_change_percent       real,

    price_high                     double precision,
    price_high_usd                 double precision,
    price_high_change              double precision,
    price_high_usd_change          double precision,
    price_high_change_percent      real,

    price_low                      double precision,
    price_low_usd                  double precision,
    price_low_change               double precision,
    price_low_usd_change           double precision,
    price_low_change_percent       real,

    trades                         int4,
    trades_change                  int4,
    trades_change_percent          real,
    trades_buy                     int4,
    trades_buy_change              int4,
    trades_buy_change_percent      real,
    trades_sell                    int4,
    trades_sell_change             int4,
    trades_sell_change_percent     real,

    volume                         double precision,
    volume_usd                     double precision,
    volume_change                  double precision,
    volume_usd_change              double precision,
    volume_change_percent          real,
    volume_buy                     double precision,
    volume_buy_usd                 double precision,
    volume_buy_change              double precision,
    volume_buy_usd_change          double precision,
    volume_buy_change_percent      real,
    volume_sell                    double precision,
    volume_sell_usd                double precision,
    volume_sell_change             double precision,
    volume_sell_usd_change         double precision,
    volume_sell_change_percent     real,

    updated_at                     timestamptz default (timezone('utc', now())),

    constraint fk_token_pair foreign key (token_pair_id) references solana.token_pair (id)
);


create trigger set_updated_at
    before update
    on pumpfun.summary_5m
    for each row execute function pumpfun.update_updated_at_column();

create table pumpfun.summary_15m
(
    token_pair_id                  int4 primary key,

    amount                         double precision,
    amount_change                  double precision,
    amount_change_percent          real,
    amount_buy                     double precision,
    amount_buy_change              double precision,
    amount_buy_change_percent      real,
    amount_sell                    double precision,
    amount_sell_change             double precision,
    amount_sell_change_percent     real,

    curve_progress_avg             real,
    curve_progress_avg_change      real,

    curve_progress_high            real,
    curve_progress_high_change     real,

    curve_progress_low             real,
    curve_progress_low_change      real,

    holder_avg                     int4,
    holder_avg_change              int4,
    holder_avg_change_percent      real,

    holder_high                    int4,
    holder_high_change             int4,
    holder_high_change_percent     real,

    holder_low                     int4,
    holder_low_change              int4,
    holder_low_change_percent      real,

    liquidity_avg                  double precision,
    liquidity_avg_usd              double precision,
    liquidity_avg_change           double precision,
    liquidity_avg_usd_change       double precision,
    liquidity_avg_change_percent   real,

    liquidity_high                 double precision,
    liquidity_high_usd             double precision,
    liquidity_high_change          double precision,
    liquidity_high_usd_change      double precision,
    liquidity_high_change_percent  real,

    liquidity_low                  double precision,
    liquidity_low_usd              double precision,
    liquidity_low_change           double precision,
    liquidity_low_usd_change       double precision,
    liquidity_low_change_percent   real,

    market_cap_avg                 double precision,
    market_cap_avg_usd             double precision,
    market_cap_avg_change          double precision,
    market_cap_avg_usd_change      double precision,
    market_cap_avg_change_percent  real,

    market_cap_high                double precision,
    market_cap_high_usd            double precision,
    market_cap_high_change         double precision,
    market_cap_high_usd_change     double precision,
    market_cap_high_change_percent real,

    market_cap_low                 double precision,
    market_cap_low_usd             double precision,
    market_cap_low_change          double precision,
    market_cap_low_usd_change      double precision,
    market_cap_low_change_percent  real,

    price_avg                      double precision,
    price_avg_usd                  double precision,
    price_avg_change               double precision,
    price_avg_usd_change           double precision,
    price_avg_change_percent       real,

    price_high                     double precision,
    price_high_usd                 double precision,
    price_high_change              double precision,
    price_high_usd_change          double precision,
    price_high_change_percent      real,

    price_low                      double precision,
    price_low_usd                  double precision,
    price_low_change               double precision,
    price_low_usd_change           double precision,
    price_low_change_percent       real,

    trades                         int4,
    trades_change                  int4,
    trades_change_percent          real,
    trades_buy                     int4,
    trades_buy_change              int4,
    trades_buy_change_percent      real,
    trades_sell                    int4,
    trades_sell_change             int4,
    trades_sell_change_percent     real,

    volume                         double precision,
    volume_usd                     double precision,
    volume_change                  double precision,
    volume_usd_change              double precision,
    volume_change_percent          real,
    volume_buy                     double precision,
    volume_buy_usd                 double precision,
    volume_buy_change              double precision,
    volume_buy_usd_change          double precision,
    volume_buy_change_percent      real,
    volume_sell                    double precision,
    volume_sell_usd                double precision,
    volume_sell_change             double precision,
    volume_sell_usd_change         double precision,
    volume_sell_change_percent     real,

    updated_at                     timestamptz default (timezone('utc', now())),

    constraint fk_token_pair foreign key (token_pair_id) references solana.token_pair (id)
);


create trigger set_updated_at
    before update
    on pumpfun.summary_15m
    for each row execute function pumpfun.update_updated_at_column();

create table pumpfun.summary_1h
(
    token_pair_id                  int4 primary key,

    amount                         double precision,
    amount_change                  double precision,
    amount_change_percent          real,
    amount_buy                     double precision,
    amount_buy_change              double precision,
    amount_buy_change_percent      real,
    amount_sell                    double precision,
    amount_sell_change             double precision,
    amount_sell_change_percent     real,

    curve_progress_avg             real,
    curve_progress_avg_change      real,

    curve_progress_high            real,
    curve_progress_high_change     real,

    curve_progress_low             real,
    curve_progress_low_change      real,

    holder_avg                     int4,
    holder_avg_change              int4,
    holder_avg_change_percent      real,

    holder_high                    int4,
    holder_high_change             int4,
    holder_high_change_percent     real,

    holder_low                     int4,
    holder_low_change              int4,
    holder_low_change_percent      real,

    liquidity_avg                  double precision,
    liquidity_avg_usd              double precision,
    liquidity_avg_change           double precision,
    liquidity_avg_usd_change       double precision,
    liquidity_avg_change_percent   real,

    liquidity_high                 double precision,
    liquidity_high_usd             double precision,
    liquidity_high_change          double precision,
    liquidity_high_usd_change      double precision,
    liquidity_high_change_percent  real,

    liquidity_low                  double precision,
    liquidity_low_usd              double precision,
    liquidity_low_change           double precision,
    liquidity_low_usd_change       double precision,
    liquidity_low_change_percent   real,

    market_cap_avg                 double precision,
    market_cap_avg_usd             double precision,
    market_cap_avg_change          double precision,
    market_cap_avg_usd_change      double precision,
    market_cap_avg_change_percent  real,

    market_cap_high                double precision,
    market_cap_high_usd            double precision,
    market_cap_high_change         double precision,
    market_cap_high_usd_change     double precision,
    market_cap_high_change_percent real,

    market_cap_low                 double precision,
    market_cap_low_usd             double precision,
    market_cap_low_change          double precision,
    market_cap_low_usd_change      double precision,
    market_cap_low_change_percent  real,

    price_avg                      double precision,
    price_avg_usd                  double precision,
    price_avg_change               double precision,
    price_avg_usd_change           double precision,
    price_avg_change_percent       real,

    price_high                     double precision,
    price_high_usd                 double precision,
    price_high_change              double precision,
    price_high_usd_change          double precision,
    price_high_change_percent      real,

    price_low                      double precision,
    price_low_usd                  double precision,
    price_low_change               double precision,
    price_low_usd_change           double precision,
    price_low_change_percent       real,

    trades                         int4,
    trades_change                  int4,
    trades_change_percent          real,
    trades_buy                     int4,
    trades_buy_change              int4,
    trades_buy_change_percent      real,
    trades_sell                    int4,
    trades_sell_change             int4,
    trades_sell_change_percent     real,

    volume                         double precision,
    volume_usd                     double precision,
    volume_change                  double precision,
    volume_usd_change              double precision,
    volume_change_percent          real,
    volume_buy                     double precision,
    volume_buy_usd                 double precision,
    volume_buy_change              double precision,
    volume_buy_usd_change          double precision,
    volume_buy_change_percent      real,
    volume_sell                    double precision,
    volume_sell_usd                double precision,
    volume_sell_change             double precision,
    volume_sell_usd_change         double precision,
    volume_sell_change_percent     real,

    updated_at                     timestamptz default (timezone('utc', now())),

    constraint fk_token_pair foreign key (token_pair_id) references solana.token_pair (id)
);


create trigger set_updated_at
    before update
    on pumpfun.summary_1h
    for each row execute function pumpfun.update_updated_at_column();

create table pumpfun.summary_6h
(
    token_pair_id                  int4 primary key,

    amount                         double precision,
    amount_change                  double precision,
    amount_change_percent          real,
    amount_buy                     double precision,
    amount_buy_change              double precision,
    amount_buy_change_percent      real,
    amount_sell                    double precision,
    amount_sell_change             double precision,
    amount_sell_change_percent     real,

    curve_progress_avg             real,
    curve_progress_avg_change      real,

    curve_progress_high            real,
    curve_progress_high_change     real,

    curve_progress_low             real,
    curve_progress_low_change      real,

    holder_avg                     int4,
    holder_avg_change              int4,
    holder_avg_change_percent      real,

    holder_high                    int4,
    holder_high_change             int4,
    holder_high_change_percent     real,

    holder_low                     int4,
    holder_low_change              int4,
    holder_low_change_percent      real,

    liquidity_avg                  double precision,
    liquidity_avg_usd              double precision,
    liquidity_avg_change           double precision,
    liquidity_avg_usd_change       double precision,
    liquidity_avg_change_percent   real,

    liquidity_high                 double precision,
    liquidity_high_usd             double precision,
    liquidity_high_change          double precision,
    liquidity_high_usd_change      double precision,
    liquidity_high_change_percent  real,

    liquidity_low                  double precision,
    liquidity_low_usd              double precision,
    liquidity_low_change           double precision,
    liquidity_low_usd_change       double precision,
    liquidity_low_change_percent   real,

    market_cap_avg                 double precision,
    market_cap_avg_usd             double precision,
    market_cap_avg_change          double precision,
    market_cap_avg_usd_change      double precision,
    market_cap_avg_change_percent  real,

    market_cap_high                double precision,
    market_cap_high_usd            double precision,
    market_cap_high_change         double precision,
    market_cap_high_usd_change     double precision,
    market_cap_high_change_percent real,

    market_cap_low                 double precision,
    market_cap_low_usd             double precision,
    market_cap_low_change          double precision,
    market_cap_low_usd_change      double precision,
    market_cap_low_change_percent  real,

    price_avg                      double precision,
    price_avg_usd                  double precision,
    price_avg_change               double precision,
    price_avg_usd_change           double precision,
    price_avg_change_percent       real,

    price_high                     double precision,
    price_high_usd                 double precision,
    price_high_change              double precision,
    price_high_usd_change          double precision,
    price_high_change_percent      real,

    price_low                      double precision,
    price_low_usd                  double precision,
    price_low_change               double precision,
    price_low_usd_change           double precision,
    price_low_change_percent       real,

    trades                         int4,
    trades_change                  int4,
    trades_change_percent          real,
    trades_buy                     int4,
    trades_buy_change              int4,
    trades_buy_change_percent      real,
    trades_sell                    int4,
    trades_sell_change             int4,
    trades_sell_change_percent     real,

    volume                         double precision,
    volume_usd                     double precision,
    volume_change                  double precision,
    volume_usd_change              double precision,
    volume_change_percent          real,
    volume_buy                     double precision,
    volume_buy_usd                 double precision,
    volume_buy_change              double precision,
    volume_buy_usd_change          double precision,
    volume_buy_change_percent      real,
    volume_sell                    double precision,
    volume_sell_usd                double precision,
    volume_sell_change             double precision,
    volume_sell_usd_change         double precision,
    volume_sell_change_percent     real,

    updated_at                     timestamptz default (timezone('utc', now())),

    constraint fk_token_pair foreign key (token_pair_id) references solana.token_pair (id)
);


create trigger set_updated_at
    before update
    on pumpfun.summary_6h
    for each row execute function pumpfun.update_updated_at_column();

create table pumpfun.summary_1d
(
    token_pair_id                  int4 primary key,

    amount                         double precision,
    amount_change                  double precision,
    amount_change_percent          real,
    amount_buy                     double precision,
    amount_buy_change              double precision,
    amount_buy_change_percent      real,
    amount_sell                    double precision,
    amount_sell_change             double precision,
    amount_sell_change_percent     real,

    curve_progress_avg             real,
    curve_progress_avg_change      real,

    curve_progress_high            real,
    curve_progress_high_change     real,

    curve_progress_low             real,
    curve_progress_low_change      real,

    holder_avg                     int4,
    holder_avg_change              int4,
    holder_avg_change_percent      real,

    holder_high                    int4,
    holder_high_change             int4,
    holder_high_change_percent     real,

    holder_low                     int4,
    holder_low_change              int4,
    holder_low_change_percent      real,

    liquidity_avg                  double precision,
    liquidity_avg_usd              double precision,
    liquidity_avg_change           double precision,
    liquidity_avg_usd_change       double precision,
    liquidity_avg_change_percent   real,

    liquidity_high                 double precision,
    liquidity_high_usd             double precision,
    liquidity_high_change          double precision,
    liquidity_high_usd_change      double precision,
    liquidity_high_change_percent  real,

    liquidity_low                  double precision,
    liquidity_low_usd              double precision,
    liquidity_low_change           double precision,
    liquidity_low_usd_change       double precision,
    liquidity_low_change_percent   real,

    market_cap_avg                 double precision,
    market_cap_avg_usd             double precision,
    market_cap_avg_change          double precision,
    market_cap_avg_usd_change      double precision,
    market_cap_avg_change_percent  real,

    market_cap_high                double precision,
    market_cap_high_usd            double precision,
    market_cap_high_change         double precision,
    market_cap_high_usd_change     double precision,
    market_cap_high_change_percent real,

    market_cap_low                 double precision,
    market_cap_low_usd             double precision,
    market_cap_low_change          double precision,
    market_cap_low_usd_change      double precision,
    market_cap_low_change_percent  real,

    price_avg                      double precision,
    price_avg_usd                  double precision,
    price_avg_change               double precision,
    price_avg_usd_change           double precision,
    price_avg_change_percent       real,

    price_high                     double precision,
    price_high_usd                 double precision,
    price_high_change              double precision,
    price_high_usd_change          double precision,
    price_high_change_percent      real,

    price_low                      double precision,
    price_low_usd                  double precision,
    price_low_change               double precision,
    price_low_usd_change           double precision,
    price_low_change_percent       real,

    trades                         int4,
    trades_change                  int4,
    trades_change_percent          real,
    trades_buy                     int4,
    trades_buy_change              int4,
    trades_buy_change_percent      real,
    trades_sell                    int4,
    trades_sell_change             int4,
    trades_sell_change_percent     real,

    volume                         double precision,
    volume_usd                     double precision,
    volume_change                  double precision,
    volume_usd_change              double precision,
    volume_change_percent          real,
    volume_buy                     double precision,
    volume_buy_usd                 double precision,
    volume_buy_change              double precision,
    volume_buy_usd_change          double precision,
    volume_buy_change_percent      real,
    volume_sell                    double precision,
    volume_sell_usd                double precision,
    volume_sell_change             double precision,
    volume_sell_usd_change         double precision,
    volume_sell_change_percent     real,

    updated_at                     timestamptz default (timezone('utc', now())),

    constraint fk_token_pair foreign key (token_pair_id) references solana.token_pair (id)
);


create trigger set_updated_at
    before update
    on pumpfun.summary_1d
    for each row execute function pumpfun.update_updated_at_column();