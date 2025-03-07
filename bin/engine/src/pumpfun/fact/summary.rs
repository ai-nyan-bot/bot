// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::model::{Fact, Facts, Value};
use bigdecimal::ToPrimitive;
use common::model::Timeframe;
use solana::model::Summary;

pub(crate) fn add_summary_to_facts(facts: &mut Facts, summary: Summary, timeframe: Timeframe) {
    facts.set_timeframe_value(Fact::SwapsCount, summary.swap.all.count, timeframe);
    facts.set_timeframe_value(Fact::SwapsBuyCount, summary.swap.buy.count, timeframe);
    facts.set_timeframe_value(Fact::SwapsSellCount, summary.swap.sell.count, timeframe);

    if let Some(change) = summary.swap.all.change {
        facts.set_timeframe_value(
            Fact::SwapsChangeCount,
            Value::count(change.0.to_i64().unwrap()),
            timeframe,
        );
    }

    if let Some(percent) = summary.swap.all.percent {
        facts.set_timeframe_value(
            Fact::SwapsChangePercent,
            Value::percent(percent.0),
            timeframe,
        );
    }

    if let Some(change) = summary.swap.buy.change {
        facts.set_timeframe_value(
            Fact::SwapsBuyCount,
            Value::count(change.0.to_i64().unwrap()),
            timeframe,
        );
    }

    if let Some(percent) = summary.swap.buy.percent {
        facts.set_timeframe_value(
            Fact::SwapsBuyChangePercent,
            Value::percent(percent.0),
            timeframe,
        );
    }

    if let Some(change) = summary.swap.sell.change {
        facts.set_timeframe_value(
            Fact::SwapsSellCount,
            Value::count(change.0.to_i64().unwrap()),
            timeframe,
        );
    }

    if let Some(percent) = summary.swap.sell.percent {
        facts.set_timeframe_value(
            Fact::SwapsSellChangePercent,
            Value::percent(percent.0),
            timeframe,
        );
    }
}
