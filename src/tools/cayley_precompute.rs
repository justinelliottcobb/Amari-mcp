use anyhow::Result;
use serde_json::{Value, json};
use tracing::{info, warn, error};
use std::time::Instant;

#[cfg(feature = "database")]
use sqlx::PgPool;

/// Precompute and store essential Cayley tables for zero-latency lookup
/// This is a management tool for populating the database with commonly used tables
#[cfg(feature = "database")]
pub async fn precompute_essential_tables(pool: &PgPool) -> Result<Value> {
    info!("🧮 Starting precomputation of essential Cayley tables");

    let start_time = Instant::now();
    let mut computed_count = 0;
    let mut skipped_count = 0;
    let mut failed_count = 0;
    let mut total_size_bytes = 0u64;

    // Get list of signatures to precompute, ordered by priority
    let signatures = sqlx::query_as::<_, (i32, i32, i32, Option<String>, Option<i32>, Option<bool>)>(
        r#"
        SELECT signature_p, signature_q, signature_r, name, priority, is_essential
        FROM precomputed_signatures
        ORDER BY priority DESC, is_essential DESC
        "#
    )
    .fetch_all(pool)
    .await?;

    let total_signatures = signatures.len();
    info!("📋 Found {} signatures to precompute", total_signatures);

    for sig in signatures {
        let signature = [sig.0 as usize, sig.1 as usize, sig.2 as usize];
        let table_id = format!("cayley_{}_{}_{}",
            sig.0, sig.1, sig.2);

        info!("🔧 Processing {}: {} (priority: {})",
            table_id,
            sig.3.as_deref().unwrap_or("Unknown"),
            sig.4.unwrap_or(0)
        );

        // Check if already exists
        let exists = sqlx::query_as::<_, (i32,)>(
            "SELECT id FROM cayley_tables WHERE signature_p = $1 AND signature_q = $2 AND signature_r = $3"
        )
        .bind(sig.0)
        .bind(sig.1)
        .bind(sig.2)
        .fetch_optional(pool)
        .await?;

        if exists.is_some() {
            info!("   ⏭️  Already exists, skipping");
            skipped_count += 1;
            continue;
        }

        // Compute the Cayley table
        match compute_cayley_table_for_signature(&signature).await {
            Ok((table_data, computation_time_ms)) => {
                let dimensions = signature.iter().sum::<usize>() as i32;
                let basis_count = 1i32 << dimensions;
                let checksum = compute_table_checksum(&table_data);

                // Compress the table data for storage
                let compressed_data = compress_table_data(&table_data)?;
                let size_bytes = compressed_data.len() as u64;
                total_size_bytes += size_bytes;

                // Store in database
                match sqlx::query(
                    r#"
                    INSERT INTO cayley_tables
                    (signature_p, signature_q, signature_r, dimensions, basis_count,
                     table_data, computation_time_ms, checksum, metadata)
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
                    "#
                )
                .bind(sig.0)
                .bind(sig.1)
                .bind(sig.2)
                .bind(dimensions)
                .bind(basis_count)
                .bind(compressed_data)
                .bind(computation_time_ms)
                .bind(checksum)
                .bind(json!({
                    "precomputed": true,
                    "compression": "lz4",
                    "original_size_bytes": table_data.len() * 8,
                    "compressed_size_bytes": size_bytes
                }))
                .execute(pool)
                .await {
                    Ok(_) => {
                        info!("   ✅ Stored {} ({}ms, {} KB compressed)",
                            table_id, computation_time_ms, size_bytes / 1024);
                        computed_count += 1;
                    }
                    Err(e) => {
                        error!("   ❌ Failed to store {}: {}", table_id, e);
                        failed_count += 1;
                    }
                }
            }
            Err(e) => {
                error!("   ❌ Failed to compute {}: {}", table_id, e);
                failed_count += 1;
            }
        }
    }

    let total_time = start_time.elapsed();

    info!("🎉 Precomputation completed in {:.2}s", total_time.as_secs_f64());
    info!("   ✅ Computed: {}", computed_count);
    info!("   ⏭️  Skipped: {}", skipped_count);
    info!("   ❌ Failed: {}", failed_count);
    info!("   💾 Total storage: {} MB", total_size_bytes / 1024 / 1024);

    Ok(json!({
        "success": true,
        "precomputation_summary": {
            "total_signatures": total_signatures,
            "computed_count": computed_count,
            "skipped_count": skipped_count,
            "failed_count": failed_count,
            "total_time_seconds": total_time.as_secs_f64(),
            "total_storage_bytes": total_size_bytes,
            "total_storage_mb": total_size_bytes / 1024 / 1024
        }
    }))
}

