# Token pair with base and quote token
```sql
select *
from solana.token_pair tp
         join solana.token base on base.id = tp.base_id
         join solana.token quote on quote.id = tp.quote_id
```

# Pumpfun currents which have progressed less than 95 percent
```sql
select c.id, progress, base.mint, quote.mint
from pumpfun.current c
         left join solana.token_pair tp on tp.id = c.id
         left join solana.token base on base.id = tp.base_id
         left join solana.token quote on quote.id = tp.quote_id
where progress < 95
order by progress desc;
```

# List currents which have been updated in the last 90 seconds relative to the most recent updated current
```sql
with latest as (select updated_at
                from pumpfun.current
                order by updated_at desc
    limit 1
    )
select c.*,
       extract(epoch from (latest.updated_at - c.updated_at)) ::int8 as age_seconds
from pumpfun.current c,
     latest
where c.updated_at > latest.updated_at - interval '90 seconds';
```

# Lag between most recent swap and 1s candle
```sql
with latest as (select timestamp from pumpfun.swap order by timestamp desc limit 1)
select
    age((select timestamp from latest), timestamp)
from pumpfun.candle_1s
order by timestamp desc
    limit 1;
```

# Reset analytics

```sql
delete
from pumpfun.candle_1s;
delete
from pumpfun.candle_1s_most_recent;
delete
from pumpfun.candle_1m;
delete
from pumpfun.candle_5m;
delete
from pumpfun.candle_15m;
delete
from pumpfun.candle_1h;
delete
from pumpfun.candle_6h;
delete
from pumpfun.candle_1d;

delete
from pumpfun.twap_1m;
delete
from pumpfun.twap_5m;
delete
from pumpfun.twap_15m;
delete
from pumpfun.twap_1h;
delete
from pumpfun.twap_6h;
delete
from pumpfun.twap_1d;

delete
from pumpfun.candle_market_cap_1m;
delete
from pumpfun.candle_market_cap_5m;
delete
from pumpfun.candle_market_cap_15m;
delete
from pumpfun.candle_market_cap_1h;
delete
from pumpfun.candle_market_cap_6h;
delete
from pumpfun.candle_market_cap_1d;

delete
from pumpfun.candle_progress_1m;
delete
from pumpfun.candle_progress_5m;
delete
from pumpfun.candle_progress_15m;
delete
from pumpfun.candle_progress_1h;
delete
from pumpfun.candle_progress_6h;
delete
from pumpfun.candle_progress_1d;

delete
from pumpfun.candle_usd_1m;
delete
from pumpfun.candle_usd_5m;
delete
from pumpfun.candle_usd_15m;
delete
from pumpfun.candle_usd_1h;
delete
from pumpfun.candle_usd_6h;
delete
from pumpfun.candle_usd_1d;

delete
from pumpfun.summary_1m;
delete
from pumpfun.summary_5m;
delete
from pumpfun.summary_15m;
delete
from pumpfun.summary_1h;
delete
from pumpfun.summary_6h;
delete
from pumpfun.summary_1d;

delete
from jupiter.candle_1s;
delete
from jupiter.candle_1s_most_recent;
delete
from jupiter.candle_1m;
delete
from jupiter.candle_5m;
delete
from jupiter.candle_15m;
delete
from jupiter.candle_1h;
delete
from jupiter.candle_6h;
delete
from jupiter.candle_1d;

delete
from jupiter.twap_1m;
delete
from jupiter.twap_5m;
delete
from jupiter.twap_15m;
delete
from jupiter.twap_1h;
delete
from jupiter.twap_6h;
delete
from jupiter.twap_1d;

delete
from jupiter.candle_market_cap_1m;
delete
from jupiter.candle_market_cap_5m;
delete
from jupiter.candle_market_cap_15m;
delete
from jupiter.candle_market_cap_1h;
delete
from jupiter.candle_market_cap_6h;
delete
from jupiter.candle_market_cap_1d;

delete
from jupiter.candle_usd_1m;
delete
from jupiter.candle_usd_5m;
delete
from jupiter.candle_usd_15m;
delete
from jupiter.candle_usd_1h;
delete
from jupiter.candle_usd_6h;
delete
from jupiter.candle_usd_1d;

```