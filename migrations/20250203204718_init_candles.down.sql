-- Copyright (c) nyanbot.com 2025.
-- This file is licensed under the AGPL-3.0-or-later.

-- This file includes portions of code from https://github.com/blockworks-foundation/traffic (AGPL 3.0).
-- Original MIT License Copyright (c) blockworks-foundation 2024.

drop table pumpfun.candle_1s_most_recent cascade;
drop function pumpfun.candle_1s_update_most_recent cascade;
drop table pumpfun.candle_1s cascade;

drop table pumpfun.candle_1m_most_recent cascade;
drop function pumpfun.candle_1m_update_most_recent cascade;
drop table pumpfun.candle_1m cascade;

drop table pumpfun.candle_5m_most_recent cascade;
drop function pumpfun.candle_5m_update_most_recent cascade;
drop table pumpfun.candle_5m cascade;

drop table pumpfun.candle_15m_most_recent cascade;
drop function pumpfun.candle_15m_update_most_recent cascade;
drop table pumpfun.candle_15m cascade;

drop table pumpfun.candle_1h_most_recent cascade;
drop function pumpfun.candle_1h_update_most_recent cascade;
drop table pumpfun.candle_1h cascade;

drop table pumpfun.candle_4h_most_recent cascade;
drop function pumpfun.candle_4h_update_most_recent cascade;
drop table pumpfun.candle_4h cascade;

drop table pumpfun.candle_1d_most_recent cascade;
drop function pumpfun.candle_1d_update_most_recent cascade;
drop table pumpfun.candle_1d cascade;