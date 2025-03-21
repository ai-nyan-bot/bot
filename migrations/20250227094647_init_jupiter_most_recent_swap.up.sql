-- Copyright (c) nyanbot.com 2025.
-- This file is licensed under the AGPL-3.0-or-later.

create table jupiter.swap_most_recent as
select distinct on
(
    token_pair_id
) *
    from jupiter.swap
    order by token_pair_id, slot desc;

create unique index swap_most_recent_idx on jupiter.swap_most_recent (token_pair_id);

create function jupiter.update_most_recent_swap() returns trigger as $$
begin
delete
from jupiter.swap_most_recent
where token_pair_id = new.token_pair_id;
insert into jupiter.swap_most_recent
values (new.*);
return new;
end;
$$
language plpgsql;

create trigger update_most_recent
    after insert
    on jupiter.swap
    for each row execute function jupiter.update_most_recent_swap();