-- Copyright (c) nyanbot.com 2025.
-- This file is licensed under the AGPL-3.0-or-later.

drop trigger trigger_update_most_recent_token_balance on solana.wallet_token_balance;
drop function solana.update_wallet_token_balance_most_recent;
drop table solana.wallet_token_balance_most_recent;
drop table solana.wallet_token_balance;

drop trigger trigger_update_most_recent_sol_balance on solana.wallet_balance_sol;
drop function solana.update_wallet_balance_sol_most_recent;
drop table solana.wallet_balance_sol_most_recent;
drop table solana.wallet_balance_sol;