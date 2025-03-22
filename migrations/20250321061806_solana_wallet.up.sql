-- Copyright (c) nyanbot.com 2025.
-- This file is licensed under the AGPL-3.0-or-later.

alter table nyanbot.wallet set schema solana;

alter table solana.wallet rename column solana_public_key to public_key;

alter table solana.wallet rename column solana_private_key to private_key;

alter index solana.unique_solana_solana_public_key rename to unique_wallet_public_key;

alter index solana.unique_solana_solana_private_key rename to unique_wallet_private_key;
