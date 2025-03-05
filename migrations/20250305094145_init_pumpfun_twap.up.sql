-- Copyright (c) nyanbot.com 2025.
-- This file is licensed under the AGPL-3.0-or-later.

create table pumpfun.twap_1m
(
    token_pair_id int4                     not null,
    timestamp     timestamp with time zone not null,
    twap          double precision         not null,
    primary key (token_pair_id, timestamp),
    constraint fk_token_pair foreign key (token_pair_id) references solana.token_pair (id)
) partition by hash (token_pair_id);

create index twap_1m_timestamp ON pumpfun.twap_1m (timestamp desc);

create table pumpfun.twap_1m_1 partition of pumpfun.twap_1m for values with (modulus 8, remainder 0);

create table pumpfun.twap_1m_2 partition of pumpfun.twap_1m for values with (modulus 8, remainder 1);

create table pumpfun.twap_1m_3 partition of pumpfun.twap_1m for values with (modulus 8, remainder 2);

create table pumpfun.twap_1m_4 partition of pumpfun.twap_1m for values with (modulus 8, remainder 3);

create table pumpfun.twap_1m_5 partition of pumpfun.twap_1m for values with (modulus 8, remainder 4);

create table pumpfun.twap_1m_6 partition of pumpfun.twap_1m for values with (modulus 8, remainder 5);

create table pumpfun.twap_1m_7 partition of pumpfun.twap_1m for values with (modulus 8, remainder 6);

create table pumpfun.twap_1m_8 partition of pumpfun.twap_1m for values with (modulus 8, remainder 7);

create table pumpfun.twap_1m_most_recent as
select distinct on
(
    token_pair_id
) * from pumpfun.twap_1m order by token_pair_id, timestamp desc;

create unique index twap_1m_most_recent_idx on pumpfun.twap_1m_most_recent (token_pair_id);

create function pumpfun.update_most_recent_twap_1m() returns trigger as $$
begin
delete
from pumpfun.twap_1m_most_recent
where token_pair_id = new.token_pair_id;
insert into pumpfun.twap_1m_most_recent
values (new.*);
return new;
end;
$$
language plpgsql;

create trigger update_most_recent
    after insert
    on pumpfun.twap_1m
    for each row execute function pumpfun.update_most_recent_twap_1m();


create table pumpfun.twap_5m
(
    token_pair_id int4                     not null,
    timestamp     timestamp with time zone not null,
    twap          double precision         not null,
    primary key (token_pair_id, timestamp),
    constraint fk_token_pair foreign key (token_pair_id) references solana.token_pair (id)
) partition by hash (token_pair_id);

create index twap_5m_timestamp ON pumpfun.twap_5m (timestamp desc);

create table pumpfun.twap_5m_1 partition of pumpfun.twap_5m for values with (modulus 8, remainder 0);

create table pumpfun.twap_5m_2 partition of pumpfun.twap_5m for values with (modulus 8, remainder 1);

create table pumpfun.twap_5m_3 partition of pumpfun.twap_5m for values with (modulus 8, remainder 2);

create table pumpfun.twap_5m_4 partition of pumpfun.twap_5m for values with (modulus 8, remainder 3);

create table pumpfun.twap_5m_5 partition of pumpfun.twap_5m for values with (modulus 8, remainder 4);

create table pumpfun.twap_5m_6 partition of pumpfun.twap_5m for values with (modulus 8, remainder 5);

create table pumpfun.twap_5m_7 partition of pumpfun.twap_5m for values with (modulus 8, remainder 6);

create table pumpfun.twap_5m_8 partition of pumpfun.twap_5m for values with (modulus 8, remainder 7);

create table pumpfun.twap_5m_most_recent as
select distinct on
(
    token_pair_id
) * from pumpfun.twap_5m order by token_pair_id, timestamp desc;

create unique index twap_5m_most_recent_idx on pumpfun.twap_5m_most_recent (token_pair_id);

create function pumpfun.update_most_recent_twap_5m() returns trigger as $$
begin
delete
from pumpfun.twap_5m_most_recent
where token_pair_id = new.token_pair_id;
insert into pumpfun.twap_5m_most_recent
values (new.*);
return new;
end;
$$
language plpgsql;

