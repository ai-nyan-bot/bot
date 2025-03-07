-- Copyright (c) nyanbot.com 2025.
-- This file is licensed under the AGPL-3.0-or-later.

create table pumpfun.twap_1m
(
    token_pair_id int4                     not null,
    timestamp     timestamp with time zone not null,
    twap          numeric(36, 12)          not null,
    primary key (token_pair_id, timestamp),
    constraint fk_token_pair foreign key (token_pair_id) references solana.token_pair (id)
) partition by hash (token_pair_id);

create table pumpfun.twap_1m_1 partition of pumpfun.twap_1m for values with (modulus 8, remainder 0);

create table pumpfun.twap_1m_2 partition of pumpfun.twap_1m for values with (modulus 8, remainder 1);

create table pumpfun.twap_1m_3 partition of pumpfun.twap_1m for values with (modulus 8, remainder 2);

create table pumpfun.twap_1m_4 partition of pumpfun.twap_1m for values with (modulus 8, remainder 3);

create table pumpfun.twap_1m_5 partition of pumpfun.twap_1m for values with (modulus 8, remainder 4);

create table pumpfun.twap_1m_6 partition of pumpfun.twap_1m for values with (modulus 8, remainder 5);

create table pumpfun.twap_1m_7 partition of pumpfun.twap_1m for values with (modulus 8, remainder 6);

create table pumpfun.twap_1m_8 partition of pumpfun.twap_1m for values with (modulus 8, remainder 7);

create index twap_1m_1_timestamp ON pumpfun.twap_1m_1 (timestamp desc);
create index twap_1m_2_timestamp ON pumpfun.twap_1m_2 (timestamp desc);
create index twap_1m_3_timestamp ON pumpfun.twap_1m_3 (timestamp desc);
create index twap_1m_4_timestamp ON pumpfun.twap_1m_4 (timestamp desc);
create index twap_1m_5_timestamp ON pumpfun.twap_1m_5 (timestamp desc);
create index twap_1m_6_timestamp ON pumpfun.twap_1m_6 (timestamp desc);
create index twap_1m_7_timestamp ON pumpfun.twap_1m_7 (timestamp desc);
create index twap_1m_8_timestamp ON pumpfun.twap_1m_8 (timestamp desc);


create table pumpfun.twap_5m
(
    token_pair_id int4                     not null,
    timestamp     timestamp with time zone not null,
    twap          numeric(36, 12)          not null,
    primary key (token_pair_id, timestamp),
    constraint fk_token_pair foreign key (token_pair_id) references solana.token_pair (id)
) partition by hash (token_pair_id);

create table pumpfun.twap_5m_1 partition of pumpfun.twap_5m for values with (modulus 8, remainder 0);

create table pumpfun.twap_5m_2 partition of pumpfun.twap_5m for values with (modulus 8, remainder 1);

create table pumpfun.twap_5m_3 partition of pumpfun.twap_5m for values with (modulus 8, remainder 2);

create table pumpfun.twap_5m_4 partition of pumpfun.twap_5m for values with (modulus 8, remainder 3);

create table pumpfun.twap_5m_5 partition of pumpfun.twap_5m for values with (modulus 8, remainder 4);

create table pumpfun.twap_5m_6 partition of pumpfun.twap_5m for values with (modulus 8, remainder 5);

create table pumpfun.twap_5m_7 partition of pumpfun.twap_5m for values with (modulus 8, remainder 6);

create table pumpfun.twap_5m_8 partition of pumpfun.twap_5m for values with (modulus 8, remainder 7);

create index twap_5m_1_timestamp ON pumpfun.twap_5m_1 (timestamp desc);
create index twap_5m_2_timestamp ON pumpfun.twap_5m_2 (timestamp desc);
create index twap_5m_3_timestamp ON pumpfun.twap_5m_3 (timestamp desc);
create index twap_5m_4_timestamp ON pumpfun.twap_5m_4 (timestamp desc);
create index twap_5m_5_timestamp ON pumpfun.twap_5m_5 (timestamp desc);
create index twap_5m_6_timestamp ON pumpfun.twap_5m_6 (timestamp desc);
create index twap_5m_7_timestamp ON pumpfun.twap_5m_7 (timestamp desc);
create index twap_5m_8_timestamp ON pumpfun.twap_5m_8 (timestamp desc);


