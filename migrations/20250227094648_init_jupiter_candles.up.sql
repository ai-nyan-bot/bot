-- Copyright (c) nyanbot.com 2025.
-- This file is licensed under the AGPL-3.0-or-later.

create table jupiter.candle_1s
(
    token_pair_id     int4             not null,
    timestamp         timestamptz      not null,
    open              numeric(36, 12) not null,
    high              numeric(36, 12) not null,
    low               numeric(36, 12) not null,
    close             numeric(36, 12) not null,
    avg               numeric(36, 12) not null,
    amount_base_buy   numeric(36, 12) not null,
    amount_quote_buy  numeric(36, 12) not null,
    amount_base_sell  numeric(36, 12) not null,
    amount_quote_sell numeric(36, 12) not null,
    trade_buy         int4             not null,
    trade_sell        int4             not null,
    volume_buy        numeric(36, 12) not null,
    volume_sell       numeric(36, 12) not null,
    duration          int4,
    primary key (token_pair_id, timestamp),

    constraint fk_token_pair foreign key (token_pair_id) references solana.token_pair (id)
) partition by hash (token_pair_id);

create table jupiter.candle_1s_1 partition of jupiter.candle_1s for values with (modulus 8, remainder 0);

create table jupiter.candle_1s_2 partition of jupiter.candle_1s for values with (modulus 8, remainder 1);

create table jupiter.candle_1s_3 partition of jupiter.candle_1s for values with (modulus 8, remainder 2);

create table jupiter.candle_1s_4 partition of jupiter.candle_1s for values with (modulus 8, remainder 3);

create table jupiter.candle_1s_5 partition of jupiter.candle_1s for values with (modulus 8, remainder 4);

create table jupiter.candle_1s_6 partition of jupiter.candle_1s for values with (modulus 8, remainder 5);

create table jupiter.candle_1s_7 partition of jupiter.candle_1s for values with (modulus 8, remainder 6);

create table jupiter.candle_1s_8 partition of jupiter.candle_1s for values with (modulus 8, remainder 7);

create index candle_1s_timestamp ON jupiter.candle_1s (timestamp desc);

create table jupiter.candle_1s_most_recent
(
    token_pair_id     int4             not null,
    timestamp         timestamptz      not null,
    open              numeric(36, 12) not null,
    high              numeric(36, 12) not null,
    low               numeric(36, 12) not null,
    close             numeric(36, 12) not null,
    avg               numeric(36, 12) not null,
    amount_base_buy   numeric(36, 12) not null,
    amount_quote_buy  numeric(36, 12) not null,
    amount_base_sell  numeric(36, 12) not null,
    amount_quote_sell numeric(36, 12) not null,
    trade_buy         int4             not null,
    trade_sell        int4             not null,
    volume_buy        numeric(36, 12) not null,
    volume_sell       numeric(36, 12) not null,
    duration          int4,
    primary key (token_pair_id, timestamp),

    constraint fk_token_pair foreign key (token_pair_id) references solana.token_pair (id)
);

create
or replace function jupiter.candle_1s_update_most_recent() returns trigger as $$
begin
delete
from jupiter.candle_1s_most_recent
where token_pair_id = new.token_pair_id;

insert into jupiter.candle_1s_most_recent
values (new.*);
return new;
end;
$$
language plpgsql;

create trigger update_most_recent_candle
    after insert
    on jupiter.candle_1s
    for each row execute function jupiter.candle_1s_update_most_recent();


create table jupiter.candle_1m
(
    token_pair_id     int4             not null,
    timestamp         timestamptz      not null,
    open              numeric(36, 12) not null,
    high              numeric(36, 12) not null,
    low               numeric(36, 12) not null,
    close             numeric(36, 12) not null,
    avg               numeric(36, 12) not null,
    amount_base_buy   numeric(36, 12) not null,
    amount_quote_buy  numeric(36, 12) not null,
    amount_base_sell  numeric(36, 12) not null,
    amount_quote_sell numeric(36, 12) not null,
    trade_buy         int4             not null,
    trade_sell        int4             not null,
    volume_buy        numeric(36, 12) not null,
    volume_sell       numeric(36, 12) not null,
    primary key (token_pair_id, timestamp),

    constraint fk_token_pair foreign key (token_pair_id) references solana.token_pair (id)
) partition by hash (token_pair_id);

create table jupiter.candle_1m_1 partition of jupiter.candle_1m for values with (modulus 8, remainder 0);

create table jupiter.candle_1m_2 partition of jupiter.candle_1m for values with (modulus 8, remainder 1);

