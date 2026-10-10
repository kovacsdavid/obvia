/*
 * This file is part of the Obvia ERP.
 *
 * Copyright (C) 2025 Kovács Dávid <kapcsolat@kovacsdavid.dev>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use bigdecimal::BigDecimal;
use rand::rngs::{StdRng, SysRng};
use rand::{RngExt, SeedableRng};

pub fn generate_string_csprng(length: usize) -> Result<String, &'static str> {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

    let mut rng = StdRng::try_from_rng(&mut SysRng).map_err(|_| "could not construct rng")?;

    Ok((0..length)
        .map(|_| {
            let idx = rng.random_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect())
}

#[cfg_attr(not(test), expect(unused))]
pub fn thousand_separated_number_i64(n: i64) -> String {
    let s = n.unsigned_abs().to_string();
    let mut out = String::with_capacity(s.len() + s.len() / 3);

    for (i, ch) in s.chars().rev().enumerate() {
        if i != 0 && i % 3 == 0 {
            out.push(' ');
        }
        out.push(ch);
    }

    let mut out: String = out.chars().rev().collect();

    if n < 0 {
        out.insert(0, '-');
    }

    out
}

pub fn thousand_separated_number_bigdecimal(x: &BigDecimal, decimals: usize) -> String {
    let rounded = x.round(decimals as i64);
    let formatted = rounded.to_string();

    let (sign, rest) = if let Some(stripped) = formatted.strip_prefix('-') {
        ("-", stripped)
    } else {
        ("", formatted.as_str())
    };

    let (int_part, frac_part) = match rest.split_once('.') {
        Some((i, f)) => (i, Some(f)),
        None => (rest, None),
    };

    let mut grouped_rev = String::with_capacity(int_part.len() + int_part.len() / 3);
    for (i, ch) in int_part.chars().rev().enumerate() {
        if i != 0 && i % 3 == 0 {
            grouped_rev.push(' ');
        }
        grouped_rev.push(ch);
    }

    let int_with_spaces: String = grouped_rev.chars().rev().collect();

    match frac_part {
        Some(f) if decimals > 0 => {
            let padded = format!("{f:0<width$}", width = decimals);
            format!("{sign}{int_with_spaces}.{padded}")
        }
        Some(_) => format!("{sign}{int_with_spaces}"),
        None if decimals > 0 => {
            format!("{sign}{int_with_spaces}.{}", "0".repeat(decimals))
        }
        None => format!("{sign}{int_with_spaces}"),
    }
}

#[cfg_attr(not(test), expect(unused))]
pub fn thousand_separated_number_f64(x: f64, decimals: usize) -> String {
    if x.is_nan() {
        return "NaN".to_string();
    }

    if x.is_infinite() {
        return if x.is_sign_negative() {
            "-inf".to_string()
        } else {
            "inf".to_string()
        };
    }

    let sign = if x.is_sign_negative() { "-" } else { "" };
    let abs = x.abs();
    let formatted = format!("{:.*}", decimals, abs);

    // If Rust ever emits scientific notation here, leave it untouched except for sign.
    if formatted.contains('e') || formatted.contains('E') {
        return format!("{sign}{formatted}");
    }

    let (int_part, frac_part) = match formatted.split_once('.') {
        Some((i, f)) => (i, Some(f)),
        None => (formatted.as_str(), None),
    };

    let mut grouped_rev = String::with_capacity(int_part.len() + int_part.len() / 3);
    for (i, ch) in int_part.chars().rev().enumerate() {
        if i != 0 && i % 3 == 0 {
            grouped_rev.push(' ');
        }
        grouped_rev.push(ch);
    }

    let int_with_spaces: String = grouped_rev.chars().rev().collect();

    match frac_part {
        Some(f) => format!("{sign}{int_with_spaces}.{f}"),
        None => format!("{sign}{int_with_spaces}"),
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    // ---------- i64 tests ----------

    #[test]
    fn i64_zero() {
        assert_eq!(thousand_separated_number_i64(0), "0");
    }

    #[test]
    fn i64_small_positive_values() {
        assert_eq!(thousand_separated_number_i64(7), "7");
        assert_eq!(thousand_separated_number_i64(42), "42");
        assert_eq!(thousand_separated_number_i64(999), "999");
    }

    #[test]
    fn i64_grouping_boundaries() {
        assert_eq!(thousand_separated_number_i64(1_000), "1 000");
        assert_eq!(thousand_separated_number_i64(1_001), "1 001");
        assert_eq!(thousand_separated_number_i64(12_345), "12 345");
        assert_eq!(thousand_separated_number_i64(123_456), "123 456");
        assert_eq!(thousand_separated_number_i64(1_234_567), "1 234 567");
    }

    #[test]
    fn i64_large_positive_values() {
        assert_eq!(
            thousand_separated_number_i64(9_223_372_036_854_775_807),
            "9 223 372 036 854 775 807"
        );
        assert_eq!(
            thousand_separated_number_i64(1_000_000_000_000_000_000),
            "1 000 000 000 000 000 000"
        );
    }

    #[test]
    fn i64_small_negative_values() {
        assert_eq!(thousand_separated_number_i64(-7), "-7");
        assert_eq!(thousand_separated_number_i64(-42), "-42");
        assert_eq!(thousand_separated_number_i64(-999), "-999");
    }

    #[test]
    fn i64_negative_grouping_boundaries() {
        assert_eq!(thousand_separated_number_i64(-1_000), "-1 000");
        assert_eq!(thousand_separated_number_i64(-1_001), "-1 001");
        assert_eq!(thousand_separated_number_i64(-12_345), "-12 345");
        assert_eq!(thousand_separated_number_i64(-123_456), "-123 456");
        assert_eq!(thousand_separated_number_i64(-1_234_567), "-1 234 567");
    }

    #[test]
    fn i64_large_negative_values() {
        assert_eq!(
            thousand_separated_number_i64(-9_223_372_036_854_775_807),
            "-9 223 372 036 854 775 807"
        );
        assert_eq!(
            thousand_separated_number_i64(-1_000_000_000_000_000_000),
            "-1 000 000 000 000 000 000"
        );
    }

    #[test]
    fn i64_boundary_transitions() {
        assert_eq!(thousand_separated_number_i64(999), "999");
        assert_eq!(thousand_separated_number_i64(1_000), "1 000");
        assert_eq!(thousand_separated_number_i64(999_999), "999 999");
        assert_eq!(thousand_separated_number_i64(1_000_000), "1 000 000");

        assert_eq!(thousand_separated_number_i64(-999), "-999");
        assert_eq!(thousand_separated_number_i64(-1_000), "-1 000");
        assert_eq!(thousand_separated_number_i64(-999_999), "-999 999");
        assert_eq!(thousand_separated_number_i64(-1_000_000), "-1 000 000");
    }

    // ---------- f64 tests ----------
    //
    // These tests intentionally avoid:
    // - tie-to-even / halfway cases like x.5
    // - values like x.995 that are sensitive to binary FP representation
    // - integers larger than 2^53 - 1, which are not exactly representable in f64

    #[test]
    fn f64_zero_formatting() {
        assert_eq!(thousand_separated_number_f64(0.0, 0), "0");
        assert_eq!(thousand_separated_number_f64(0.0, 2), "0.00");
        assert_eq!(thousand_separated_number_f64(0.0, 5), "0.00000");
    }

    #[test]
    fn f64_negative_zero_keeps_sign() {
        assert_eq!(thousand_separated_number_f64(-0.0, 0), "-0");
        assert_eq!(thousand_separated_number_f64(-0.0, 2), "-0.00");
    }

    #[test]
    fn f64_positive_without_grouping() {
        assert_eq!(thousand_separated_number_f64(12.25, 2), "12.25");
        assert_eq!(thousand_separated_number_f64(999.12, 2), "999.12");
    }

    #[test]
    fn f64_positive_with_grouping() {
        assert_eq!(thousand_separated_number_f64(1_234.25, 2), "1 234.25");
        assert_eq!(thousand_separated_number_f64(12_345.125, 3), "12 345.125");
        assert_eq!(
            thousand_separated_number_f64(1_234_567.875, 3),
            "1 234 567.875"
        );
    }

    #[test]
    fn f64_negative_with_grouping() {
        assert_eq!(thousand_separated_number_f64(-1_234.25, 2), "-1 234.25");
        assert_eq!(thousand_separated_number_f64(-12_345.125, 3), "-12 345.125");
        assert_eq!(
            thousand_separated_number_f64(-1_234_567.875, 3),
            "-1 234 567.875"
        );
    }

    #[test]
    fn f64_fractional_padding() {
        assert_eq!(thousand_separated_number_f64(1_234.25, 4), "1 234.2500");
        assert_eq!(thousand_separated_number_f64(-1_234.25, 4), "-1 234.2500");
    }

    #[test]
    fn f64_rounding_down_is_stable() {
        assert_eq!(thousand_separated_number_f64(1_234.24, 1), "1 234.2");
        assert_eq!(thousand_separated_number_f64(1_234.249, 2), "1 234.25");
        assert_eq!(thousand_separated_number_f64(98765.4321, 3), "98 765.432");
    }

    #[test]
    fn f64_rounding_up_is_stable() {
        assert_eq!(thousand_separated_number_f64(1_234.26, 1), "1 234.3");
        assert_eq!(thousand_separated_number_f64(1_234.251, 2), "1 234.25");
        assert_eq!(thousand_separated_number_f64(98765.4329, 3), "98 765.433");
    }

    #[test]
    fn f64_rounding_can_change_integer_part() {
        assert_eq!(thousand_separated_number_f64(999.999, 2), "1 000.00");
        assert_eq!(thousand_separated_number_f64(9_999.999, 2), "10 000.00");
    }

    #[test]
    fn f64_rounding_can_change_grouped_integer_part() {
        assert_eq!(
            thousand_separated_number_f64(999_999.999, 2),
            "1 000 000.00"
        );
        assert_eq!(
            thousand_separated_number_f64(-999_999.999, 2),
            "-1 000 000.00"
        );
    }

    #[test]
    fn f64_integer_values() {
        assert_eq!(thousand_separated_number_f64(1_234_567.0, 0), "1 234 567");
        assert_eq!(
            thousand_separated_number_f64(1_234_567.0, 3),
            "1 234 567.000"
        );
        assert_eq!(
            thousand_separated_number_f64(-1_234_567.0, 3),
            "-1 234 567.000"
        );
    }

    #[test]
    fn f64_large_exact_integer_values_within_safe_range() {
        assert_eq!(
            thousand_separated_number_f64(9_007_199_254_740_991.0, 0),
            "9 007 199 254 740 991"
        );
        assert_eq!(
            thousand_separated_number_f64(-9_007_199_254_740_991.0, 0),
            "-9 007 199 254 740 991"
        );
    }

    #[test]
    fn f64_nan_formats_as_nan() {
        assert_eq!(thousand_separated_number_f64(f64::NAN, 2), "NaN");
    }

    #[test]
    fn f64_positive_infinity_formats_as_inf() {
        assert_eq!(thousand_separated_number_f64(f64::INFINITY, 2), "inf");
    }

    #[test]
    fn f64_negative_infinity_formats_as_negative_inf() {
        assert_eq!(thousand_separated_number_f64(f64::NEG_INFINITY, 2), "-inf");
    }

    // ---------- BigDecimal tests ----------

    fn bd(s: &str) -> BigDecimal {
        BigDecimal::from_str(s).unwrap()
    }

    #[test]
    fn bigdecimal_zero_formatting() {
        assert_eq!(thousand_separated_number_bigdecimal(&bd("0"), 0), "0");
        assert_eq!(thousand_separated_number_bigdecimal(&bd("0"), 2), "0.00");
        assert_eq!(thousand_separated_number_bigdecimal(&bd("0"), 5), "0.00000");
    }

    #[test]
    fn bigdecimal_negative_zero_formats_as_zero() {
        assert_eq!(thousand_separated_number_bigdecimal(&bd("0"), 0), "0");
        assert_eq!(thousand_separated_number_bigdecimal(&bd("0"), 2), "0.00");
    }

    #[test]
    fn bigdecimal_positive_without_grouping() {
        assert_eq!(
            thousand_separated_number_bigdecimal(&bd("12.25"), 2),
            "12.25"
        );
        assert_eq!(
            thousand_separated_number_bigdecimal(&bd("999.12"), 2),
            "999.12"
        );
    }

    #[test]
    fn bigdecimal_positive_with_grouping() {
        assert_eq!(
            thousand_separated_number_bigdecimal(&bd("1234.25"), 2),
            "1 234.25"
        );
        assert_eq!(
            thousand_separated_number_bigdecimal(&bd("12345.125"), 3),
            "12 345.125"
        );
        assert_eq!(
            thousand_separated_number_bigdecimal(&bd("1234567.875"), 3),
            "1 234 567.875"
        );
    }

    #[test]
    fn bigdecimal_negative_with_grouping() {
        assert_eq!(
            thousand_separated_number_bigdecimal(&bd("-1234.25"), 2),
            "-1 234.25"
        );
        assert_eq!(
            thousand_separated_number_bigdecimal(&bd("-12345.125"), 3),
            "-12 345.125"
        );
        assert_eq!(
            thousand_separated_number_bigdecimal(&bd("-1234567.875"), 3),
            "-1 234 567.875"
        );
    }

    #[test]
    fn bigdecimal_fractional_padding() {
        assert_eq!(
            thousand_separated_number_bigdecimal(&bd("1234.25"), 4),
            "1 234.2500"
        );
        assert_eq!(
            thousand_separated_number_bigdecimal(&bd("-1234.25"), 4),
            "-1 234.2500"
        );
    }

    #[test]
    fn bigdecimal_rounding_down() {
        assert_eq!(
            thousand_separated_number_bigdecimal(&bd("1234.24"), 1),
            "1 234.2"
        );
        assert_eq!(
            thousand_separated_number_bigdecimal(&bd("1234.249"), 2),
            "1 234.25"
        );
        assert_eq!(
            thousand_separated_number_bigdecimal(&bd("98765.4321"), 3),
            "98 765.432"
        );
    }

    #[test]
    fn bigdecimal_rounding_up() {
        assert_eq!(
            thousand_separated_number_bigdecimal(&bd("1234.26"), 1),
            "1 234.3"
        );
        assert_eq!(
            thousand_separated_number_bigdecimal(&bd("1234.251"), 2),
            "1 234.25"
        );
        assert_eq!(
            thousand_separated_number_bigdecimal(&bd("98765.4329"), 3),
            "98 765.433"
        );
    }

    #[test]
    fn bigdecimal_rounding_can_change_integer_part() {
        assert_eq!(
            thousand_separated_number_bigdecimal(&bd("999.999"), 2),
            "1 000.00"
        );
        assert_eq!(
            thousand_separated_number_bigdecimal(&bd("9999.999"), 2),
            "10 000.00"
        );
    }

    #[test]
    fn bigdecimal_rounding_can_change_grouped_integer_part() {
        assert_eq!(
            thousand_separated_number_bigdecimal(&bd("999999.999"), 2),
            "1 000 000.00"
        );
        assert_eq!(
            thousand_separated_number_bigdecimal(&bd("-999999.999"), 2),
            "-1 000 000.00"
        );
    }

    #[test]
    fn bigdecimal_integer_values() {
        assert_eq!(
            thousand_separated_number_bigdecimal(&bd("1234567"), 0),
            "1 234 567"
        );
        assert_eq!(
            thousand_separated_number_bigdecimal(&bd("1234567"), 3),
            "1 234 567.000"
        );
        assert_eq!(
            thousand_separated_number_bigdecimal(&bd("-1234567"), 3),
            "-1 234 567.000"
        );
    }

    #[test]
    fn bigdecimal_large_integer_values() {
        assert_eq!(
            thousand_separated_number_bigdecimal(&bd("9007199254740991"), 0),
            "9 007 199 254 740 991"
        );
        assert_eq!(
            thousand_separated_number_bigdecimal(&bd("-9007199254740991"), 0),
            "-9 007 199 254 740 991"
        );
    }

    #[test]
    fn bigdecimal_very_large_values_beyond_f64_safe_range() {
        assert_eq!(
            thousand_separated_number_bigdecimal(&bd("9223372036854775808"), 0),
            "9 223 372 036 854 775 808"
        );
        assert_eq!(
            thousand_separated_number_bigdecimal(&bd("123456789012345678901234567890.12"), 2),
            "123 456 789 012 345 678 901 234 567 890.12"
        );
    }

    #[test]
    fn bigdecimal_zero_decimals_removes_fraction() {
        assert_eq!(
            thousand_separated_number_bigdecimal(&bd("1234.999"), 0),
            "1 235"
        );
        assert_eq!(
            thousand_separated_number_bigdecimal(&bd("-1234.999"), 0),
            "-1 235"
        );
    }

    #[test]
    fn bigdecimal_halfway_rounding_behavior() {
        // Documents actual BigDecimal::round behavior used by the function.
        assert_eq!(
            thousand_separated_number_bigdecimal(&bd("1234.5"), 0),
            "1 234"
        );
        assert_eq!(
            thousand_separated_number_bigdecimal(&bd("999999.995"), 2),
            "1 000 000.00"
        );
    }
}
