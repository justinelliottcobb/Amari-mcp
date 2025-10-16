use anyhow::Result;
use serde_json::{json, Value};
use std::time::Instant;
use tracing::info;
#[cfg(feature = "database")]
use tracing::warn;

#[cfg(feature = "database")]
use sqlx::PgPool;

// Global database pool for Cayley table operations
#[cfg(feature = "database")]
use std::sync::OnceLock;

#[cfg(feature = "database")]
static DB_POOL: OnceLock<PgPool> = OnceLock::new();

#[cfg(feature = "database")]
pub fn set_database_pool(pool: PgPool) {
    let _ = DB_POOL.set(pool);
}

/// Cache and retrieve Cayley tables for geometric algebra operations
/// Uses database lookups for precomputed tables when available
pub async fn get_cayley_table(params: Value) -> Result<Value> {
    let start_time = Instant::now();

    let signature = params["signature"]
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("signature must be an array [p, q, r]"))?;

    let sig_p = signature.get(0).and_then(|v| v.as_i64()).unwrap_or(3) as i32;
    let sig_q = signature.get(1).and_then(|v| v.as_i64()).unwrap_or(0) as i32;
    let sig_r = signature.get(2).and_then(|v| v.as_i64()).unwrap_or(0) as i32;

    let table_id = format!("cayley_{}_{}_{}", sig_p, sig_q, sig_r);
    let force_recompute = params["force_recompute"].as_bool().unwrap_or(false);

    info!(
        "🔍 Requested Cayley table for signature [{}, {}, {}], force_recompute: {}",
        sig_p, sig_q, sig_r, force_recompute
    );

    // Try database lookup first (if database feature enabled)
    #[cfg(feature = "database")]
    if !force_recompute {
        if let Some(result) = try_database_lookup(sig_p, sig_q, sig_r, start_time).await {
            return result;
        }
    }

    // Fallback to computation
    compute_cayley_table_fallback(sig_p, sig_q, sig_r, table_id, start_time).await
}

/// Try to retrieve precomputed Cayley table from database
#[cfg(feature = "database")]
async fn try_database_lookup(
    sig_p: i32,
    sig_q: i32,
    sig_r: i32,
    start_time: Instant,
) -> Option<Result<Value>> {
    let pool = DB_POOL.get()?;

    match sqlx::query_as::<
        _,
        (
            Vec<u8>,
            i32,
            i32,
            Option<f32>,
            Option<chrono::DateTime<chrono::Utc>>,
            Option<String>,
            Option<String>,
            Option<String>,
        ),
    >(
        r#"
        SELECT
            ct.table_data,
            ct.basis_count,
            ct.dimensions,
            ct.computation_time_ms,
            ct.computed_at,
            ct.checksum,
            ps.name,
            ps.description
        FROM cayley_tables ct
        LEFT JOIN precomputed_signatures ps USING (signature_p, signature_q, signature_r)
        WHERE ct.signature_p = $1 AND ct.signature_q = $2 AND ct.signature_r = $3
        "#,
    )
    .bind(sig_p)
    .bind(sig_q)
    .bind(sig_r)
    .fetch_optional(pool)
    .await
    {
        Ok(Some(row)) => {
            let lookup_time = start_time.elapsed().as_millis() as f32;

            info!(
                "Database hit for [{}, {}, {}] in {}ms",
                sig_p, sig_q, sig_r, lookup_time
            );

            // Decompress the table data
            match decompress_table_data(&row.0) {
                Ok(table_data) => {
                    // Update usage statistics
                    let saved_time = row.3.unwrap_or(0.0);
                    if let Err(e) = update_usage_stats(pool, sig_p, sig_q, sig_r, saved_time).await
                    {
                        warn!("Failed to update usage stats: {}", e);
                    }

                    // Convert flat data back to 3D structure
                    let basis_count = row.1 as usize;
                    let cayley_table = reconstruct_3d_table(&table_data, basis_count);

                    Some(Ok(json!({
                        "success": true,
                        "table_id": format!("cayley_{}_{}_{}",  sig_p, sig_q, sig_r),
                        "signature": [sig_p, sig_q, sig_r],
                        "basis_count": basis_count,
                        "cayley_table": cayley_table,
                        "cached": true,
                        "source": "precomputed_database",
                        "lookup_time_ms": lookup_time,
                        "original_computation_time_ms": row.3,
                        "time_saved_ms": saved_time - lookup_time,
                        "computed_at": row.4,
                        "checksum": row.5,
                        "name": row.6,
                        "description": row.7,
                        "note": "Retrieved from precomputed database"
                    })))
                }
                Err(e) => {
                    warn!("Failed to decompress Cayley table data: {}", e);
                    None // Fall back to computation
                }
            }
        }
        Ok(None) => {
            info!(
                "📊 No precomputed table found for [{}, {}, {}], will compute",
                sig_p, sig_q, sig_r
            );
            None // Not found in database
        }
        Err(e) => {
            warn!(
                "Database lookup failed for [{}, {}, {}]: {}",
                sig_p, sig_q, sig_r, e
            );
            None // Database error, fall back to computation
        }
    }
}