create table jupiter.candle_1m_3 partition of jupiter.candle_1m for values with (modulus 8, remainder 2);

create table jupiter.candle_1m_4 partition of jupiter.candle_1m for values with (modulus 8, remainder 3);

create table jupiter.candle_1m_5 partition of jupiter.candle_1m for values with (modulus 8, remainder 4);

create table jupiter.candle_1m_6 partition of jupiter.candle_1m for values with (modulus 8, remainder 5);

create table jupiter.candle_1m_7 partition of jupiter.candle_1m for values with (modulus 8, remainder 6);

create table jupiter.candle_1m_8 partition of jupiter.candle_1m for values with (modulus 8, remainder 7);

create index candle_1m_timestamp ON jupiter.candle_1m (timestamp desc);

create table jupiter.candle_1m_most_recent
(
    token_pair_id     int4             not null,
    timestamp         timestamptz      not null,
    open              numeric(36, 12) not null,
    high              numeric(36, 12) not null,
    low               numeric(36, 12) not null,
    close             numeric(36, 12) not null,
    avg               numeric(36, 12) not null,
    amount_base_buy   numeric(36, 12) not null,
    amount_quote_buy  numeric(36, 12) not null,
    amount_base_sell  numeric(36, 12) not null,
    amount_quote_sell numeric(36, 12) not null,
    trade_buy         int4             not null,
    trade_sell        int4             not null,
    volume_buy        numeric(36, 12) not null,
    volume_sell       numeric(36, 12) not null,
    primary key (token_pair_id, timestamp),

    constraint fk_token_pair foreign key (token_pair_id) references solana.token_pair (id)
);

create
or replace function jupiter.candle_1m_update_most_recent() returns trigger as $$
begin
delete
from jupiter.candle_1m_most_recent
where token_pair_id = new.token_pair_id;

insert into jupiter.candle_1m_most_recent
values (new.*);
return new;
end;
$$
language plpgsql;

create trigger update_most_recent_candle
    after insert
    on jupiter.candle_1m
    for each row execute function jupiter.candle_1m_update_most_recent();

create table jupiter.candle_5m
(
    token_pair_id     int4             not null,
    timestamp         timestamptz      not null,
    open              numeric(36, 12) not null,
    high              numeric(36, 12) not null,
    low               numeric(36, 12) not null,
    close             numeric(36, 12) not null,
    avg               numeric(36, 12) not null,
    amount_base_buy   numeric(36, 12) not null,
    amount_quote_buy  numeric(36, 12) not null,
    amount_base_sell  numeric(36, 12) not null,
    amount_quote_sell numeric(36, 12) not null,
    trade_buy         int4             not null,
    trade_sell        int4             not null,
    volume_buy        numeric(36, 12) not null,
    volume_sell       numeric(36, 12) not null,
    primary key (token_pair_id, timestamp),

    constraint fk_token_pair foreign key (token_pair_id) references solana.token_pair (id)
) partition by hash (token_pair_id);

create table jupiter.candle_5m_1 partition of jupiter.candle_5m for values with (modulus 8, remainder 0);

create table jupiter.candle_5m_2 partition of jupiter.candle_5m for values with (modulus 8, remainder 1);

create table jupiter.candle_5m_3 partition of jupiter.candle_5m for values with (modulus 8, remainder 2);

create table jupiter.candle_5m_4 partition of jupiter.candle_5m for values with (modulus 8, remainder 3);

create table jupiter.candle_5m_5 partition of jupiter.candle_5m for values with (modulus 8, remainder 4);

create table jupiter.candle_5m_6 partition of jupiter.candle_5m for values with (modulus 8, remainder 5);

create table jupiter.candle_5m_7 partition of jupiter.candle_5m for values with (modulus 8, remainder 6);

create table jupiter.candle_5m_8 partition of jupiter.candle_5m for values with (modulus 8, remainder 7);

create index candle_5m_timestamp ON jupiter.candle_5m (timestamp desc);

create table jupiter.candle_5m_most_recent
(
    token_pair_id     int4             not null,
    timestamp         timestamptz      not null,
    open              numeric(36, 12) not null,
    high              numeric(36, 12) not null,
    low               numeric(36, 12) not null,
    close             numeric(36, 12) not null,
    avg               numeric(36, 12) not null,
    amount_base_buy   numeric(36, 12) not null,
    amount_quote_buy  numeric(36, 12) not null,
    amount_base_sell  numeric(36, 12) not null,
    amount_quote_sell numeric(36, 12) not null,
    trade_buy         int4             not null,
    trade_sell        int4             not null,
    volume_buy        numeric(36, 12) not null,
    volume_sell       numeric(36, 12) not null,
    primary key (token_pair_id, timestamp),

    constraint fk_token_pair foreign key (token_pair_id) references solana.token_pair (id)
);

