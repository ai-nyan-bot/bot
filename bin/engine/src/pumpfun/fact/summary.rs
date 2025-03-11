// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::model::{Fact, Facts, Value};
use bigdecimal::ToPrimitive;
use common::model::Timeframe;
use solana::model::TimeframeSummary;

pub(crate) fn add_summary_to_facts(
    facts: &mut Facts,
    summary: TimeframeSummary,
    timeframe: Timeframe,
) {
    if let Some(count) = summary.swap.all.count {
        facts.set_timeframe_value(Fact::SwapAllCount, count, timeframe);
    }

    if let Some(count) = summary.swap.buy.count {
        facts.set_timeframe_value(Fact::SwapBuyCount, count, timeframe);
    }

    if let Some(count) = summary.swap.sell.count {
        facts.set_timeframe_value(Fact::SwapSellCount, count, timeframe);
    }

    if let Some(change) = summary.swap.all.change {
        facts.set_timeframe_value(
            Fact::SwapAllChangeCount,
            Value::count(change.0.to_i64().unwrap()),
            timeframe,
        );
    }

    if let Some(percent) = summary.swap.all.percent {
        facts.set_timeframe_value(
            Fact::SwapChangePercent,
            Value::percent(percent.0),
            timeframe,
        );
    }

    if let Some(change) = summary.swap.buy.change {
        facts.set_timeframe_value(
            Fact::SwapBuyCount,
            Value::count(change.0.to_i64().unwrap()),
            timeframe,
        );
    }

    if let Some(percent) = summary.swap.buy.percent {
        facts.set_timeframe_value(
            Fact::SwapBuyChangePercent,
            Value::percent(percent.0),
            timeframe,
        );
    }

    if let Some(change) = summary.swap.sell.change {
        facts.set_timeframe_value(
            Fact::SwapSellCount,
            Value::count(change.0.to_i64().unwrap()),
            timeframe,
        );
    }

    if let Some(percent) = summary.swap.sell.percent {
        facts.set_timeframe_value(
            Fact::SwapSellChangePercent,
            Value::percent(percent.0),
            timeframe,
        );
    }
}
