-- Copyright (c) nyanbot.com 2025.
-- This file is licensed under the AGPL-3.0-or-later.

create table pumpfun.candle_progress_1s
(
    token_pair_id int8        not null,
    timestamp     timestamptz not null,
    open          real        not null,
    high          real        not null,
    low           real        not null,
    close         real        not null,
    avg           real        not null,
    created_at    timestamptz not null default (timezone('utc', now())),
    updated_at    timestamptz not null default (timezone('utc', now())),
    primary key (token_pair_id, timestamp),

    constraint fk_token_pair foreign key (token_pair_id) references solana.token_pair (id)
) partition by hash (token_pair_id);

create table pumpfun.candle_progress_1s_1 partition of pumpfun.candle_progress_1s for values with (modulus 8, remainder 0);

create table pumpfun.candle_progress_1s_2 partition of pumpfun.candle_progress_1s for values with (modulus 8, remainder 1);

create table pumpfun.candle_progress_1s_3 partition of pumpfun.candle_progress_1s for values with (modulus 8, remainder 2);

create table pumpfun.candle_progress_1s_4 partition of pumpfun.candle_progress_1s for values with (modulus 8, remainder 3);

create table pumpfun.candle_progress_1s_5 partition of pumpfun.candle_progress_1s for values with (modulus 8, remainder 4);

create table pumpfun.candle_progress_1s_6 partition of pumpfun.candle_progress_1s for values with (modulus 8, remainder 5);

create table pumpfun.candle_progress_1s_7 partition of pumpfun.candle_progress_1s for values with (modulus 8, remainder 6);

create table pumpfun.candle_progress_1s_8 partition of pumpfun.candle_progress_1s for values with (modulus 8, remainder 7);

create index candle_progress_1s_1_timestamp ON pumpfun.candle_progress_1s_1 (timestamp desc);
create index candle_progress_1s_2_timestamp ON pumpfun.candle_progress_1s_2 (timestamp desc);
create index candle_progress_1s_3_timestamp ON pumpfun.candle_progress_1s_3 (timestamp desc);
create index candle_progress_1s_4_timestamp ON pumpfun.candle_progress_1s_4 (timestamp desc);
create index candle_progress_1s_5_timestamp ON pumpfun.candle_progress_1s_5 (timestamp desc);
create index candle_progress_1s_6_timestamp ON pumpfun.candle_progress_1s_6 (timestamp desc);
create index candle_progress_1s_7_timestamp ON pumpfun.candle_progress_1s_7 (timestamp desc);
create index candle_progress_1s_8_timestamp ON pumpfun.candle_progress_1s_8 (timestamp desc);

create table pumpfun.candle_progress_1s_most_recent
(
    token_pair_id int8        not null,
    timestamp     timestamptz not null,
    open          real        not null,
    high          real        not null,
    low           real        not null,
    close         real        not null,
    avg           real        not null,
    created_at    timestamptz not null default (timezone('utc', now())),
    updated_at    timestamptz not null default (timezone('utc', now())),
    primary key (token_pair_id, timestamp),

    constraint fk_token_pair foreign key (token_pair_id) references solana.token_pair (id)
);

create
or replace function pumpfun.candle_progress_1s_update_most_recent() returns trigger as $$
begin
delete
from pumpfun.candle_progress_1s_most_recent
where token_pair_id = new.token_pair_id;

insert into pumpfun.candle_progress_1s_most_recent
values (new.*);
return new;
end;
$$
language plpgsql;

create trigger update_most_recent_candle
    after insert
    on pumpfun.candle_progress_1s
    for each row execute function pumpfun.candle_progress_1s_update_most_recent();


create table pumpfun.candle_progress_1m
(
    token_pair_id int8        not null,
    timestamp     timestamptz not null,
    open          real        not null,
    high          real        not null,
    low           real        not null,
    close         real        not null,
    avg           real        not null,
    created_at    timestamptz not null default (timezone('utc', now())),
    updated_at    timestamptz not null default (timezone('utc', now())),
    primary key (token_pair_id, timestamp),

    constraint fk_token_pair foreign key (token_pair_id) references solana.token_pair (id)
) partition by hash (token_pair_id);

create table pumpfun.candle_progress_1m_1 partition of pumpfun.candle_progress_1m for values with (modulus 8, remainder 0);

create table pumpfun.candle_progress_1m_2 partition of pumpfun.candle_progress_1m for values with (modulus 8, remainder 1);

create table pumpfun.candle_progress_1m_3 partition of pumpfun.candle_progress_1m for values with (modulus 8, remainder 2);

create table pumpfun.candle_progress_1m_4 partition of pumpfun.candle_progress_1m for values with (modulus 8, remainder 3);

