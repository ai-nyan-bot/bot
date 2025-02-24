// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::pumpfun::model::{Curve, CurveInfo};
use base::model::{Amount, Percent};
use std::ops::{Div, Mul, Sub};

pub trait CalculateProgress {
    fn total_supply(&self) -> Amount;
    fn real_base_reserves(&self) -> Amount;

    fn calculate_progress(&self) -> Percent {
        let reserved_tokens: i64 = 206_900_000 * 1_000_000;
        let total_supply = self.total_supply().0;
        let real_base_reserves = self.real_base_reserves().0;
        let initial_real_base_reserves = total_supply.sub(reserved_tokens);

        Percent::from(100 - real_base_reserves.mul(100).div(initial_real_base_reserves))
    }
}

impl CalculateProgress for Curve {
    fn total_supply(&self) -> Amount {
        self.total_supply.clone()
    }

    fn real_base_reserves(&self) -> Amount {
        self.real_base_reserves.clone()
    }
}

impl CalculateProgress for CurveInfo {
    fn total_supply(&self) -> Amount {
        self.total_supply.clone()
    }

    fn real_base_reserves(&self) -> Amount {
        self.real_base_reserves.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::pumpfun::model::{CalculateProgress, CurveInfo};

    #[test]
    fn test_1_percent() {
        let test_instance = CurveInfo {
            virtual_base_reserves: 1071443444605882i64.into(),
            virtual_quote_reserves: 30043583654i64.into(),
            real_base_reserves: 791543444605882i64.into(),
            real_quote_reserves: 43583654i64.into(),
            total_supply: 1000000000000000i64.into(),
            complete: false,
        };

        let result = test_instance.calculate_progress();
        assert_eq!(result, 1);
    }

    #[test]
    fn test_71_percent() {
        let test_instance = CurveInfo {
            virtual_base_reserves: 512561011366544i64.into(),
            virtual_quote_reserves: 62802280169i64.into(),
            real_base_reserves: 232661011366544i64.into(),
            real_quote_reserves: 32802280169i64.into(),
            total_supply: 1000000000000000i64.into(),
            complete: false,
        };

        let result = test_instance.calculate_progress();
        assert_eq!(result, 71);
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

        let result = test_instance.calculate_progress();
        assert_eq!(result, 100);
    }
}
