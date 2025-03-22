-- Copyright (c) nyanbot.com 2025.
-- This file is licensed under the AGPL-3.0-or-later.

alter table solana.wallet set schema nyanbot;

alter table nyanbot.wallet rename column public_key to solana_public_key;

alter table nyanbot.wallet rename column private_key to solana_private_key;

alter index nyanbot.unique_wallet_public_key rename to unique_solana_solana_public_key;

alter index nyanbot.unique_wallet_private_key rename to unique_solana_solana_private_key;
