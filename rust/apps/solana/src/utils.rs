use alloc::string::{String, ToString};

use crate::errors::{Result, SolanaError};

pub fn format_token_amount(amount: &str, decimals: u8) -> Result<String> {
    if amount.is_empty() || !amount.bytes().all(|value| value.is_ascii_digit()) {
        return Err(SolanaError::InvalidData("invalid token amount".to_string()));
    }

    let normalized = amount.trim_start_matches('0');
    let digits = if normalized.is_empty() {
        "0"
    } else {
        normalized
    };
    let decimals = decimals as usize;
    if decimals == 0 {
        return Ok(digits.to_string());
    }

    let mut result = String::new();
    if digits.len() <= decimals {
        result.push_str("0.");
        for _ in 0..decimals - digits.len() {
            result.push('0');
        }
        result.push_str(digits);
    } else {
        let split = digits.len() - decimals;
        result.push_str(&digits[..split]);
        result.push('.');
        result.push_str(&digits[split..]);
    }

    while result.ends_with('0') {
        result.pop();
    }
    if result.ends_with('.') {
        result.pop();
    }
    Ok(result)
}

// tokenAmount to human readable
pub fn token_amount_to_human_readable(token_amount: u64, decimals: u32) -> f64 {
    token_amount as f64 / 10u64.pow(decimals) as f64
}

#[cfg(test)]
mod tests {
    use super::*;
    // https://solscan.io/tx/63W8ceGvHycrseQaedkacoVnassLq4VpRdjx23fVYZnqRzwBQeTgjZL5GdNVf9B5gkJMgkF66W643T5BduWMWpis
    #[test]
    fn test_token_amount_to_human_readable() {
        assert_eq!(token_amount_to_human_readable(574810, 6), 0.57481);
    }

    #[test]
    fn test_token_amount_to_human_readable_2() {
        assert_eq!(token_amount_to_human_readable(10000, 8), 0.0001);
    }

    #[test]
    fn test_token_amount_to_human_readable_3() {
        assert_eq!(token_amount_to_human_readable(10000, 0), 10000.0);
    }

    #[test]
    fn formats_large_token_amounts_without_floating_point() {
        assert_eq!(
            format_token_amount("18446744073709551615", 6).unwrap(),
            "18446744073709.551615"
        );
        assert_eq!(format_token_amount("1000000", 6).unwrap(), "1");
        assert_eq!(format_token_amount("1", 10).unwrap(), "0.0000000001");
        assert_eq!(format_token_amount("0", u8::MAX).unwrap(), "0");
    }
}
