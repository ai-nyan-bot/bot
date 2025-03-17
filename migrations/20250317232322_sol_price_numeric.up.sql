-- Copyright (c) nyanbot.com 2025.
-- This file is licensed under the AGPL-3.0-or-later.

alter table solana.sol_price_1m alter column usd type numeric(36, 12) using usd::numeric(36, 12);
alter table solana.sol_price_5m alter column usd type numeric(36, 12) using usd::numeric(36, 12);
alter table solana.sol_price_15m alter column usd type numeric(36, 12) using usd::numeric(36, 12);
alter table solana.sol_price_1h alter column usd type numeric(36, 12) using usd::numeric(36, 12);
alter table solana.sol_price_6h alter column usd type numeric(36, 12) using usd::numeric(36, 12);
alter table solana.sol_price_1d alter column usd type numeric(36, 12) using usd::numeric(36, 12);

