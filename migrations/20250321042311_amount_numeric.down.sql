-- Copyright (c) nyanbot.com 2025.
-- This file is licensed under the AGPL-3.0-or-later.

alter table pumpfun.swap_most_recent alter column virtual_base_reserves type int8 using virtual_base_reserves::int8;
alter table pumpfun.swap_most_recent alter column virtual_quote_reserves type int8 using virtual_quote_reserves::int8;

alter table pumpfun.swap alter column virtual_base_reserves type int8 using virtual_base_reserves::int8;
alter table pumpfun.swap alter column virtual_quote_reserves type int8 using virtual_quote_reserves::int8;

alter table pumpfun.micro_swap alter column virtual_base_reserves type int8 using virtual_base_reserves::int8;
alter table pumpfun.micro_swap alter column virtual_quote_reserves type int8 using virtual_quote_reserves::int8;

alter table pumpfun.current alter column virtual_base_reserves type int8 using virtual_base_reserves::int8;
alter table pumpfun.current alter column virtual_quote_reserves type int8 using virtual_quote_reserves::int8;
