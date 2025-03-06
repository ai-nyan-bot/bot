-- Copyright (c) nyanbot.com 2025.
-- This file is licensed under the AGPL-3.0-or-later.


create table pumpfun.summary_1m
(
    token_pair_id               int4 primary key,

    amount_base                      double precision,
    amount_base_change               double precision,
    amount_base_percent              real,
    amount_base_buy                  double precision,
    amount_base_buy_change           double precision,
    amount_base_buy_percent          real,
    amount_base_sell                 double precision,
    amount_base_sell_change          double precision,
    amount_base_sell_percent         real,

    amount_quote                      double precision,
    amount_quote_change               double precision,
    amount_quote_percent              real,
    amount_quote_buy                  double precision,
    amount_quote_buy_change           double precision,
    amount_quote_buy_percent          real,
    amount_quote_sell                 double precision,
    amount_quote_sell_change          double precision,
    amount_quote_sell_percent         real,


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

    holder_open                 int4,
    holder_open_change          double precision,
    holder_open_percent         real,

    holder_high                 int4,
    holder_high_change          double precision,
    holder_high_percent         real,

    holder_low                  int4,
    holder_low_change           double precision,
    holder_low_percent          real,

    holder_close                int4,
    holder_close_change         double precision,
    holder_close_percent        real,

    holder_avg                  int4,
    holder_avg_change           double precision,
    holder_avg_percent          real,

    market_cap_open             double precision,
    market_cap_open_usd         double precision,
    market_cap_open_change      double precision,
    market_cap_open_usd_change  double precision,
    market_cap_open_percent     real,

    market_cap_high             double precision,
    market_cap_high_usd         double precision,
    market_cap_high_change      double precision,
    market_cap_high_usd_change  double precision,
    market_cap_high_percent     real,

    market_cap_low              double precision,
    market_cap_low_usd          double precision,
    market_cap_low_change       double precision,
    market_cap_low_usd_change   double precision,
    market_cap_low_percent      real,

    market_cap_close            double precision,
    market_cap_close_usd        double precision,
    market_cap_close_change     double precision,
    market_cap_close_usd_change double precision,
    market_cap_close_percent    real,

    market_cap_avg              double precision,
    market_cap_avg_usd          double precision,
    market_cap_avg_change       double precision,
    market_cap_avg_usd_change   double precision,
    market_cap_avg_percent      real,

    price_open                  double precision,
    price_open_usd              double precision,
    price_open_change           double precision,
    price_open_usd_change       double precision,
    price_open_percent          real,

    price_high                  double precision,
    price_high_usd              double precision,
    price_high_change           double precision,
    price_high_usd_change       double precision,
    price_high_percent          real,

    price_low                   double precision,
    price_low_usd               double precision,
    price_low_change            double precision,
    price_low_usd_change        double precision,
    price_low_percent           real,

    price_close                 double precision,
    price_close_usd             double precision,
    price_close_change          double precision,
    price_close_usd_change      double precision,
    price_close_percent         real,

    price_avg                   double precision,
    price_avg_usd               double precision,
    price_avg_change            double precision,
    price_avg_usd_change        double precision,
    price_avg_percent           real,

    trade                       int4,
    trade_change                double precision,
    trade_percent               real,
    trade_buy                   int4,
    trade_buy_change            double precision,
    trade_buy_percent           real,
    trade_sell                  int4,
    trade_sell_change           double precision,
    trade_sell_percent          real,

    volume                      double precision,
    volume_usd                  double precision,
    volume_change               double precision,
    volume_usd_change           double precision,
    volume_percent              real,
    volume_buy                  double precision,
    volume_buy_usd              double precision,
    volume_buy_change           double precision,
    volume_buy_usd_change       double precision,
    volume_buy_percent          real,
    volume_sell                 double precision,
    volume_sell_usd             double precision,
    volume_sell_change          double precision,
    volume_sell_usd_change      double precision,
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

    amount_base                      double precision,
    amount_base_change               double precision,
    amount_base_percent              real,
    amount_base_buy                  double precision,
    amount_base_buy_change           double precision,
    amount_base_buy_percent          real,
    amount_base_sell                 double precision,
    amount_base_sell_change          double precision,
    amount_base_sell_percent         real,

    amount_quote                      double precision,
    amount_quote_change               double precision,
    amount_quote_percent              real,
    amount_quote_buy                  double precision,
    amount_quote_buy_change           double precision,
    amount_quote_buy_percent          real,
    amount_quote_sell                 double precision,
    amount_quote_sell_change          double precision,
    amount_quote_sell_percent         real,

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

    holder_open                 int4,
    holder_open_change          double precision,
    holder_open_percent         real,

    holder_high                 int4,
    holder_high_change          double precision,
    holder_high_percent         real,

    holder_low                  int4,
    holder_low_change           double precision,
    holder_low_percent          real,

    holder_close                int4,
    holder_close_change         double precision,
    holder_close_percent        real,

    holder_avg                  int4,
    holder_avg_change           double precision,
    holder_avg_percent          real,

    market_cap_open             double precision,
    market_cap_open_usd         double precision,
    market_cap_open_change      double precision,
    market_cap_open_usd_change  double precision,
    market_cap_open_percent     real,

    market_cap_high             double precision,
    market_cap_high_usd         double precision,
    market_cap_high_change      double precision,
    market_cap_high_usd_change  double precision,
    market_cap_high_percent     real,

    market_cap_low              double precision,
    market_cap_low_usd          double precision,
    market_cap_low_change       double precision,
    market_cap_low_usd_change   double precision,
    market_cap_low_percent      real,

    market_cap_close            double precision,
    market_cap_close_usd        double precision,
    market_cap_close_change     double precision,
    market_cap_close_usd_change double precision,
    market_cap_close_percent    real,

    market_cap_avg              double precision,
    market_cap_avg_usd          double precision,
    market_cap_avg_change       double precision,
    market_cap_avg_usd_change   double precision,
    market_cap_avg_percent      real,

    price_open                  double precision,
    price_open_usd              double precision,
    price_open_change           double precision,
    price_open_usd_change       double precision,
    price_open_percent          real,

    price_high                  double precision,
    price_high_usd              double precision,
    price_high_change           double precision,
    price_high_usd_change       double precision,
    price_high_percent          real,

    price_low                   double precision,
    price_low_usd               double precision,
    price_low_change            double precision,
    price_low_usd_change        double precision,
    price_low_percent           real,

    price_close                 double precision,
    price_close_usd             double precision,
    price_close_change          double precision,
    price_close_usd_change      double precision,
    price_close_percent         real,

    price_avg                   double precision,
    price_avg_usd               double precision,
    price_avg_change            double precision,
    price_avg_usd_change        double precision,
    price_avg_percent           real,

    trade                       int4,
    trade_change                double precision,
    trade_percent               real,
    trade_buy                   int4,
    trade_buy_change            double precision,
    trade_buy_percent           real,
    trade_sell                  int4,
    trade_sell_change           double precision,
    trade_sell_percent          real,

    volume                      double precision,
    volume_usd                  double precision,
    volume_change               double precision,
    volume_usd_change           double precision,
    volume_percent              real,
    volume_buy                  double precision,
    volume_buy_usd              double precision,
    volume_buy_change           double precision,
    volume_buy_usd_change       double precision,
    volume_buy_percent          real,
    volume_sell                 double precision,
    volume_sell_usd             double precision,
    volume_sell_change          double precision,
    volume_sell_usd_change      double precision,
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

    amount_base                      double precision,
    amount_base_change               double precision,
    amount_base_percent              real,
    amount_base_buy                  double precision,
    amount_base_buy_change           double precision,
    amount_base_buy_percent          real,
    amount_base_sell                 double precision,
    amount_base_sell_change          double precision,
    amount_base_sell_percent         real,

    amount_quote                      double precision,
    amount_quote_change               double precision,
    amount_quote_percent              real,
    amount_quote_buy                  double precision,
    amount_quote_buy_change           double precision,
    amount_quote_buy_percent          real,
    amount_quote_sell                 double precision,
    amount_quote_sell_change          double precision,
    amount_quote_sell_percent         real,

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

    holder_open                 int4,
    holder_open_change          double precision,
    holder_open_percent         real,

    holder_high                 int4,
    holder_high_change          double precision,
    holder_high_percent         real,

    holder_low                  int4,
    holder_low_change           double precision,
    holder_low_percent          real,

    holder_close                int4,
    holder_close_change         double precision,
    holder_close_percent        real,

    holder_avg                  int4,
    holder_avg_change           double precision,
    holder_avg_percent          real,

    market_cap_open             double precision,
    market_cap_open_usd         double precision,
    market_cap_open_change      double precision,
    market_cap_open_usd_change  double precision,
    market_cap_open_percent     real,

    market_cap_high             double precision,
    market_cap_high_usd         double precision,
    market_cap_high_change      double precision,
    market_cap_high_usd_change  double precision,
    market_cap_high_percent     real,

    market_cap_low              double precision,
    market_cap_low_usd          double precision,
    market_cap_low_change       double precision,
    market_cap_low_usd_change   double precision,
    market_cap_low_percent      real,

    market_cap_close            double precision,
    market_cap_close_usd        double precision,
    market_cap_close_change     double precision,
    market_cap_close_usd_change double precision,
    market_cap_close_percent    real,

    market_cap_avg              double precision,
    market_cap_avg_usd          double precision,
    market_cap_avg_change       double precision,
    market_cap_avg_usd_change   double precision,
    market_cap_avg_percent      real,

    price_open                  double precision,
    price_open_usd              double precision,
    price_open_change           double precision,
    price_open_usd_change       double precision,
    price_open_percent          real,

    price_high                  double precision,
    price_high_usd              double precision,
    price_high_change           double precision,
    price_high_usd_change       double precision,
    price_high_percent          real,

    price_low                   double precision,
    price_low_usd               double precision,
    price_low_change            double precision,
    price_low_usd_change        double precision,
    price_low_percent           real,

    price_close                 double precision,
    price_close_usd             double precision,
    price_close_change          double precision,
    price_close_usd_change      double precision,
    price_close_percent         real,

    price_avg                   double precision,
    price_avg_usd               double precision,
    price_avg_change            double precision,
    price_avg_usd_change        double precision,
    price_avg_percent           real,

    trade                       int4,
    trade_change                double precision,
    trade_percent               real,
    trade_buy                   int4,
    trade_buy_change            double precision,
    trade_buy_percent           real,
    trade_sell                  int4,
    trade_sell_change           double precision,
    trade_sell_percent          real,

    volume                      double precision,
    volume_usd                  double precision,
    volume_change               double precision,
    volume_usd_change           double precision,
    volume_percent              real,
    volume_buy                  double precision,
    volume_buy_usd              double precision,
    volume_buy_change           double precision,
    volume_buy_usd_change       double precision,
    volume_buy_percent          real,
    volume_sell                 double precision,
    volume_sell_usd             double precision,
    volume_sell_change          double precision,
    volume_sell_usd_change      double precision,
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

    amount_base                      double precision,
    amount_base_change               double precision,
    amount_base_percent              real,
    amount_base_buy                  double precision,
    amount_base_buy_change           double precision,
    amount_base_buy_percent          real,
    amount_base_sell                 double precision,
    amount_base_sell_change          double precision,
    amount_base_sell_percent         real,

    amount_quote                      double precision,
    amount_quote_change               double precision,
    amount_quote_percent              real,
    amount_quote_buy                  double precision,
    amount_quote_buy_change           double precision,
    amount_quote_buy_percent          real,
    amount_quote_sell                 double precision,
    amount_quote_sell_change          double precision,
    amount_quote_sell_percent         real,

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

    holder_open                 int4,
    holder_open_change          double precision,
    holder_open_percent         real,

    holder_high                 int4,
    holder_high_change          double precision,
    holder_high_percent         real,

    holder_low                  int4,
    holder_low_change           double precision,
    holder_low_percent          real,

    holder_close                int4,
    holder_close_change         double precision,
    holder_close_percent        real,

    holder_avg                  int4,
    holder_avg_change           double precision,
    holder_avg_percent          real,

    market_cap_open             double precision,
    market_cap_open_usd         double precision,
    market_cap_open_change      double precision,
    market_cap_open_usd_change  double precision,
    market_cap_open_percent     real,

    market_cap_high             double precision,
    market_cap_high_usd         double precision,
    market_cap_high_change      double precision,
    market_cap_high_usd_change  double precision,
    market_cap_high_percent     real,

    market_cap_low              double precision,
    market_cap_low_usd          double precision,
    market_cap_low_change       double precision,
    market_cap_low_usd_change   double precision,
    market_cap_low_percent      real,

    market_cap_close            double precision,
    market_cap_close_usd        double precision,
    market_cap_close_change     double precision,
    market_cap_close_usd_change double precision,
    market_cap_close_percent    real,

    market_cap_avg              double precision,
    market_cap_avg_usd          double precision,
    market_cap_avg_change       double precision,
    market_cap_avg_usd_change   double precision,
    market_cap_avg_percent      real,

    price_open                  double precision,
    price_open_usd              double precision,
    price_open_change           double precision,
    price_open_usd_change       double precision,
    price_open_percent          real,

    price_high                  double precision,
    price_high_usd              double precision,
    price_high_change           double precision,
    price_high_usd_change       double precision,
    price_high_percent          real,

    price_low                   double precision,
    price_low_usd               double precision,
    price_low_change            double precision,
    price_low_usd_change        double precision,
    price_low_percent           real,

    price_close                 double precision,
    price_close_usd             double precision,
    price_close_change          double precision,
    price_close_usd_change      double precision,
    price_close_percent         real,

    price_avg                   double precision,
    price_avg_usd               double precision,
    price_avg_change            double precision,
    price_avg_usd_change        double precision,
    price_avg_percent           real,

    trade                       int4,
    trade_change                double precision,
    trade_percent               real,
    trade_buy                   int4,
    trade_buy_change            double precision,
    trade_buy_percent           real,
    trade_sell                  int4,
    trade_sell_change           double precision,
    trade_sell_percent          real,

    volume                      double precision,
    volume_usd                  double precision,
    volume_change               double precision,
    volume_usd_change           double precision,
    volume_percent              real,
    volume_buy                  double precision,
    volume_buy_usd              double precision,
    volume_buy_change           double precision,
    volume_buy_usd_change       double precision,
    volume_buy_percent          real,
    volume_sell                 double precision,
    volume_sell_usd             double precision,
    volume_sell_change          double precision,
    volume_sell_usd_change      double precision,
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

    amount_base                      double precision,
    amount_base_change               double precision,
    amount_base_percent              real,
    amount_base_buy                  double precision,
    amount_base_buy_change           double precision,
    amount_base_buy_percent          real,
    amount_base_sell                 double precision,
    amount_base_sell_change          double precision,
    amount_base_sell_percent         real,

    amount_quote                      double precision,
    amount_quote_change               double precision,
    amount_quote_percent              real,
    amount_quote_buy                  double precision,
    amount_quote_buy_change           double precision,
    amount_quote_buy_percent          real,
    amount_quote_sell                 double precision,
    amount_quote_sell_change          double precision,
    amount_quote_sell_percent         real,

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

    holder_open                 int4,
    holder_open_change          double precision,
    holder_open_percent         real,

    holder_high                 int4,
    holder_high_change          double precision,
    holder_high_percent         real,

    holder_low                  int4,
    holder_low_change           double precision,
    holder_low_percent          real,

    holder_close                int4,
    holder_close_change         double precision,
    holder_close_percent        real,

    holder_avg                  int4,
    holder_avg_change           double precision,
    holder_avg_percent          real,

    market_cap_open             double precision,
    market_cap_open_usd         double precision,
    market_cap_open_change      double precision,
    market_cap_open_usd_change  double precision,
    market_cap_open_percent     real,

    market_cap_high             double precision,
    market_cap_high_usd         double precision,
    market_cap_high_change      double precision,
    market_cap_high_usd_change  double precision,
    market_cap_high_percent     real,

    market_cap_low              double precision,
    market_cap_low_usd          double precision,
    market_cap_low_change       double precision,
    market_cap_low_usd_change   double precision,
    market_cap_low_percent      real,

    market_cap_close            double precision,
    market_cap_close_usd        double precision,
    market_cap_close_change     double precision,
    market_cap_close_usd_change double precision,
    market_cap_close_percent    real,

    market_cap_avg              double precision,
    market_cap_avg_usd          double precision,
    market_cap_avg_change       double precision,
    market_cap_avg_usd_change   double precision,
    market_cap_avg_percent      real,

    price_open                  double precision,
    price_open_usd              double precision,
    price_open_change           double precision,
    price_open_usd_change       double precision,
    price_open_percent          real,

    price_high                  double precision,
    price_high_usd              double precision,
    price_high_change           double precision,
    price_high_usd_change       double precision,
    price_high_percent          real,

    price_low                   double precision,
    price_low_usd               double precision,
    price_low_change            double precision,
    price_low_usd_change        double precision,
    price_low_percent           real,

    price_close                 double precision,
    price_close_usd             double precision,
    price_close_change          double precision,
    price_close_usd_change      double precision,
    price_close_percent         real,

    price_avg                   double precision,
    price_avg_usd               double precision,
    price_avg_change            double precision,
    price_avg_usd_change        double precision,
    price_avg_percent           real,

    trade                       int4,
    trade_change                double precision,
    trade_percent               real,
    trade_buy                   int4,
    trade_buy_change            double precision,
    trade_buy_percent           real,
    trade_sell                  int4,
    trade_sell_change           double precision,
    trade_sell_percent          real,

    volume                      double precision,
    volume_usd                  double precision,
    volume_change               double precision,
    volume_usd_change           double precision,
    volume_percent              real,
    volume_buy                  double precision,
    volume_buy_usd              double precision,
    volume_buy_change           double precision,
    volume_buy_usd_change       double precision,
    volume_buy_percent          real,
    volume_sell                 double precision,
    volume_sell_usd             double precision,
    volume_sell_change          double precision,
    volume_sell_usd_change      double precision,
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

    amount_base                      double precision,
    amount_base_change               double precision,
    amount_base_percent              real,
    amount_base_buy                  double precision,
    amount_base_buy_change           double precision,
    amount_base_buy_percent          real,
    amount_base_sell                 double precision,
    amount_base_sell_change          double precision,
    amount_base_sell_percent         real,

    amount_quote                      double precision,
    amount_quote_change               double precision,
    amount_quote_percent              real,
    amount_quote_buy                  double precision,
    amount_quote_buy_change           double precision,
    amount_quote_buy_percent          real,
    amount_quote_sell                 double precision,
    amount_quote_sell_change          double precision,
    amount_quote_sell_percent         real,

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

    holder_open                 int4,
    holder_open_change          double precision,
    holder_open_percent         real,

    holder_high                 int4,
    holder_high_change          double precision,
    holder_high_percent         real,

    holder_low                  int4,
    holder_low_change           double precision,
    holder_low_percent          real,

    holder_close                int4,
    holder_close_change         double precision,
    holder_close_percent        real,

    holder_avg                  int4,
    holder_avg_change           double precision,
    holder_avg_percent          real,

    market_cap_open             double precision,
    market_cap_open_usd         double precision,
    market_cap_open_change      double precision,
    market_cap_open_usd_change  double precision,
    market_cap_open_percent     real,

    market_cap_high             double precision,
    market_cap_high_usd         double precision,
    market_cap_high_change      double precision,
    market_cap_high_usd_change  double precision,
    market_cap_high_percent     real,

    market_cap_low              double precision,
    market_cap_low_usd          double precision,
    market_cap_low_change       double precision,
    market_cap_low_usd_change   double precision,
    market_cap_low_percent      real,

    market_cap_close            double precision,
    market_cap_close_usd        double precision,
    market_cap_close_change     double precision,
    market_cap_close_usd_change double precision,
    market_cap_close_percent    real,

    market_cap_avg              double precision,
    market_cap_avg_usd          double precision,
    market_cap_avg_change       double precision,
    market_cap_avg_usd_change   double precision,
    market_cap_avg_percent      real,

    price_open                  double precision,
    price_open_usd              double precision,
    price_open_change           double precision,
    price_open_usd_change       double precision,
    price_open_percent          real,

    price_high                  double precision,
    price_high_usd              double precision,
    price_high_change           double precision,
    price_high_usd_change       double precision,
    price_high_percent          real,

    price_low                   double precision,
    price_low_usd               double precision,
    price_low_change            double precision,
    price_low_usd_change        double precision,
    price_low_percent           real,

    price_close                 double precision,
    price_close_usd             double precision,
    price_close_change          double precision,
    price_close_usd_change      double precision,
    price_close_percent         real,

    price_avg                   double precision,
    price_avg_usd               double precision,
    price_avg_change            double precision,
    price_avg_usd_change        double precision,
    price_avg_percent           real,

    trade                       int4,
    trade_change                double precision,
    trade_percent               real,
    trade_buy                   int4,
    trade_buy_change            double precision,
    trade_buy_percent           real,
    trade_sell                  int4,
    trade_sell_change           double precision,
    trade_sell_percent          real,

    volume                      double precision,
    volume_usd                  double precision,
    volume_change               double precision,
    volume_usd_change           double precision,
    volume_percent              real,
    volume_buy                  double precision,
    volume_buy_usd              double precision,
    volume_buy_change           double precision,
    volume_buy_usd_change       double precision,
    volume_buy_percent          real,
    volume_sell                 double precision,
    volume_sell_usd             double precision,
    volume_sell_change          double precision,
    volume_sell_usd_change      double precision,
    volume_sell_percent         real,

    updated_at                  timestamptz default (timezone('utc', now())),

    constraint fk_token_pair foreign key (token_pair_id) references solana.token_pair (id)
);


create trigger set_updated_at
    before update
    on pumpfun.summary_1d
    for each row execute function pumpfun.update_updated_at_column();