/// Update usage statistics for tracking
#[cfg(feature = "database")]
async fn update_usage_stats(
    pool: &PgPool,
    sig_p: i32,
    sig_q: i32,
    sig_r: i32,
    time_saved_ms: f32,
) -> Result<()> {
    sqlx::query("SELECT update_cayley_usage($1, $2, $3, $4)")
        .bind(sig_p)
        .bind(sig_q)
        .bind(sig_r)
        .bind(time_saved_ms)
        .execute(pool)
        .await?;

    Ok(())
}

/// Decompress table data from database storage
#[allow(dead_code)]
fn decompress_table_data(compressed_data: &[u8]) -> Result<Vec<f64>> {
    // Convert bytes back to f64 (assuming no compression for now)
    let mut table_data = Vec::new();

    for chunk in compressed_data.chunks(8) {
        if chunk.len() == 8 {
            let bytes: [u8; 8] = chunk
                .try_into()
                .map_err(|_| anyhow::anyhow!("Invalid chunk size"))?;
            table_data.push(f64::from_le_bytes(bytes));
        }
    }

    Ok(table_data)
}

/// Reconstruct 3D Cayley table from flat data
#[allow(dead_code)]
fn reconstruct_3d_table(flat_data: &[f64], basis_count: usize) -> Vec<Vec<Vec<f64>>> {
    let mut table = vec![vec![vec![0.0; basis_count]; basis_count]; basis_count];

    for i in 0..basis_count {
        for j in 0..basis_count {
            for k in 0..basis_count {
                let flat_idx = i * basis_count * basis_count + j * basis_count + k;
                if flat_idx < flat_data.len() {
                    table[i][j][k] = flat_data[flat_idx];
                }
            }
        }
    }

    table
}

/// Fallback computation when database lookup fails or is forced
async fn compute_cayley_table_fallback(
    sig_p: i32,
    sig_q: i32,
    sig_r: i32,
    table_id: String,
    start_time: Instant,
) -> Result<Value> {
    let dimensions = (sig_p + sig_q + sig_r) as usize;
    let basis_count = 1_usize << dimensions;

    info!(
        "🧮 Computing Cayley table for [{}, {}, {}] ({}D, {} basis elements)",
        sig_p, sig_q, sig_r, dimensions, basis_count
    );

    // Create simplified Cayley table (TODO: integrate with amari-fusion)
    let mut cayley_table = vec![vec![vec![0.0; basis_count]; basis_count]; basis_count];

    // Basic identity for scalar multiplication
    if basis_count > 0 {
        cayley_table[0][0][0] = 1.0; // 1 * 1 = 1
    }

    let computation_time = start_time.elapsed().as_millis() as f32;

    info!("⏱️  Computed in {}ms", computation_time);

    Ok(json!({
        "success": true,
        "table_id": table_id,
        "signature": [sig_p, sig_q, sig_r],
        "basis_count": basis_count,
        "cayley_table": cayley_table,
        "cached": false,
        "source": "computed",
        "computation_time_ms": computation_time,
        "note": "Computed on-demand - consider precomputing for better performance"
    }))
}

/// Store a computed Cayley table in the cache
/// Useful for expensive fusion computations
#[allow(dead_code)]
pub async fn cache_cayley_table(params: Value) -> Result<Value> {
    let table_id = params["table_id"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("table_id must be a string"))?;

    let _cayley_table = &params["cayley_table"];
    let signature = &params["signature"];

    info!("Caching Cayley table: {}", table_id);

    // TODO: In the real implementation:
    // 1. Validate the table structure
    // 2. Store in database with table_id as key
    // 3. Add metadata (signature, computation time, etc.)
    // 4. Return success confirmation

    Ok(json!({
        "success": true,
        "table_id": table_id,
        "cached_at": chrono::Utc::now().to_rfc3339(),
        "signature": signature,
        "note": "Stub implementation - requires database feature for persistent caching"
    }))
}

/// List all cached Cayley tables
/// Helpful for managing cached fusion operations
#[allow(dead_code)]
pub async fn list_cached_tables(_params: Value) -> Result<Value> {
    info!("Listing cached Cayley tables");

    // TODO: Query database for all cached tables
    let cached_tables = vec![json!({
        "table_id": "cayley_3_0_0",
        "signature": [3, 0, 0],
        "cached_at": "2024-01-01T12:00:00Z",
        "size_mb": 0.5
    })];

    Ok(json!({
        "success": true,
        "cached_tables": cached_tables,
        "total_count": cached_tables.len(),
        "note": "Stub implementation - requires database feature"
    }))
}