create table pumpfun.candle_progress_1m_5 partition of pumpfun.candle_progress_1m for values with (modulus 8, remainder 4);

create table pumpfun.candle_progress_1m_6 partition of pumpfun.candle_progress_1m for values with (modulus 8, remainder 5);

create table pumpfun.candle_progress_1m_7 partition of pumpfun.candle_progress_1m for values with (modulus 8, remainder 6);

create table pumpfun.candle_progress_1m_8 partition of pumpfun.candle_progress_1m for values with (modulus 8, remainder 7);

create index candle_progress_1m_1_timestamp ON pumpfun.candle_progress_1m_1 (timestamp desc);
create index candle_progress_1m_2_timestamp ON pumpfun.candle_progress_1m_2 (timestamp desc);
create index candle_progress_1m_3_timestamp ON pumpfun.candle_progress_1m_3 (timestamp desc);
create index candle_progress_1m_4_timestamp ON pumpfun.candle_progress_1m_4 (timestamp desc);
create index candle_progress_1m_5_timestamp ON pumpfun.candle_progress_1m_5 (timestamp desc);
create index candle_progress_1m_6_timestamp ON pumpfun.candle_progress_1m_6 (timestamp desc);
create index candle_progress_1m_7_timestamp ON pumpfun.candle_progress_1m_7 (timestamp desc);
create index candle_progress_1m_8_timestamp ON pumpfun.candle_progress_1m_8 (timestamp desc);

create table pumpfun.candle_progress_5m
(
    token_pair_id int8        not null,
    timestamp     timestamptz not null,
    open          real        not null,
    high          real        not null,
    low           real        not null,
    close         real        not null,
    avg           real        not null,
    created_at    timestamptz not null default (timezone('utc', now())),
    updated_at    timestamptz not null default (timezone('utc', now())),
    primary key (token_pair_id, timestamp),

    constraint fk_token_pair foreign key (token_pair_id) references solana.token_pair (id)
) partition by hash (token_pair_id);

create table pumpfun.candle_progress_5m_1 partition of pumpfun.candle_progress_5m for values with (modulus 8, remainder 0);

create table pumpfun.candle_progress_5m_2 partition of pumpfun.candle_progress_5m for values with (modulus 8, remainder 1);

create table pumpfun.candle_progress_5m_3 partition of pumpfun.candle_progress_5m for values with (modulus 8, remainder 2);

create table pumpfun.candle_progress_5m_4 partition of pumpfun.candle_progress_5m for values with (modulus 8, remainder 3);

create table pumpfun.candle_progress_5m_5 partition of pumpfun.candle_progress_5m for values with (modulus 8, remainder 4);

create table pumpfun.candle_progress_5m_6 partition of pumpfun.candle_progress_5m for values with (modulus 8, remainder 5);

create table pumpfun.candle_progress_5m_7 partition of pumpfun.candle_progress_5m for values with (modulus 8, remainder 6);

create table pumpfun.candle_progress_5m_8 partition of pumpfun.candle_progress_5m for values with (modulus 8, remainder 7);

create index candle_progress_5m_1_timestamp ON pumpfun.candle_progress_5m_1 (timestamp desc);
create index candle_progress_5m_2_timestamp ON pumpfun.candle_progress_5m_2 (timestamp desc);
create index candle_progress_5m_3_timestamp ON pumpfun.candle_progress_5m_3 (timestamp desc);
create index candle_progress_5m_4_timestamp ON pumpfun.candle_progress_5m_4 (timestamp desc);
create index candle_progress_5m_5_timestamp ON pumpfun.candle_progress_5m_5 (timestamp desc);
create index candle_progress_5m_6_timestamp ON pumpfun.candle_progress_5m_6 (timestamp desc);
create index candle_progress_5m_7_timestamp ON pumpfun.candle_progress_5m_7 (timestamp desc);
create index candle_progress_5m_8_timestamp ON pumpfun.candle_progress_5m_8 (timestamp desc);

create table pumpfun.candle_progress_15m
(
    token_pair_id int8        not null,
    timestamp     timestamptz not null,
    open          real        not null,
    high          real        not null,
    low           real        not null,
    close         real        not null,
    avg           real        not null,
    created_at    timestamptz not null default (timezone('utc', now())),
    updated_at    timestamptz not null default (timezone('utc', now())),
    primary key (token_pair_id, timestamp),

    constraint fk_token_pair foreign key (token_pair_id) references solana.token_pair (id)
) partition by hash (token_pair_id);

create table pumpfun.candle_progress_15m_1 partition of pumpfun.candle_progress_15m for values with (modulus 8, remainder 0);