create
or replace function jupiter.candle_5m_update_most_recent() returns trigger as $$
begin
delete
from jupiter.candle_5m_most_recent
where token_pair_id = new.token_pair_id;

insert into jupiter.candle_5m_most_recent
values (new.*);
return new;
end;
$$
language plpgsql;

create trigger update_most_recent_candle
    after insert
    on jupiter.candle_5m
    for each row execute function jupiter.candle_5m_update_most_recent();

create table jupiter.candle_15m
(
    token_pair_id     int4             not null,
    timestamp         timestamptz      not null,
    open              numeric(36, 12) not null,
    high              numeric(36, 12) not null,
    low               numeric(36, 12) not null,
    close             numeric(36, 12) not null,
    avg               numeric(36, 12) not null,
    amount_base_buy   numeric(36, 12) not null,
    amount_quote_buy  numeric(36, 12) not null,
    amount_base_sell  numeric(36, 12) not null,
    amount_quote_sell numeric(36, 12) not null,
    trade_buy         int4             not null,
    trade_sell        int4             not null,
    volume_buy        numeric(36, 12) not null,
    volume_sell       numeric(36, 12) not null,
    primary key (token_pair_id, timestamp),

    constraint fk_token_pair foreign key (token_pair_id) references solana.token_pair (id)
) partition by hash (token_pair_id);

create table jupiter.candle_15m_1 partition of jupiter.candle_15m for values with (modulus 8, remainder 0);

create table jupiter.candle_15m_2 partition of jupiter.candle_15m for values with (modulus 8, remainder 1);

create table jupiter.candle_15m_3 partition of jupiter.candle_15m for values with (modulus 8, remainder 2);

create table jupiter.candle_15m_4 partition of jupiter.candle_15m for values with (modulus 8, remainder 3);

create table jupiter.candle_15m_5 partition of jupiter.candle_15m for values with (modulus 8, remainder 4);

create table jupiter.candle_15m_6 partition of jupiter.candle_15m for values with (modulus 8, remainder 5);

create table jupiter.candle_15m_7 partition of jupiter.candle_15m for values with (modulus 8, remainder 6);

create table jupiter.candle_15m_8 partition of jupiter.candle_15m for values with (modulus 8, remainder 7);

create index candle_15m_timestamp ON jupiter.candle_15m (timestamp desc);

create table jupiter.candle_15m_most_recent
(
    token_pair_id     int4             not null,
    timestamp         timestamptz      not null,
    open              numeric(36, 12) not null,
    high              numeric(36, 12) not null,
    low               numeric(36, 12) not null,
    close             numeric(36, 12) not null,
    avg               numeric(36, 12) not null,
    amount_base_buy   numeric(36, 12) not null,
    amount_quote_buy  numeric(36, 12) not null,
    amount_base_sell  numeric(36, 12) not null,
    amount_quote_sell numeric(36, 12) not null,
    trade_buy         int4             not null,
    trade_sell        int4             not null,
    volume_buy        numeric(36, 12) not null,
    volume_sell       numeric(36, 12) not null,
    primary key (token_pair_id, timestamp),

    constraint fk_token_pair foreign key (token_pair_id) references solana.token_pair (id)
);

create
or replace function jupiter.candle_15m_update_most_recent() returns trigger as $$
begin
delete
from jupiter.candle_15m_most_recent
where token_pair_id = new.token_pair_id;

insert into jupiter.candle_15m_most_recent
values (new.*);
return new;
end;
$$
language plpgsql;

create trigger update_most_recent_candle
    after insert
    on jupiter.candle_15m
    for each row execute function jupiter.candle_15m_update_most_recent();

create table jupiter.candle_1h
(
    token_pair_id     int4             not null,
    timestamp         timestamptz      not null,
    open              numeric(36, 12) not null,
    high              numeric(36, 12) not null,
    low               numeric(36, 12) not null,
    close             numeric(36, 12) not null,
    avg               numeric(36, 12) not null,
    amount_base_buy   numeric(36, 12) not null,
    amount_quote_buy  numeric(36, 12) not null,
    amount_base_sell  numeric(36, 12) not null,
    amount_quote_sell numeric(36, 12) not null,
    trade_buy         int4             not null,
    trade_sell        int4             not null,
    volume_buy        numeric(36, 12) not null,
    volume_sell       numeric(36, 12) not null,
    primary key (token_pair_id, timestamp),

    constraint fk_token_pair foreign key (token_pair_id) references solana.token_pair (id)
) partition by hash (token_pair_id);