create trigger update_most_recent
    after insert
    on pumpfun.twap_5m
    for each row execute function pumpfun.update_most_recent_twap_5m();


create table pumpfun.twap_15m
(
    token_pair_id int4                     not null,
    timestamp     timestamp with time zone not null,
    twap          double precision         not null,
    primary key (token_pair_id, timestamp),
    constraint fk_token_pair foreign key (token_pair_id) references solana.token_pair (id)
) partition by hash (token_pair_id);

create index twap_15m_timestamp ON pumpfun.twap_15m (timestamp desc);

create table pumpfun.twap_15m_1 partition of pumpfun.twap_15m for values with (modulus 8, remainder 0);

create table pumpfun.twap_15m_2 partition of pumpfun.twap_15m for values with (modulus 8, remainder 1);

create table pumpfun.twap_15m_3 partition of pumpfun.twap_15m for values with (modulus 8, remainder 2);

create table pumpfun.twap_15m_4 partition of pumpfun.twap_15m for values with (modulus 8, remainder 3);

create table pumpfun.twap_15m_5 partition of pumpfun.twap_15m for values with (modulus 8, remainder 4);

create table pumpfun.twap_15m_6 partition of pumpfun.twap_15m for values with (modulus 8, remainder 5);

create table pumpfun.twap_15m_7 partition of pumpfun.twap_15m for values with (modulus 8, remainder 6);

create table pumpfun.twap_15m_8 partition of pumpfun.twap_15m for values with (modulus 8, remainder 7);

create table pumpfun.twap_15m_most_recent as
select distinct on
(
    token_pair_id
) * from pumpfun.twap_15m order by token_pair_id, timestamp desc;

create unique index twap_15m_most_recent_idx on pumpfun.twap_15m_most_recent (token_pair_id);

create function pumpfun.update_most_recent_twap_15m() returns trigger as $$
begin
delete
from pumpfun.twap_15m_most_recent
where token_pair_id = new.token_pair_id;
insert into pumpfun.twap_15m_most_recent
values (new.*);
return new;
end;
$$
language plpgsql;

create trigger update_most_recent
    after insert
    on pumpfun.twap_15m
    for each row execute function pumpfun.update_most_recent_twap_15m();

create table pumpfun.twap_1h
(
    token_pair_id int4                     not null,
    timestamp     timestamp with time zone not null,
    twap          double precision         not null,
    primary key (token_pair_id, timestamp),
    constraint fk_token_pair foreign key (token_pair_id) references solana.token_pair (id)
) partition by hash (token_pair_id);

create index twap_1h_timestamp ON pumpfun.twap_1h (timestamp desc);

create table pumpfun.twap_1h_1 partition of pumpfun.twap_1h for values with (modulus 8, remainder 0);

create table pumpfun.twap_1h_2 partition of pumpfun.twap_1h for values with (modulus 8, remainder 1);

create table pumpfun.twap_1h_3 partition of pumpfun.twap_1h for values with (modulus 8, remainder 2);

create table pumpfun.twap_1h_4 partition of pumpfun.twap_1h for values with (modulus 8, remainder 3);

create table pumpfun.twap_1h_5 partition of pumpfun.twap_1h for values with (modulus 8, remainder 4);

create table pumpfun.twap_1h_6 partition of pumpfun.twap_1h for values with (modulus 8, remainder 5);

create table pumpfun.twap_1h_7 partition of pumpfun.twap_1h for values with (modulus 8, remainder 6);

create table pumpfun.twap_1h_8 partition of pumpfun.twap_1h for values with (modulus 8, remainder 7);

create table pumpfun.twap_1h_most_recent as
select distinct on
(
    token_pair_id
) * from pumpfun.twap_1h order by token_pair_id, timestamp desc;

create unique index twap_1h_most_recent_idx on pumpfun.twap_1h_most_recent (token_pair_id);

create function pumpfun.update_most_recent_twap_1h() returns trigger as $$
begin
delete
from pumpfun.twap_1h_most_recent
where token_pair_id = new.token_pair_id;
insert into pumpfun.twap_1h_most_recent
values (new.*);
return new;
end;
$$
language plpgsql;

create trigger update_most_recent
    after insert
    on pumpfun.twap_1h
    for each row execute function pumpfun.update_most_recent_twap_1h();


