// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::model::{Amount, Percent};

pub fn calculate_progress(virtual_base_reserves: Amount) -> Percent {
    let base_reserve = virtual_base_reserves.0 / 1_000_000;
    let progress = ((((1_073_000_000) - base_reserve) * 100) as f64) / (793_100_000) as f64;
    let progress = progress.clamp(0.0, 100.0) as f32;
    Percent::from(progress)
}

#[cfg(test)]
mod tests {
    use crate::pumpfun::model::{calculate_progress, CurveInfo};

    #[test]
    fn test_close_to_0_percent() {
        let test_instance = CurveInfo {
            virtual_base_reserves: 1071443444605882i64.into(),
            virtual_quote_reserves: 30043583654i64.into(),
            real_base_reserves: 791543444605882i64.into(),
            real_quote_reserves: 43583654i64.into(),
            total_supply: 1000000000000000i64.into(),
            complete: false,
        };

        let result = calculate_progress(test_instance.virtual_base_reserves);
        assert_eq!(result, 0.19626226);
    }

    #[test]
    fn test_40_percent() {
        let test_instance = CurveInfo {
            virtual_base_reserves: 757214460226289i64.into(),
            virtual_quote_reserves: 42511074286i64.into(),
            real_base_reserves: 477314460226289i64.into(),
            real_quote_reserves: 12511074286i64.into(),
            total_supply: 1000000000000000i64.into(),
            complete: false,
        };

        let result = calculate_progress(test_instance.virtual_base_reserves);
        assert_eq!(result, 39.816612);
    }

    #[test]
    fn test_70_percent() {
        let test_instance = CurveInfo {
            virtual_base_reserves: 512561011366544i64.into(),
            virtual_quote_reserves: 62802280169i64.into(),
            real_base_reserves: 232661011366544i64.into(),
            real_quote_reserves: 32802280169i64.into(),
            total_supply: 1000000000000000i64.into(),
            complete: false,
        };

        let result = calculate_progress(test_instance.virtual_base_reserves);
        assert_eq!(result, 70.66435);
    }

    #[test]
    fn test_completed() {
        let test_instance = CurveInfo {
            virtual_base_reserves: 0i64.into(),
            virtual_quote_reserves: 0i64.into(),
            real_base_reserves: 0i64.into(),
            real_quote_reserves: 0i64.into(),
            total_supply: 1000000000000000i64.into(),
            complete: true,
        };

        let result = calculate_progress(test_instance.virtual_base_reserves);
        assert_eq!(result, 100);
    }
}