create table pumpfun.candle_progress_15m_2 partition of pumpfun.candle_progress_15m for values with (modulus 8, remainder 1);

create table pumpfun.candle_progress_15m_3 partition of pumpfun.candle_progress_15m for values with (modulus 8, remainder 2);

create table pumpfun.candle_progress_15m_4 partition of pumpfun.candle_progress_15m for values with (modulus 8, remainder 3);

create table pumpfun.candle_progress_15m_5 partition of pumpfun.candle_progress_15m for values with (modulus 8, remainder 4);

create table pumpfun.candle_progress_15m_6 partition of pumpfun.candle_progress_15m for values with (modulus 8, remainder 5);

create table pumpfun.candle_progress_15m_7 partition of pumpfun.candle_progress_15m for values with (modulus 8, remainder 6);

create table pumpfun.candle_progress_15m_8 partition of pumpfun.candle_progress_15m for values with (modulus 8, remainder 7);

create index candle_progress_15m_1_timestamp ON pumpfun.candle_progress_15m_1 (timestamp desc);
create index candle_progress_15m_2_timestamp ON pumpfun.candle_progress_15m_2 (timestamp desc);
create index candle_progress_15m_3_timestamp ON pumpfun.candle_progress_15m_3 (timestamp desc);
create index candle_progress_15m_4_timestamp ON pumpfun.candle_progress_15m_4 (timestamp desc);
create index candle_progress_15m_5_timestamp ON pumpfun.candle_progress_15m_5 (timestamp desc);
create index candle_progress_15m_6_timestamp ON pumpfun.candle_progress_15m_6 (timestamp desc);
create index candle_progress_15m_7_timestamp ON pumpfun.candle_progress_15m_7 (timestamp desc);
create index candle_progress_15m_8_timestamp ON pumpfun.candle_progress_15m_8 (timestamp desc);

create table pumpfun.candle_progress_1h
(
    token_pair_id int8        not null,
    timestamp     timestamptz not null,
    open          real        not null,
    high          real        not null,
    low           real        not null,
    close         real        not null,
    avg           real        not null,
    created_at    timestamptz not null default (timezone('utc', now())),
    updated_at    timestamptz not null default (timezone('utc', now())),
    primary key (token_pair_id, timestamp),

    constraint fk_token_pair foreign key (token_pair_id) references solana.token_pair (id)
) partition by hash (token_pair_id);

create table pumpfun.candle_progress_1h_1 partition of pumpfun.candle_progress_1h for values with (modulus 8, remainder 0);

create table pumpfun.candle_progress_1h_2 partition of pumpfun.candle_progress_1h for values with (modulus 8, remainder 1);

create table pumpfun.candle_progress_1h_3 partition of pumpfun.candle_progress_1h for values with (modulus 8, remainder 2);

create table pumpfun.candle_progress_1h_4 partition of pumpfun.candle_progress_1h for values with (modulus 8, remainder 3);

create table pumpfun.candle_progress_1h_5 partition of pumpfun.candle_progress_1h for values with (modulus 8, remainder 4);

create table pumpfun.candle_progress_1h_6 partition of pumpfun.candle_progress_1h for values with (modulus 8, remainder 5);

create table pumpfun.candle_progress_1h_7 partition of pumpfun.candle_progress_1h for values with (modulus 8, remainder 6);

create table pumpfun.candle_progress_1h_8 partition of pumpfun.candle_progress_1h for values with (modulus 8, remainder 7);

create index candle_progress_1h_1_timestamp ON pumpfun.candle_progress_1h_1 (timestamp desc);
create index candle_progress_1h_2_timestamp ON pumpfun.candle_progress_1h_2 (timestamp desc);
create index candle_progress_1h_3_timestamp ON pumpfun.candle_progress_1h_3 (timestamp desc);
create index candle_progress_1h_4_timestamp ON pumpfun.candle_progress_1h_4 (timestamp desc);
create index candle_progress_1h_5_timestamp ON pumpfun.candle_progress_1h_5 (timestamp desc);
create index candle_progress_1h_6_timestamp ON pumpfun.candle_progress_1h_6 (timestamp desc);
create index candle_progress_1h_7_timestamp ON pumpfun.candle_progress_1h_7 (timestamp desc);
create index candle_progress_1h_8_timestamp ON pumpfun.candle_progress_1h_8 (timestamp desc);

create table pumpfun.candle_progress_6h
(
    token_pair_id int8        not null,
    timestamp     timestamptz not null,
    open          real        not null,
    high          real        not null,
    low           real        not null,
    close         real        not null,
    avg           real        not null,
    created_at    timestamptz not null default (timezone('utc', now())),
    updated_at    timestamptz not null default (timezone('utc', now())),
    primary key (token_pair_id, timestamp),

    constraint fk_token_pair foreign key (token_pair_id) references solana.token_pair (id)
) partition by hash (token_pair_id);

