-- Copyright (c) nyanbot.com 2025.
-- This file is licensed under the AGPL-3.0-or-later.

drop trigger set_updated_at on nyanbot.auth;
drop table nyanbot.auth cascade;

drop trigger set_updated_at on nyanbot.user;
drop table nyanbot.user cascade;