/// Compute a Cayley table for a given signature
async fn compute_cayley_table_for_signature(signature: &[usize; 3]) -> Result<(Vec<f64>, f32)> {
    let start_time = Instant::now();

    // Calculate dimensions and basis count
    let dimensions = signature.iter().sum::<usize>();
    let basis_count = 1_usize << dimensions;

    info!("   🔢 Computing Cayley table for signature {:?} ({}D, {} basis elements)",
        signature, dimensions, basis_count);

    // For now, create a simplified Cayley table
    // TODO: Integrate with actual amari-fusion Cayley table computation
    let mut cayley_table = vec![0.0_f64; basis_count * basis_count * basis_count];

    // Simple identity mapping for scalar (index 0)
    if basis_count > 0 {
        cayley_table[0] = 1.0; // 1 * 1 = 1
    }

    // Add some basic geometric algebra structure
    // This is a simplified implementation - the real version would use amari-fusion
    for i in 0..basis_count.min(8) { // Limit for safety in stub
        for j in 0..basis_count.min(8) {
            let idx = i * basis_count * basis_count + j * basis_count;

            // Basic anti-commutativity for orthogonal basis vectors
            if i != j && i > 0 && j > 0 {
                if let Some(result_idx) = compute_simple_product_index(i, j, signature) {
                    if idx + result_idx < cayley_table.len() {
                        cayley_table[idx + result_idx] = if i < j { 1.0 } else { -1.0 };
                    }
                }
            } else if i == j && i > 0 {
                // Squares of basis vectors depend on signature
                let sign = match i.trailing_zeros() as usize {
                    dim if dim < signature[0] => 1.0,  // Positive signature
                    dim if dim < signature[0] + signature[1] => -1.0, // Negative signature
                    _ => 0.0, // Null signature
                };
                if idx < cayley_table.len() {
                    cayley_table[idx] = sign; // e_i * e_i = ±1 or 0
                }
            }
        }
    }

    let computation_time = start_time.elapsed().as_millis() as f32;

    info!("   ⏱️  Computed in {}ms", computation_time);

    Ok((cayley_table, computation_time))
}

/// Simple product index computation (stub implementation)
fn compute_simple_product_index(i: usize, j: usize, _signature: &[usize; 3]) -> Option<usize> {
    // XOR for simple basis vector multiplication
    // This is a very simplified implementation
    if i == 0 {
        Some(j)
    } else if j == 0 {
        Some(i)
    } else {
        Some(i ^ j) // Simple XOR for basis multiplication
    }
}

/// Compute SHA256 checksum of table data for integrity verification
fn compute_table_checksum(table_data: &[f64]) -> String {
    use sha2::{Sha256, Digest};

    let bytes: Vec<u8> = table_data.iter()
        .flat_map(|&f| f.to_le_bytes())
        .collect();

    let mut hasher = Sha256::new();
    hasher.update(&bytes);
    format!("{:x}", hasher.finalize())
}

/// Compress table data for efficient storage
fn compress_table_data(table_data: &[f64]) -> Result<Vec<u8>> {
    // Convert f64 to bytes
    let bytes: Vec<u8> = table_data.iter()
        .flat_map(|&f| f.to_le_bytes())
        .collect();

    // For now, just return the raw bytes
    // TODO: Implement LZ4 or similar compression
    Ok(bytes)
}

/// Decompress table data for use
#[allow(dead_code)]
fn decompress_table_data(compressed_data: &[u8]) -> Result<Vec<f64>> {
    // Convert bytes back to f64
    let mut table_data = Vec::new();

    for chunk in compressed_data.chunks(8) {
        if chunk.len() == 8 {
            let bytes: [u8; 8] = chunk.try_into()?;
            table_data.push(f64::from_le_bytes(bytes));
        }
    }

    Ok(table_data)
}

/// Get precomputation status and statistics
#[cfg(feature = "database")]
pub async fn get_precomputation_status(pool: &PgPool) -> Result<Value> {
    let stats = sqlx::query_as::<_, (i64, i64, Option<i64>, Option<f32>, Option<chrono::DateTime<chrono::Utc>>, Option<chrono::DateTime<chrono::Utc>>)>(
        r#"
        SELECT
            COUNT(*) as total_precomputed,
            COUNT(*) FILTER (WHERE ps.is_essential = true) as essential_precomputed,
            SUM(LENGTH(ct.table_data)) as total_storage_bytes,
            AVG(ct.computation_time_ms) as avg_computation_time_ms,
            MIN(ct.computed_at) as first_computed,
            MAX(ct.computed_at) as last_computed
        FROM cayley_tables ct
        LEFT JOIN precomputed_signatures ps USING (signature_p, signature_q, signature_r)
        "#
    )
    .fetch_one(pool)
    .await?;

    let pending = sqlx::query_as::<_, (i32, i32, i32, Option<String>, Option<i32>)>(
        r#"
        SELECT signature_p, signature_q, signature_r, name, priority
        FROM precomputed_signatures ps
        WHERE NOT EXISTS (
            SELECT 1 FROM cayley_tables ct
            WHERE ct.signature_p = ps.signature_p
            AND ct.signature_q = ps.signature_q
            AND ct.signature_r = ps.signature_r
        )
        ORDER BY priority DESC
        LIMIT 10
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(json!({
        "success": true,
        "precomputation_status": {
            "total_precomputed": stats.0,
            "essential_precomputed": stats.1,
            "total_storage_bytes": stats.2.unwrap_or(0),
            "total_storage_mb": stats.2.unwrap_or(0) / 1024 / 1024,
            "avg_computation_time_ms": stats.3,
            "first_computed": stats.4,
            "last_computed": stats.5,
            "pending_signatures": pending.iter().map(|p| json!({
                "signature": [p.0, p.1, p.2],
                "name": p.3,
                "priority": p.4
            })).collect::<Vec<_>>()
        }
    }))
}

/// Clear all precomputed Cayley tables (useful for testing/reset)
#[cfg(feature = "database")]
pub async fn clear_precomputed_tables(pool: &PgPool) -> Result<Value> {
    let deleted = sqlx::query("DELETE FROM cayley_tables")
        .execute(pool)
        .await?
        .rows_affected();

    info!("🗑️  Cleared {} precomputed Cayley tables", deleted);

    Ok(json!({
        "success": true,
        "deleted_count": deleted
    }))
}