create table pumpfun.candle_progress_6h_1 partition of pumpfun.candle_progress_6h for values with (modulus 8, remainder 0);

create table pumpfun.candle_progress_6h_2 partition of pumpfun.candle_progress_6h for values with (modulus 8, remainder 1);

create table pumpfun.candle_progress_6h_3 partition of pumpfun.candle_progress_6h for values with (modulus 8, remainder 2);

create table pumpfun.candle_progress_6h_4 partition of pumpfun.candle_progress_6h for values with (modulus 8, remainder 3);

create table pumpfun.candle_progress_6h_5 partition of pumpfun.candle_progress_6h for values with (modulus 8, remainder 4);

create table pumpfun.candle_progress_6h_6 partition of pumpfun.candle_progress_6h for values with (modulus 8, remainder 5);

create table pumpfun.candle_progress_6h_7 partition of pumpfun.candle_progress_6h for values with (modulus 8, remainder 6);

create table pumpfun.candle_progress_6h_8 partition of pumpfun.candle_progress_6h for values with (modulus 8, remainder 7);

create index candle_progress_6h_1_timestamp ON pumpfun.candle_progress_6h_1 (timestamp desc);
create index candle_progress_6h_2_timestamp ON pumpfun.candle_progress_6h_2 (timestamp desc);
create index candle_progress_6h_3_timestamp ON pumpfun.candle_progress_6h_3 (timestamp desc);
create index candle_progress_6h_4_timestamp ON pumpfun.candle_progress_6h_4 (timestamp desc);
create index candle_progress_6h_5_timestamp ON pumpfun.candle_progress_6h_5 (timestamp desc);
create index candle_progress_6h_6_timestamp ON pumpfun.candle_progress_6h_6 (timestamp desc);
create index candle_progress_6h_7_timestamp ON pumpfun.candle_progress_6h_7 (timestamp desc);
create index candle_progress_6h_8_timestamp ON pumpfun.candle_progress_6h_8 (timestamp desc);

create table pumpfun.candle_progress_1d
(
    token_pair_id int8        not null,
    timestamp     timestamptz not null,
    open          real        not null,
    high          real        not null,
    low           real        not null,
    close         real        not null,
    avg           real        not null,
    created_at    timestamptz not null default (timezone('utc', now())),
    updated_at    timestamptz not null default (timezone('utc', now())),
    primary key (token_pair_id, timestamp),

    constraint fk_token_pair foreign key (token_pair_id) references solana.token_pair (id)
) partition by hash (token_pair_id);

create table pumpfun.candle_progress_1d_1 partition of pumpfun.candle_progress_1d for values with (modulus 8, remainder 0);

create table pumpfun.candle_progress_1d_2 partition of pumpfun.candle_progress_1d for values with (modulus 8, remainder 1);

create table pumpfun.candle_progress_1d_3 partition of pumpfun.candle_progress_1d for values with (modulus 8, remainder 2);

create table pumpfun.candle_progress_1d_4 partition of pumpfun.candle_progress_1d for values with (modulus 8, remainder 3);

create table pumpfun.candle_progress_1d_5 partition of pumpfun.candle_progress_1d for values with (modulus 8, remainder 4);

create table pumpfun.candle_progress_1d_6 partition of pumpfun.candle_progress_1d for values with (modulus 8, remainder 5);

create table pumpfun.candle_progress_1d_7 partition of pumpfun.candle_progress_1d for values with (modulus 8, remainder 6);

create table pumpfun.candle_progress_1d_8 partition of pumpfun.candle_progress_1d for values with (modulus 8, remainder 7);

create index candle_progress_1d_1_timestamp ON pumpfun.candle_progress_1d_1 (timestamp desc);
create index candle_progress_1d_2_timestamp ON pumpfun.candle_progress_1d_2 (timestamp desc);
create index candle_progress_1d_3_timestamp ON pumpfun.candle_progress_1d_3 (timestamp desc);
create index candle_progress_1d_4_timestamp ON pumpfun.candle_progress_1d_4 (timestamp desc);
create index candle_progress_1d_5_timestamp ON pumpfun.candle_progress_1d_5 (timestamp desc);
create index candle_progress_1d_6_timestamp ON pumpfun.candle_progress_1d_6 (timestamp desc);
create index candle_progress_1d_7_timestamp ON pumpfun.candle_progress_1d_7 (timestamp desc);
create index candle_progress_1d_8_timestamp ON pumpfun.candle_progress_1d_8 (timestamp desc);