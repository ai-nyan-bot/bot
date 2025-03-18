-- Copyright (c) nyanbot.com 2025.
-- This file is licensed under the AGPL-3.0-or-later.

drop table jupiter.micro_swap;
drop table pumpfun.micro_swap;

alter table pumpfun.swap add constraint unique_signature unique (token_pair_id, signature);
alter table jupiter.swap add constraint unique_signature unique (token_pair_id, signature);