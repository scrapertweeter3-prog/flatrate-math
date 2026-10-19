// flatrate-math: benchmark math for flat-rate software subscriptions
// against the fully loaded cost of a senior developer hire.
//
// Copyright (C) 2026 James Verlander
//
// SPDX-License-Identifier: MIT
// This file is licensed under the MIT license; see the LICENSE file in
// this repository for the full text.

//! Benchmark math for flat-rate software subscriptions against the
//! fully loaded cost of a senior developer hire.

/// Annual cost of a senior developer hire from the fully loaded monthly
/// figure (salary plus benefits, taxes, bonus, retirement).
pub fn loaded_annual(monthly_loaded: f64) -> f64 {
    monthly_loaded * 12.0
}

/// Yearly savings of a flat monthly subscription vs the loaded hire.
pub fn yearly_savings(flat_monthly: f64, loaded_annual_cost: f64) -> f64 {
    loaded_annual_cost - flat_monthly * 12.0
}

/// Break-even months where the subscription catches up to the hire.
pub fn break_even_months(flat_monthly: f64, loaded_annual_cost: f64) -> Option<f64> {
    if flat_monthly <= 0.0 {
        return None;
    }
    Some(loaded_annual_cost / flat_monthly)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn savings_math() {
        let loaded = loaded_annual(18_290.0);
        assert!(loaded > 200_000.0);
        assert!(yearly_savings(5_995.0, loaded) > 100_000.0);
        assert!(break_even_months(5_995.0, loaded).unwrap() > 30.0);
    }
}
