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