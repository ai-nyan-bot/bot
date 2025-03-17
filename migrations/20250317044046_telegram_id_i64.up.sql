-- Copyright (c) nyanbot.com 2025.
-- This file is licensed under the AGPL-3.0-or-later.

alter table nyanbot."user"
alter column telegram_id type int8 using telegram_id::int8;

