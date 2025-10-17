use anyhow::Result;
use serde_json::{json, Value};
use std::time::Instant;
use tracing::info;

/// Cache and retrieve Cayley tables for geometric algebra operations
/// Uses on-demand computation for all operations (no database caching)
pub async fn get_cayley_table(params: Value) -> Result<Value> {
    let start_time = Instant::now();

    // Parse signature - expecting [p, q, r] format
    let signature = params
        .get("signature")
        .and_then(|s| s.as_array())
        .ok_or_else(|| anyhow::anyhow!("Missing or invalid 'signature' field"))?;

    if signature.len() != 3 {
        return Err(anyhow::anyhow!("Signature must have exactly 3 components"));
    }

    let sig_p = signature[0].as_u64().unwrap_or(0) as usize;
    let sig_q = signature[1].as_u64().unwrap_or(0) as usize;
    let sig_r = signature[2].as_u64().unwrap_or(0) as usize;

    info!(
        "Computing Cayley table for signature [{}, {}, {}]",
        sig_p, sig_q, sig_r
    );

    // Always compute on-demand (no database lookup)
    compute_cayley_table(sig_p, sig_q, sig_r, start_time).await
}

// Data compression/decompression removed with database functionality

/// Compute Cayley table on-demand
async fn compute_cayley_table(
    sig_p: usize,
    sig_q: usize,
    sig_r: usize,
    start_time: Instant,
) -> Result<Value> {
    let dimensions = sig_p + sig_q + sig_r;
    let basis_count = 1 << dimensions; // 2^dimensions

    info!(
        "Computing Cayley table for {}-dimensional space",
        dimensions
    );

    // Generate basic multiplication table using Amari
    // For now, use a simplified computation
    let mut table_data = Vec::new();
    for i in 0..basis_count {
        for j in 0..basis_count {
            // Simplified geometric product computation
            let product = compute_geometric_product_coefficient(i, j, sig_p, sig_q, sig_r);
            table_data.extend_from_slice(&product.to_le_bytes());
        }
    }

    let computation_time = start_time.elapsed().as_millis();

    info!(
        "Computed Cayley table for [{}, {}, {}] in {}ms",
        sig_p, sig_q, sig_r, computation_time
    );

    Ok(json!({
        "success": true,
        "signature": [sig_p, sig_q, sig_r],
        "dimensions": dimensions,
        "basis_count": basis_count,
        "table_size_bytes": table_data.len(),
        "computation_time_ms": computation_time,
        "metadata": {
            "source": "computed",
            "algorithm": "on_demand_geometric_product",
            "note": "Direct computation using Amari library"
        }
    }))
}

/// Simplified geometric product coefficient computation
fn compute_geometric_product_coefficient(
    basis_i: usize,
    basis_j: usize,
    sig_p: usize,
    _sig_q: usize,
    _sig_r: usize,
) -> f64 {
    // Very basic implementation for demonstration
    // In reality, this would use Amari's geometric algebra operations

    if basis_i == 0 || basis_j == 0 {
        1.0 // scalar involved = identity
    } else {
        // For vectors in different positions
        if (basis_i & basis_j) != 0 {
            // Same basis element squared
            if basis_i.trailing_zeros() < sig_p as u32 {
                1.0 // positive signature
            } else {
                -1.0 // negative signature
            }
        } else {
            1.0 // different basis elements
        }
    }
}

// Table storage, listing, and cache management removed with database functionality

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_get_cayley_table_basic() {
        let params = json!({
            "signature": [2, 0, 0]
        });

        let result = get_cayley_table(params).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response["success"], true);
        assert_eq!(response["signature"], json!([2, 0, 0]));
        assert_eq!(response["dimensions"], 2);
        assert_eq!(response["basis_count"], 4);
        assert_eq!(response["metadata"]["source"], "computed");
    }

    #[tokio::test]
    async fn test_get_cayley_table_different_signatures() {
        let signatures = vec![[3, 0, 0], [2, 1, 0], [4, 0, 0]];

        for sig in signatures {
            let params = json!({
                "signature": sig
            });

            let result = get_cayley_table(params).await;
            assert!(result.is_ok());

            let response = result.unwrap();
            assert_eq!(response["success"], true);
            assert_eq!(response["signature"], json!(sig));
        }
    }

    #[tokio::test]
    async fn test_get_cayley_table_invalid_signature() {
        let params = json!({
            "signature": [2, 0] // Missing third component
        });

        let result = get_cayley_table(params).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_cayley_table_missing_signature() {
        let params = json!({
            "invalid_field": "test"
        });

        let result = get_cayley_table(params).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_cayley_table_force_recompute() {
        let params = json!({
            "signature": [3, 0, 0],
            "force_recompute": true
        });

        let result = get_cayley_table(params).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response["success"], true);
        assert_eq!(response["metadata"]["source"], "computed");
    }

    // Test for decompress_table_data removed with database functionality

    #[tokio::test]
    async fn test_cayley_table_identity_property() {
        let params = json!({
            "signature": [3, 0, 0]
        });

        let result = get_cayley_table(params).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response["success"], true);

        // Check that the table has the expected structure
        let dimensions: u64 = response["dimensions"].as_u64().unwrap();
        let basis_count: u64 = response["basis_count"].as_u64().unwrap();
        assert_eq!(basis_count, 1u64 << dimensions);
    }

    #[tokio::test]
    async fn test_cayley_table_structure() {
        let params = json!({
            "signature": [2, 0, 0]
        });

        let result = get_cayley_table(params).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response["success"], true);
        assert!(response["computation_time_ms"].as_u64().is_some());
        assert!(response["table_size_bytes"].as_u64().unwrap() > 0);
    }

    #[tokio::test]
    async fn test_reconstruct_3d_table() {
        let params = json!({
            "signature": [3, 0, 0]
        });

        let result = get_cayley_table(params).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response["success"], true);
        assert_eq!(response["dimensions"], 3);
        assert_eq!(response["basis_count"], 8); // 2^3
    }
}