create table pumpfun.twap_15m
(
    token_pair_id int4                     not null,
    timestamp     timestamp with time zone not null,
    twap          numeric(36, 12)          not null,
    primary key (token_pair_id, timestamp),
    constraint fk_token_pair foreign key (token_pair_id) references solana.token_pair (id)
) partition by hash (token_pair_id);

create table pumpfun.twap_15m_1 partition of pumpfun.twap_15m for values with (modulus 8, remainder 0);

create table pumpfun.twap_15m_2 partition of pumpfun.twap_15m for values with (modulus 8, remainder 1);

create table pumpfun.twap_15m_3 partition of pumpfun.twap_15m for values with (modulus 8, remainder 2);

create table pumpfun.twap_15m_4 partition of pumpfun.twap_15m for values with (modulus 8, remainder 3);

create table pumpfun.twap_15m_5 partition of pumpfun.twap_15m for values with (modulus 8, remainder 4);

create table pumpfun.twap_15m_6 partition of pumpfun.twap_15m for values with (modulus 8, remainder 5);

create table pumpfun.twap_15m_7 partition of pumpfun.twap_15m for values with (modulus 8, remainder 6);

create table pumpfun.twap_15m_8 partition of pumpfun.twap_15m for values with (modulus 8, remainder 7);

create index twap_15m_1_timestamp ON pumpfun.twap_15m_1 (timestamp desc);
create index twap_15m_2_timestamp ON pumpfun.twap_15m_2 (timestamp desc);
create index twap_15m_3_timestamp ON pumpfun.twap_15m_3 (timestamp desc);
create index twap_15m_4_timestamp ON pumpfun.twap_15m_4 (timestamp desc);
create index twap_15m_5_timestamp ON pumpfun.twap_15m_5 (timestamp desc);
create index twap_15m_6_timestamp ON pumpfun.twap_15m_6 (timestamp desc);
create index twap_15m_7_timestamp ON pumpfun.twap_15m_7 (timestamp desc);
create index twap_15m_8_timestamp ON pumpfun.twap_15m_8 (timestamp desc);


create table pumpfun.twap_1h
(
    token_pair_id int4                     not null,
    timestamp     timestamp with time zone not null,
    twap          numeric(36, 12)          not null,
    primary key (token_pair_id, timestamp),
    constraint fk_token_pair foreign key (token_pair_id) references solana.token_pair (id)
) partition by hash (token_pair_id);


create table pumpfun.twap_1h_1 partition of pumpfun.twap_1h for values with (modulus 8, remainder 0);

create table pumpfun.twap_1h_2 partition of pumpfun.twap_1h for values with (modulus 8, remainder 1);

create table pumpfun.twap_1h_3 partition of pumpfun.twap_1h for values with (modulus 8, remainder 2);

create table pumpfun.twap_1h_4 partition of pumpfun.twap_1h for values with (modulus 8, remainder 3);

create table pumpfun.twap_1h_5 partition of pumpfun.twap_1h for values with (modulus 8, remainder 4);

create table pumpfun.twap_1h_6 partition of pumpfun.twap_1h for values with (modulus 8, remainder 5);

create table pumpfun.twap_1h_7 partition of pumpfun.twap_1h for values with (modulus 8, remainder 6);

create table pumpfun.twap_1h_8 partition of pumpfun.twap_1h for values with (modulus 8, remainder 7);

create index twap_1h_1_timestamp ON pumpfun.twap_1h_1 (timestamp desc);
create index twap_1h_2_timestamp ON pumpfun.twap_1h_2 (timestamp desc);
create index twap_1h_3_timestamp ON pumpfun.twap_1h_3 (timestamp desc);
create index twap_1h_4_timestamp ON pumpfun.twap_1h_4 (timestamp desc);
create index twap_1h_5_timestamp ON pumpfun.twap_1h_5 (timestamp desc);
create index twap_1h_6_timestamp ON pumpfun.twap_1h_6 (timestamp desc);
create index twap_1h_7_timestamp ON pumpfun.twap_1h_7 (timestamp desc);
create index twap_1h_8_timestamp ON pumpfun.twap_1h_8 (timestamp desc);