create table pumpfun.twap_6h
(
    token_pair_id int4                     not null,
    timestamp     timestamp with time zone not null,
    twap          double precision         not null,
    primary key (token_pair_id, timestamp),
    constraint fk_token_pair foreign key (token_pair_id) references solana.token_pair (id)
) partition by hash (token_pair_id);

create index twap_6h_timestamp ON pumpfun.twap_6h (timestamp desc);

create table pumpfun.twap_6h_1 partition of pumpfun.twap_6h for values with (modulus 8, remainder 0);

create table pumpfun.twap_6h_2 partition of pumpfun.twap_6h for values with (modulus 8, remainder 1);

create table pumpfun.twap_6h_3 partition of pumpfun.twap_6h for values with (modulus 8, remainder 2);

create table pumpfun.twap_6h_4 partition of pumpfun.twap_6h for values with (modulus 8, remainder 3);

create table pumpfun.twap_6h_5 partition of pumpfun.twap_6h for values with (modulus 8, remainder 4);

create table pumpfun.twap_6h_6 partition of pumpfun.twap_6h for values with (modulus 8, remainder 5);

create table pumpfun.twap_6h_7 partition of pumpfun.twap_6h for values with (modulus 8, remainder 6);

create table pumpfun.twap_6h_8 partition of pumpfun.twap_6h for values with (modulus 8, remainder 7);

create table pumpfun.twap_6h_most_recent as
select distinct on
(
    token_pair_id
) * from pumpfun.twap_6h order by token_pair_id, timestamp desc;

create unique index twap_6h_most_recent_idx on pumpfun.twap_6h_most_recent (token_pair_id);

create function pumpfun.update_most_recent_twap_6h() returns trigger as $$
begin
delete
from pumpfun.twap_6h_most_recent
where token_pair_id = new.token_pair_id;
insert into pumpfun.twap_6h_most_recent
values (new.*);
return new;
end;
$$
language plpgsql;

create trigger update_most_recent
    after insert
    on pumpfun.twap_6h
    for each row execute function pumpfun.update_most_recent_twap_6h();

create table pumpfun.twap_1d
(
    token_pair_id int4                     not null,
    timestamp     timestamp with time zone not null,
    twap          double precision         not null,
    primary key (token_pair_id, timestamp),
    constraint fk_token_pair foreign key (token_pair_id) references solana.token_pair (id)
) partition by hash (token_pair_id);

create index twap_1d_timestamp ON pumpfun.twap_1d (timestamp desc);

create table pumpfun.twap_1d_1 partition of pumpfun.twap_1d for values with (modulus 8, remainder 0);

create table pumpfun.twap_1d_2 partition of pumpfun.twap_1d for values with (modulus 8, remainder 1);

create table pumpfun.twap_1d_3 partition of pumpfun.twap_1d for values with (modulus 8, remainder 2);

create table pumpfun.twap_1d_4 partition of pumpfun.twap_1d for values with (modulus 8, remainder 3);

create table pumpfun.twap_1d_5 partition of pumpfun.twap_1d for values with (modulus 8, remainder 4);

create table pumpfun.twap_1d_6 partition of pumpfun.twap_1d for values with (modulus 8, remainder 5);

create table pumpfun.twap_1d_7 partition of pumpfun.twap_1d for values with (modulus 8, remainder 6);

create table pumpfun.twap_1d_8 partition of pumpfun.twap_1d for values with (modulus 8, remainder 7);

create table pumpfun.twap_1d_most_recent as
select distinct on
(
    token_pair_id
) * from pumpfun.twap_1d order by token_pair_id, timestamp desc;

create unique index twap_1d_most_recent_idx on pumpfun.twap_1d_most_recent (token_pair_id);

create function pumpfun.update_most_recent_twap_1d() returns trigger as $$
begin
delete
from pumpfun.twap_1d_most_recent
where token_pair_id = new.token_pair_id;
insert into pumpfun.twap_1d_most_recent
values (new.*);
return new;
end;
$$
language plpgsql;

create trigger update_most_recent
    after insert
    on pumpfun.twap_1d
    for each row execute function pumpfun.update_most_recent_twap_1d();

