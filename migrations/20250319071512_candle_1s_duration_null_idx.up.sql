-- Copyright (c) nyanbot.com 2025.
-- This file is licensed under the AGPL-3.0-or-later.

create index candle_1s_1_duration_null on pumpfun.candle_1s_1 (token_pair_id, timestamp) where duration is null;
create index candle_1s_2_duration_null on pumpfun.candle_1s_2 (token_pair_id, timestamp) where duration is null;
create index candle_1s_3_duration_null on pumpfun.candle_1s_3 (token_pair_id, timestamp) where duration is null;
create index candle_1s_4_duration_null on pumpfun.candle_1s_4 (token_pair_id, timestamp) where duration is null;
create index candle_1s_5_duration_null on pumpfun.candle_1s_5 (token_pair_id, timestamp) where duration is null;
create index candle_1s_6_duration_null on pumpfun.candle_1s_6 (token_pair_id, timestamp) where duration is null;
create index candle_1s_7_duration_null on pumpfun.candle_1s_7 (token_pair_id, timestamp) where duration is null;
create index candle_1s_8_duration_null on pumpfun.candle_1s_8 (token_pair_id, timestamp) where duration is null;

create index candle_1s_1_duration_null on jupiter.candle_1s_1 (token_pair_id, timestamp) where duration is null;
create index candle_1s_2_duration_null on jupiter.candle_1s_2 (token_pair_id, timestamp) where duration is null;
create index candle_1s_3_duration_null on jupiter.candle_1s_3 (token_pair_id, timestamp) where duration is null;
create index candle_1s_4_duration_null on jupiter.candle_1s_4 (token_pair_id, timestamp) where duration is null;
create index candle_1s_5_duration_null on jupiter.candle_1s_5 (token_pair_id, timestamp) where duration is null;
create index candle_1s_6_duration_null on jupiter.candle_1s_6 (token_pair_id, timestamp) where duration is null;
create index candle_1s_7_duration_null on jupiter.candle_1s_7 (token_pair_id, timestamp) where duration is null;
create index candle_1s_8_duration_null on jupiter.candle_1s_8 (token_pair_id, timestamp) where duration is null;