create table jupiter.candle_1h_1 partition of jupiter.candle_1h for values with (modulus 8, remainder 0);

create table jupiter.candle_1h_2 partition of jupiter.candle_1h for values with (modulus 8, remainder 1);

create table jupiter.candle_1h_3 partition of jupiter.candle_1h for values with (modulus 8, remainder 2);

create table jupiter.candle_1h_4 partition of jupiter.candle_1h for values with (modulus 8, remainder 3);

create table jupiter.candle_1h_5 partition of jupiter.candle_1h for values with (modulus 8, remainder 4);

create table jupiter.candle_1h_6 partition of jupiter.candle_1h for values with (modulus 8, remainder 5);

create table jupiter.candle_1h_7 partition of jupiter.candle_1h for values with (modulus 8, remainder 6);

create table jupiter.candle_1h_8 partition of jupiter.candle_1h for values with (modulus 8, remainder 7);

create index candle_1h_timestamp ON jupiter.candle_1h (timestamp desc);

create table jupiter.candle_1h_most_recent
(
    token_pair_id     int4             not null,
    timestamp         timestamptz      not null,
    open              numeric(36, 12) not null,
    high              numeric(36, 12) not null,
    low               numeric(36, 12) not null,
    close             numeric(36, 12) not null,
    avg               numeric(36, 12) not null,
    amount_base_buy   numeric(36, 12) not null,
    amount_quote_buy  numeric(36, 12) not null,
    amount_base_sell  numeric(36, 12) not null,
    amount_quote_sell numeric(36, 12) not null,
    trade_buy         int4             not null,
    trade_sell        int4             not null,
    volume_buy        numeric(36, 12) not null,
    volume_sell       numeric(36, 12) not null,
    primary key (token_pair_id, timestamp),

    constraint fk_token_pair foreign key (token_pair_id) references solana.token_pair (id)
);

create
or replace function jupiter.candle_1h_update_most_recent() returns trigger as $$
begin
delete
from jupiter.candle_1h_most_recent
where token_pair_id = new.token_pair_id;

insert into jupiter.candle_1h_most_recent
values (new.*);
return new;
end;
$$
language plpgsql;

create trigger update_most_recent_candle
    after insert
    on jupiter.candle_1h
    for each row execute function jupiter.candle_1h_update_most_recent();

create table jupiter.candle_6h
(
    token_pair_id     int4             not null,
    timestamp         timestamptz      not null,
    open              numeric(36, 12) not null,
    high              numeric(36, 12) not null,
    low               numeric(36, 12) not null,
    close             numeric(36, 12) not null,
    avg               numeric(36, 12) not null,
    amount_base_buy   numeric(36, 12) not null,
    amount_quote_buy  numeric(36, 12) not null,
    amount_base_sell  numeric(36, 12) not null,
    amount_quote_sell numeric(36, 12) not null,
    trade_buy         int4             not null,
    trade_sell        int4             not null,
    volume_buy        numeric(36, 12) not null,
    volume_sell       numeric(36, 12) not null,
    primary key (token_pair_id, timestamp),

    constraint fk_token_pair foreign key (token_pair_id) references solana.token_pair (id)
) partition by hash (token_pair_id);

create table jupiter.candle_6h_1 partition of jupiter.candle_6h for values with (modulus 8, remainder 0);

create table jupiter.candle_6h_2 partition of jupiter.candle_6h for values with (modulus 8, remainder 1);

create table jupiter.candle_6h_3 partition of jupiter.candle_6h for values with (modulus 8, remainder 2);

create table jupiter.candle_6h_4 partition of jupiter.candle_6h for values with (modulus 8, remainder 3);

create table jupiter.candle_6h_5 partition of jupiter.candle_6h for values with (modulus 8, remainder 4);

create table jupiter.candle_6h_6 partition of jupiter.candle_6h for values with (modulus 8, remainder 5);

create table jupiter.candle_6h_7 partition of jupiter.candle_6h for values with (modulus 8, remainder 6);

create table jupiter.candle_6h_8 partition of jupiter.candle_6h for values with (modulus 8, remainder 7);

create index candle_6h_timestamp ON jupiter.candle_6h (timestamp desc);

