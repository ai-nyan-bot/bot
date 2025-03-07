-- Copyright (c) nyanbot.com 2025.
-- This file is licensed under the AGPL-3.0-or-later.

create index swap_1_timestamp ON pumpfun.swap_1 (timestamp desc);
create index swap_2_timestamp ON pumpfun.swap_2 (timestamp desc);
create index swap_3_timestamp ON pumpfun.swap_3 (timestamp desc);
create index swap_4_timestamp ON pumpfun.swap_4 (timestamp desc);
create index swap_5_timestamp ON pumpfun.swap_5 (timestamp desc);
create index swap_6_timestamp ON pumpfun.swap_6 (timestamp desc);
create index swap_7_timestamp ON pumpfun.swap_7 (timestamp desc);
create index swap_8_timestamp ON pumpfun.swap_8 (timestamp desc);

create index swap_1_timestamp ON jupiter.swap_1 (timestamp desc);
create index swap_2_timestamp ON jupiter.swap_2 (timestamp desc);
create index swap_3_timestamp ON jupiter.swap_3 (timestamp desc);
create index swap_4_timestamp ON jupiter.swap_4 (timestamp desc);
create index swap_5_timestamp ON jupiter.swap_5 (timestamp desc);
create index swap_6_timestamp ON jupiter.swap_6 (timestamp desc);
create index swap_7_timestamp ON jupiter.swap_7 (timestamp desc);
create index swap_8_timestamp ON jupiter.swap_8 (timestamp desc);