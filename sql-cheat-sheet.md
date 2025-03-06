```sql
select *
from solana.token_pair tp
         join solana.token base on base.id = tp.base_id
         join solana.token quote on quote.id = tp.quote_id
```

```sql
select c.id, progress, base.mint, quote.mint
from pumpfun.curve c
         left join solana.token_pair tp on tp.id = c.id
         left join solana.token base on base.id = tp.base_id
         left join solana.token quote on quote.id = tp.quote_id
where progress < 95
order by progress desc;
```

```sql
with latest as (
    select updated_at
    from pumpfun.curve
    order by updated_at desc
    limit 1
    )
select
    c.*,
    extract(epoch from (latest.updated_at - c.updated_at))::int8 as age_seconds
from
    pumpfun.curve c,
    latest
where
    c.updated_at > latest.updated_at - interval  '90 seconds';
```