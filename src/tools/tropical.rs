use anyhow::Result;
// use amari::*; // Using amari::* instead of prelude until we verify the API
use serde_json::{json, Value};
use tracing::info;

/// Tropical matrix multiplication (min-plus algebra)
pub async fn matrix_multiply(params: Value) -> Result<Value> {
    let matrix_a: Vec<Vec<f64>> = params["matrix_a"]
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("matrix_a must be an array"))?
        .iter()
        .map(|row| {
            row.as_array()
                .ok_or_else(|| anyhow::anyhow!("Each row must be an array"))?
                .iter()
                .map(|v| {
                    v.as_f64()
                        .ok_or_else(|| anyhow::anyhow!("Invalid matrix element"))
                })
                .collect::<Result<Vec<_>>>()
        })
        .collect::<Result<Vec<_>>>()?;

    let matrix_b: Vec<Vec<f64>> = params["matrix_b"]
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("matrix_b must be an array"))?
        .iter()
        .map(|row| {
            row.as_array()
                .ok_or_else(|| anyhow::anyhow!("Each row must be an array"))?
                .iter()
                .map(|v| {
                    v.as_f64()
                        .ok_or_else(|| anyhow::anyhow!("Invalid matrix element"))
                })
                .collect::<Result<Vec<_>>>()
        })
        .collect::<Result<Vec<_>>>()?;

    let rows_a = matrix_a.len();
    let cols_a = matrix_a.get(0).map(|r| r.len()).unwrap_or(0);
    let rows_b = matrix_b.len();
    let cols_b = matrix_b.get(0).map(|r| r.len()).unwrap_or(0);

    if cols_a != rows_b {
        return Err(anyhow::anyhow!(
            "Matrix dimensions incompatible: {}x{} * {}x{}",
            rows_a,
            cols_a,
            rows_b,
            cols_b
        ));
    }

    info!(
        "Tropical matrix multiplication: {}x{} * {}x{}",
        rows_a, cols_a, rows_b, cols_b
    );

    // Convert to TropicalMatrix (assuming this exists in amari-tropical)
    let mut result = vec![vec![f64::INFINITY; cols_b]; rows_a];

    // Tropical matrix multiplication: (A ⊗ B)ᵢⱼ = min_k(aᵢₖ + bₖⱼ)
    for i in 0..rows_a {
        for j in 0..cols_b {
            for k in 0..cols_a {
                let sum = matrix_a[i][k] + matrix_b[k][j];
                if sum < result[i][j] {
                    result[i][j] = sum;
                }
            }
        }
    }

    Ok(json!({
        "success": true,
        "result": result,
        "operation": "tropical_matrix_multiply",
        "dimensions": {
            "input_a": [rows_a, cols_a],
            "input_b": [rows_b, cols_b],
            "output": [rows_a, cols_b]
        }
    }))
}

/// Find shortest paths using tropical algebra
pub async fn shortest_path(params: Value) -> Result<Value> {
    let adjacency_matrix: Vec<Vec<f64>> = params["adjacency_matrix"]
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("adjacency_matrix must be an array"))?
        .iter()
        .map(|row| {
            row.as_array()
                .ok_or_else(|| anyhow::anyhow!("Each row must be an array"))?
                .iter()
                .map(|v| {
                    if v.is_null() {
                        Ok(f64::INFINITY)
                    } else {
                        v.as_f64()
                            .ok_or_else(|| anyhow::anyhow!("Invalid matrix element"))
                    }
                })
                .collect::<Result<Vec<_>>>()
        })
        .collect::<Result<Vec<_>>>()?;

    let source = params["source"]
        .as_u64()
        .ok_or_else(|| anyhow::anyhow!("source must be an integer"))? as usize;

    let target = params["target"].as_u64().map(|t| t as usize);

    let n = adjacency_matrix.len();
    if adjacency_matrix.iter().any(|row| row.len() != n) {
        return Err(anyhow::anyhow!("Adjacency matrix must be square"));
    }

    if source >= n {
        return Err(anyhow::anyhow!(
            "Source vertex {} out of range [0, {})",
            source,
            n
        ));
    }

    if let Some(t) = target {
        if t >= n {
            return Err(anyhow::anyhow!(
                "Target vertex {} out of range [0, {})",
                t,
                n
            ));
        }
    }

    info!(
        "Computing shortest paths from vertex {} in {}-vertex graph",
        source, n
    );

    // Floyd-Warshall algorithm using tropical algebra
    let mut dist = adjacency_matrix.clone();

    // Initialize diagonal to 0 (identity in tropical algebra)
    for i in 0..n {
        dist[i][i] = 0.0;
    }

    // Floyd-Warshall iterations
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                let new_dist = dist[i][k] + dist[k][j];
                if new_dist < dist[i][j] {
                    dist[i][j] = new_dist;
                }
            }
        }
    }

    // Extract distances from source
    let distances: Vec<f64> = dist[source].clone();

    let result = if let Some(target_vertex) = target {
        json!({
            "success": true,
            "source": source,
            "target": target_vertex,
            "distance": distances[target_vertex],
            "reachable": distances[target_vertex].is_finite()
        })
    } else {
        let reachable_vertices: Vec<usize> = distances
            .iter()
            .enumerate()
            .filter(|(_, &d)| d.is_finite())
            .map(|(i, _)| i)
            .collect();

        json!({
            "success": true,
            "source": source,
            "distances": distances,
            "reachable_vertices": reachable_vertices,
            "all_pairs_distances": dist
        })
    };

    Ok(result)
}
