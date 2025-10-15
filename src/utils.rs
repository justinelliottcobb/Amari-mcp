use anyhow::Result;
use serde_json::Value;

/// Utility functions for the Amari MCP server

/// Validate multivector coefficients for given signature
pub fn validate_multivector_coefficients(coefficients: &[f64], signature: &[usize; 3]) -> Result<()> {
    let expected_size = 1 << (signature[0] + signature[1] + signature[2]);

    if coefficients.len() != expected_size {
        return Err(anyhow::anyhow!(
            "Expected {} coefficients for signature {:?}, got {}",
            expected_size,
            signature,
            coefficients.len()
        ));
    }

    Ok(())
}

/// Parse signature from JSON value with defaults
pub fn parse_signature(value: Option<&Value>) -> [usize; 3] {
    if let Some(sig_array) = value.and_then(|v| v.as_array()) {
        [
            sig_array.get(0).and_then(|v| v.as_u64()).unwrap_or(3) as usize,
            sig_array.get(1).and_then(|v| v.as_u64()).unwrap_or(0) as usize,
            sig_array.get(2).and_then(|v| v.as_u64()).unwrap_or(0) as usize,
        ]
    } else {
        [3, 0, 0] // Default to 3D Euclidean
    }
}

/// Convert f64 to JSON value, handling infinities and NaNs
pub fn float_to_json(value: f64) -> Value {
    if value.is_infinite() {
        if value.is_sign_positive() {
            Value::String("Infinity".to_string())
        } else {
            Value::String("-Infinity".to_string())
        }
    } else if value.is_nan() {
        Value::String("NaN".to_string())
    } else {
        Value::from(value)
    }
}

/// Convert matrix to JSON, handling special float values
pub fn matrix_to_json(matrix: &[Vec<f64>]) -> Value {
    let json_matrix: Vec<Vec<Value>> = matrix
        .iter()
        .map(|row| row.iter().map(|&val| float_to_json(val)).collect())
        .collect();

    Value::Array(json_matrix.into_iter().map(Value::Array).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_multivector_coefficients() {
        // 3D Euclidean space requires 2^3 = 8 coefficients
        let coeffs = vec![1.0, 0.5, 0.3, 0.2, 0.1, 0.15, 0.25, 0.05];
        assert!(validate_multivector_coefficients(&coeffs, &[3, 0, 0]).is_ok());

        // Wrong number of coefficients should fail
        let wrong_coeffs = vec![1.0, 2.0, 3.0, 4.0];
        assert!(validate_multivector_coefficients(&wrong_coeffs, &[3, 0, 0]).is_err());
    }

    #[test]
    fn test_parse_signature() {
        use serde_json::json;

        // Default signature
        assert_eq!(parse_signature(None), [3, 0, 0]);

        // Custom signature
        let sig_value = json!([1, 3, 0]);
        assert_eq!(parse_signature(Some(&sig_value)), [1, 3, 0]);
    }

    #[test]
    fn test_float_to_json() {
        assert_eq!(float_to_json(1.5), Value::from(1.5));
        assert_eq!(float_to_json(f64::INFINITY), Value::String("Infinity".to_string()));
        assert_eq!(float_to_json(f64::NEG_INFINITY), Value::String("-Infinity".to_string()));
        assert_eq!(float_to_json(f64::NAN), Value::String("NaN".to_string()));
    }
}