create table jupiter.candle_6h_most_recent
(
    token_pair_id     int4             not null,
    timestamp         timestamptz      not null,
    open              numeric(36, 12) not null,
    high              numeric(36, 12) not null,
    low               numeric(36, 12) not null,
    close             numeric(36, 12) not null,
    avg               numeric(36, 12) not null,
    amount_base_buy   numeric(36, 12) not null,
    amount_quote_buy  numeric(36, 12) not null,
    amount_base_sell  numeric(36, 12) not null,
    amount_quote_sell numeric(36, 12) not null,
    trade_buy         int4             not null,
    trade_sell        int4             not null,
    volume_buy        numeric(36, 12) not null,
    volume_sell       numeric(36, 12) not null,
    primary key (token_pair_id, timestamp),

    constraint fk_token_pair foreign key (token_pair_id) references solana.token_pair (id)
);

create
or replace function jupiter.candle_6h_update_most_recent() returns trigger as $$
begin
delete
from jupiter.candle_6h_most_recent
where token_pair_id = new.token_pair_id;

insert into jupiter.candle_6h_most_recent
values (new.*);
return new;
end;
$$
language plpgsql;

create trigger update_most_recent_candle
    after insert
    on jupiter.candle_6h
    for each row execute function jupiter.candle_6h_update_most_recent();

create table jupiter.candle_1d
(
    token_pair_id     int4             not null,
    timestamp         timestamptz      not null,
    open              numeric(36, 12) not null,
    high              numeric(36, 12) not null,
    low               numeric(36, 12) not null,
    close             numeric(36, 12) not null,
    avg               numeric(36, 12) not null,
    amount_base_buy   numeric(36, 12) not null,
    amount_quote_buy  numeric(36, 12) not null,
    amount_base_sell  numeric(36, 12) not null,
    amount_quote_sell numeric(36, 12) not null,
    trade_buy         int4             not null,
    trade_sell        int4             not null,
    volume_buy        numeric(36, 12) not null,
    volume_sell       numeric(36, 12) not null,
    primary key (token_pair_id, timestamp),

    constraint fk_token_pair foreign key (token_pair_id) references solana.token_pair (id)
) partition by hash (token_pair_id);

create table jupiter.candle_1d_1 partition of jupiter.candle_1d for values with (modulus 8, remainder 0);

create table jupiter.candle_1d_2 partition of jupiter.candle_1d for values with (modulus 8, remainder 1);

create table jupiter.candle_1d_3 partition of jupiter.candle_1d for values with (modulus 8, remainder 2);

create table jupiter.candle_1d_4 partition of jupiter.candle_1d for values with (modulus 8, remainder 3);

create table jupiter.candle_1d_5 partition of jupiter.candle_1d for values with (modulus 8, remainder 4);

create table jupiter.candle_1d_6 partition of jupiter.candle_1d for values with (modulus 8, remainder 5);

create table jupiter.candle_1d_7 partition of jupiter.candle_1d for values with (modulus 8, remainder 6);

create table jupiter.candle_1d_8 partition of jupiter.candle_1d for values with (modulus 8, remainder 7);

create index candle_1d_timestamp ON jupiter.candle_1d (timestamp desc);

create table jupiter.candle_1d_most_recent
(
    token_pair_id     int4             not null,
    timestamp         timestamptz      not null,
    open              numeric(36, 12) not null,
    high              numeric(36, 12) not null,
    low               numeric(36, 12) not null,
    close             numeric(36, 12) not null,
    avg               numeric(36, 12) not null,
    amount_base_buy   numeric(36, 12) not null,
    amount_quote_buy  numeric(36, 12) not null,
    amount_base_sell  numeric(36, 12) not null,
    amount_quote_sell numeric(36, 12) not null,
    trade_buy         int4             not null,
    trade_sell        int4             not null,
    volume_buy        numeric(36, 12) not null,
    volume_sell       numeric(36, 12) not null,
    primary key (token_pair_id, timestamp),

    constraint fk_token_pair foreign key (token_pair_id) references solana.token_pair (id)
);

create
or replace function jupiter.candle_1d_update_most_recent() returns trigger as $$
begin
delete
from jupiter.candle_1d_most_recent
where token_pair_id = new.token_pair_id;

insert into jupiter.candle_1d_most_recent
values (new.*);
return new;
end;
$$
language plpgsql;

create trigger update_most_recent_candle
    after insert
    on jupiter.candle_1d
    for each row execute function jupiter.candle_1d_update_most_recent();