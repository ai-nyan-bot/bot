-- Copyright (c) nyanbot.com 2025.
-- This file is licensed under the AGPL-3.0-or-later.

drop trigger set_updated_at on pumpfun.summary_1m;
drop table pumpfun.summary_1m cascade;

drop trigger set_updated_at on pumpfun.summary_5m;
drop table pumpfun.summary_5m cascade;

drop trigger set_updated_at on pumpfun.summary_15m;
drop table pumpfun.summary_15m cascade;

drop trigger set_updated_at on pumpfun.summary_1h;
drop table pumpfun.summary_1h cascade;

drop trigger set_updated_at on pumpfun.summary_6h;
drop table pumpfun.summary_6h cascade;

drop trigger set_updated_at on pumpfun.summary_1d;
drop table pumpfun.summary_1d cascade;