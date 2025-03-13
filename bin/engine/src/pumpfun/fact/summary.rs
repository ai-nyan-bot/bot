// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::model::{Fact, Facts, Value};
use bigdecimal::ToPrimitive;
use common::model::Timeframe;
use solana::model::TimeframeSummary;
use Fact::{
    MarketCapQuoteAggregate, MarketCapSolAggregate, MarketCapUsdAggregate, SwapAllChangeAggregate,
    SwapAllCountAggregate, SwapAllPercentAggregate, SwapBuyCountAggregate, SwapBuyPercentAggregate,
    SwapSellCountAggregate, SwapSellPercentAggregate,
};

pub(crate) fn add_summary_to_facts(
    facts: &mut Facts,
    summary: TimeframeSummary,
    timeframe: Timeframe,
) {
    if let Some(quote) = summary.cap.avg.quote {
        facts.set_timeframe_value(
            MarketCapQuoteAggregate,
            Value::quote(quote.0.clone()),
            timeframe,
        );

        facts.set_timeframe_value(MarketCapSolAggregate, Value::sol(quote.0), timeframe);
    }

    if let Some(usd) = summary.cap.avg.usd {
        facts.set_timeframe_value(MarketCapUsdAggregate, Value::usd(usd.0), timeframe);
    }

    if let Some(count) = summary.swap.all.count {
        facts.set_timeframe_value(SwapAllCountAggregate, count, timeframe);
    }

    if let Some(count) = summary.swap.buy.count {
        facts.set_timeframe_value(SwapBuyCountAggregate, count, timeframe);
    }

    if let Some(count) = summary.swap.sell.count {
        facts.set_timeframe_value(SwapSellCountAggregate, count, timeframe);
    }

    if let Some(change) = summary.swap.all.change {
        facts.set_timeframe_value(
            SwapAllChangeAggregate,
            Value::count(change.0.to_i64().unwrap()),
            timeframe,
        );
    }

    if let Some(percent) = summary.swap.all.percent {
        facts.set_timeframe_value(
            SwapAllPercentAggregate,
            Value::percent(percent.0),
            timeframe,
        );
    }

    if let Some(change) = summary.swap.buy.change {
        facts.set_timeframe_value(
            SwapBuyCountAggregate,
            Value::count(change.0.to_i64().unwrap()),
            timeframe,
        );
    }

    if let Some(percent) = summary.swap.buy.percent {
        facts.set_timeframe_value(
            SwapBuyPercentAggregate,
            Value::percent(percent.0),
            timeframe,
        );
    }

    if let Some(change) = summary.swap.sell.change {
        facts.set_timeframe_value(
            SwapSellCountAggregate,
            Value::count(change.0.to_i64().unwrap()),
            timeframe,
        );
    }

    if let Some(percent) = summary.swap.sell.percent {
        facts.set_timeframe_value(
            SwapSellPercentAggregate,
            Value::percent(percent.0),
            timeframe,
        );
    }
}