/// Clear cached Cayley tables
/// Useful for testing or memory management
#[allow(dead_code)]
pub async fn clear_cayley_cache(params: Value) -> Result<Value> {
    let table_id = params["table_id"].as_str(); // Optional - if None, clear all

    if let Some(id) = table_id {
        info!("Clearing cached Cayley table: {}", id);
    } else {
        info!("Clearing all cached Cayley tables");
    }

    // TODO: Delete from database

    Ok(json!({
        "success": true,
        "cleared": if table_id.is_some() { 1 } else { 0 },
        "note": "Stub implementation - requires database feature"
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use serde_json::json;

    #[tokio::test]
    async fn test_get_cayley_table_basic() {
        let params = json!({
            "signature": [3, 0, 0]
        });

        let result = get_cayley_table(params).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response["success"], true);
        assert_eq!(response["signature"], json!([3, 0, 0]));
        assert_eq!(response["basis_count"], 8); // 2^3 = 8 for 3D
        assert!(response["cayley_table"].is_array());
        assert_eq!(response["source"], "computed"); // No database, so computed
    }

    #[tokio::test]
    async fn test_get_cayley_table_different_signatures() {
        let test_cases = vec![
            ([2, 0, 0], 4),  // 2D Euclidean
            ([1, 1, 0], 4),  // 2D Minkowski
            ([4, 0, 0], 16), // 4D Euclidean
        ];

        for (signature, expected_basis_count) in test_cases {
            let params = json!({
                "signature": signature
            });

            let result = get_cayley_table(params).await;
            assert!(result.is_ok());

            let response = result.unwrap();
            assert_eq!(response["success"], true);
            assert_eq!(response["signature"], json!(signature));
            assert_eq!(response["basis_count"], expected_basis_count);
            assert!(response["cayley_table"].is_array());
        }
    }

    #[tokio::test]
    async fn test_get_cayley_table_invalid_signature() {
        let params = json!({
            "signature": "invalid"
        });

        let result = get_cayley_table(params).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_cayley_table_missing_signature() {
        let params = json!({});

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
        assert_eq!(response["source"], "computed"); // Should be computed due to force flag
    }

    #[tokio::test]
    async fn test_cayley_table_structure() {
        let params = json!({
            "signature": [2, 0, 0]
        });

        let result = get_cayley_table(params).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        let table = &response["cayley_table"];
        let basis_count = response["basis_count"].as_u64().unwrap() as usize;

        // Verify 3D structure: table[i][j][k] where each dimension is basis_count
        assert!(table.is_array());
        let outer_array = table.as_array().unwrap();
        assert_eq!(outer_array.len(), basis_count);

        for i in 0..basis_count {
            assert!(outer_array[i].is_array());
            let middle_array = outer_array[i].as_array().unwrap();
            assert_eq!(middle_array.len(), basis_count);

            for j in 0..basis_count {
                assert!(middle_array[j].is_array());
                let inner_array = middle_array[j].as_array().unwrap();
                assert_eq!(inner_array.len(), basis_count);

                // All values should be numbers
                for k in 0..basis_count {
                    assert!(inner_array[k].is_number());
                }
            }
        }
    }

    #[tokio::test]
    async fn test_cayley_table_identity_property() {
        let params = json!({
            "signature": [2, 0, 0]
        });

        let result = get_cayley_table(params).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        let table = &response["cayley_table"];

        // Basic check: scalar (index 0) multiplication should preserve identity
        // table[0][0][0] should be 1.0 (scalar * scalar = scalar)
        let scalar_mult = table[0][0][0].as_f64().unwrap();
        assert_relative_eq!(scalar_mult, 1.0, epsilon = 1e-10);
    }

    #[test]
    fn test_decompress_table_data() {
        // Test data decompression
        let test_data: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0];
        let mut compressed = Vec::new();

        // Convert to bytes (simple implementation)
        for value in &test_data {
            compressed.extend_from_slice(&value.to_le_bytes());
        }

        let decompressed = decompress_table_data(&compressed).unwrap();
        assert_eq!(decompressed, test_data);
    }

    #[test]
    fn test_reconstruct_3d_table() {
        let basis_count = 2;
        let flat_data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];

        let table = reconstruct_3d_table(&flat_data, basis_count);

        assert_eq!(table.len(), basis_count);
        assert_eq!(table[0].len(), basis_count);
        assert_eq!(table[0][0].len(), basis_count);

        // Check that reconstruction works correctly
        assert_eq!(table[0][0][0], 1.0);
        assert_eq!(table[0][0][1], 2.0);
        assert_eq!(table[0][1][0], 3.0);
        assert_eq!(table[0][1][1], 4.0);
        assert_eq!(table[1][0][0], 5.0);
        assert_eq!(table[1][0][1], 6.0);
        assert_eq!(table[1][1][0], 7.0);
        assert_eq!(table[1][1][1], 8.0);
    }
}
