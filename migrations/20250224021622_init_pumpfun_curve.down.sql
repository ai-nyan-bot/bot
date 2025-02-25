-- Copyright (c) nyanbot.com 2025.
-- This file is licensed under the AGPL-3.0-or-later.
drop table pumpfun.curve_log;

drop trigger curve_log on pumpfun.curve;
drop trigger set_updated_at on pumpfun.curve;
drop function pumpfun.curve_log_trigger_fn;
drop table pumpfun.curve;