create table pumpfun.twap_6h
(
    token_pair_id int4                     not null,
    timestamp     timestamp with time zone not null,
    twap          numeric(36, 12)          not null,
    primary key (token_pair_id, timestamp),
    constraint fk_token_pair foreign key (token_pair_id) references solana.token_pair (id)
) partition by hash (token_pair_id);

create table pumpfun.twap_6h_1 partition of pumpfun.twap_6h for values with (modulus 8, remainder 0);

create table pumpfun.twap_6h_2 partition of pumpfun.twap_6h for values with (modulus 8, remainder 1);

create table pumpfun.twap_6h_3 partition of pumpfun.twap_6h for values with (modulus 8, remainder 2);

create table pumpfun.twap_6h_4 partition of pumpfun.twap_6h for values with (modulus 8, remainder 3);

create table pumpfun.twap_6h_5 partition of pumpfun.twap_6h for values with (modulus 8, remainder 4);

create table pumpfun.twap_6h_6 partition of pumpfun.twap_6h for values with (modulus 8, remainder 5);

create table pumpfun.twap_6h_7 partition of pumpfun.twap_6h for values with (modulus 8, remainder 6);

create table pumpfun.twap_6h_8 partition of pumpfun.twap_6h for values with (modulus 8, remainder 7);

create index twap_6h_1_timestamp ON pumpfun.twap_6h_1 (timestamp desc);
create index twap_6h_2_timestamp ON pumpfun.twap_6h_2 (timestamp desc);
create index twap_6h_3_timestamp ON pumpfun.twap_6h_3 (timestamp desc);
create index twap_6h_4_timestamp ON pumpfun.twap_6h_4 (timestamp desc);
create index twap_6h_5_timestamp ON pumpfun.twap_6h_5 (timestamp desc);
create index twap_6h_6_timestamp ON pumpfun.twap_6h_6 (timestamp desc);
create index twap_6h_7_timestamp ON pumpfun.twap_6h_7 (timestamp desc);
create index twap_6h_8_timestamp ON pumpfun.twap_6h_8 (timestamp desc);


create table pumpfun.twap_1d
(
    token_pair_id int4                     not null,
    timestamp     timestamp with time zone not null,
    twap          numeric(36, 12)          not null,
    primary key (token_pair_id, timestamp),
    constraint fk_token_pair foreign key (token_pair_id) references solana.token_pair (id)
) partition by hash (token_pair_id);

create table pumpfun.twap_1d_1 partition of pumpfun.twap_1d for values with (modulus 8, remainder 0);

create table pumpfun.twap_1d_2 partition of pumpfun.twap_1d for values with (modulus 8, remainder 1);

create table pumpfun.twap_1d_3 partition of pumpfun.twap_1d for values with (modulus 8, remainder 2);

create table pumpfun.twap_1d_4 partition of pumpfun.twap_1d for values with (modulus 8, remainder 3);

create table pumpfun.twap_1d_5 partition of pumpfun.twap_1d for values with (modulus 8, remainder 4);

create table pumpfun.twap_1d_6 partition of pumpfun.twap_1d for values with (modulus 8, remainder 5);

create table pumpfun.twap_1d_7 partition of pumpfun.twap_1d for values with (modulus 8, remainder 6);

create table pumpfun.twap_1d_8 partition of pumpfun.twap_1d for values with (modulus 8, remainder 7);

create index twap_1d_1_timestamp ON pumpfun.twap_1d_1 (timestamp desc);
create index twap_1d_2_timestamp ON pumpfun.twap_1d_2 (timestamp desc);
create index twap_1d_3_timestamp ON pumpfun.twap_1d_3 (timestamp desc);
create index twap_1d_4_timestamp ON pumpfun.twap_1d_4 (timestamp desc);
create index twap_1d_5_timestamp ON pumpfun.twap_1d_5 (timestamp desc);
create index twap_1d_6_timestamp ON pumpfun.twap_1d_6 (timestamp desc);
create index twap_1d_7_timestamp ON pumpfun.twap_1d_7 (timestamp desc);
create index twap_1d_8_timestamp ON pumpfun.twap_1d_8 (timestamp desc);
