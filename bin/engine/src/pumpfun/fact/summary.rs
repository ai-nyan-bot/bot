// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::model::{Fact, Facts, Value};
use bigdecimal::ToPrimitive;
use common::model::Timeframe;
use solana::model::Summary;

pub(crate) fn add_summary_to_facts(facts: &mut Facts, summary: Summary, timeframe: Timeframe) {
    facts.set_timeframe_value(Fact::TradesCount, summary.trade.all.count, timeframe);
    facts.set_timeframe_value(Fact::TradesBuyCount, summary.trade.buy.count, timeframe);
    facts.set_timeframe_value(Fact::TradesSellCount, summary.trade.sell.count, timeframe);

    if let Some(change) = summary.trade.all.change {
        facts.set_timeframe_value(
            Fact::TradesChangeCount,
            Value::count(change.0.to_i64().unwrap()),
            timeframe,
        );
    }

    if let Some(percent) = summary.trade.all.percent {
        facts.set_timeframe_value(
            Fact::TradesChangePercent,
            Value::percent(percent.0),
            timeframe,
        );
    }

    if let Some(change) = summary.trade.buy.change {
        facts.set_timeframe_value(
            Fact::TradesBuyCount,
            Value::count(change.0.to_i64().unwrap()),
            timeframe,
        );
    }

    if let Some(percent) = summary.trade.buy.percent {
        facts.set_timeframe_value(
            Fact::TradesBuyChangePercent,
            Value::percent(percent.0),
            timeframe,
        );
    }

    if let Some(change) = summary.trade.sell.change {
        facts.set_timeframe_value(
            Fact::TradesSellCount,
            Value::count(change.0.to_i64().unwrap()),
            timeframe,
        );
    }

    if let Some(percent) = summary.trade.sell.percent {
        facts.set_timeframe_value(
            Fact::TradesSellChangePercent,
            Value::percent(percent.0),
            timeframe,
        );
    }
}
