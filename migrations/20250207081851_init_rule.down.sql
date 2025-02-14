-- Copyright (c) nyanbot.com 2025.
-- This file is licensed under the AGPL-3.0-or-later.

drop trigger set_updated_at on solana.rule;
drop trigger trigger_increment_version on solana.rule;
drop function solana.increment_rule_version;
drop table solana.rule cascade;