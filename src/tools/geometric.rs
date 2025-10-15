use anyhow::Result;
use amari::*; // Using amari::* instead of prelude until we verify the API
use serde_json::{Value, json};
use tracing::info;

/// Create a multivector from coefficients
pub async fn create_multivector(params: Value) -> Result<Value> {
    let coefficients: Vec<f64> = params["coefficients"]
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("coefficients must be an array"))?
        .iter()
        .map(|v| v.as_f64().ok_or_else(|| anyhow::anyhow!("Invalid coefficient")))
        .collect::<Result<Vec<_>>>()?;

    let dimensions = params["dimensions"].as_u64().unwrap_or(3) as usize;
    let signature = if let Some(sig) = params["signature"].as_array() {
        vec![
            sig.get(0).and_then(|v| v.as_u64()).unwrap_or(dimensions as u64) as usize,
            sig.get(1).and_then(|v| v.as_u64()).unwrap_or(0) as usize,
            sig.get(2).and_then(|v| v.as_u64()).unwrap_or(0) as usize,
        ]
    } else {
        vec![dimensions, 0, 0]
    };

    info!("Creating multivector with {} coefficients in {}D space", coefficients.len(), dimensions);

    // For now, default to 3D Euclidean space
    let mv = match (signature[0], signature[1], signature[2]) {
        (3, 0, 0) => {
            if coefficients.len() != 8 {
                return Err(anyhow::anyhow!("3D Euclidean space requires 8 coefficients"));
            }
            let mv = Multivector::<3, 0, 0>::from_coefficients(coefficients);
            json!({
                "coefficients": mv.as_slice(),
                "scalar": mv.scalar_part(),
                "vector": [mv.get(1), mv.get(2), mv.get(3)],
                "bivector": [mv.get(4), mv.get(5), mv.get(6)],
                "trivector": mv.get(7),
                "magnitude": mv.magnitude(),
                "signature": signature
            })
        },
        (2, 0, 0) => {
            if coefficients.len() != 4 {
                return Err(anyhow::anyhow!("2D Euclidean space requires 4 coefficients"));
            }
            let mv = Multivector::<2, 0, 0>::from_coefficients(coefficients);
            json!({
                "coefficients": mv.as_slice(),
                "scalar": mv.scalar_part(),
                "vector": [mv.get(1), mv.get(2)],
                "bivector": mv.get(3),
                "magnitude": mv.magnitude(),
                "signature": signature
            })
        },
        (1, 3, 0) => {
            if coefficients.len() != 16 {
                return Err(anyhow::anyhow!("Minkowski spacetime requires 16 coefficients"));
            }
            let mv = Multivector::<1, 3, 0>::from_coefficients(coefficients);
            json!({
                "coefficients": mv.as_slice(),
                "scalar": mv.scalar_part(),
                "magnitude": mv.magnitude(),
                "signature": signature
            })
        },
        _ => {
            return Err(anyhow::anyhow!("Unsupported signature: {:?}", signature));
        }
    };

    Ok(json!({
        "success": true,
        "multivector": mv,
        "info": format!("Created multivector in {}D space with signature {:?}", dimensions, signature)
    }))
}

/// Compute geometric product of two multivectors
pub async fn geometric_product(params: Value) -> Result<Value> {
    let a_coeffs: Vec<f64> = params["a"]
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("Parameter 'a' must be an array"))?
        .iter()
        .map(|v| v.as_f64().ok_or_else(|| anyhow::anyhow!("Invalid coefficient in 'a'")))
        .collect::<Result<Vec<_>>>()?;

    let b_coeffs: Vec<f64> = params["b"]
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("Parameter 'b' must be an array"))?
        .iter()
        .map(|v| v.as_f64().ok_or_else(|| anyhow::anyhow!("Invalid coefficient in 'b'")))
        .collect::<Result<Vec<_>>>()?;

    if a_coeffs.len() != b_coeffs.len() {
        return Err(anyhow::anyhow!("Multivectors must have the same number of coefficients"));
    }

    let signature = if let Some(sig) = params["signature"].as_array() {
        vec![
            sig.get(0).and_then(|v| v.as_u64()).unwrap_or(3) as usize,
            sig.get(1).and_then(|v| v.as_u64()).unwrap_or(0) as usize,
            sig.get(2).and_then(|v| v.as_u64()).unwrap_or(0) as usize,
        ]
    } else {
        vec![3, 0, 0]
    };

    info!("Computing geometric product of multivectors with {} coefficients", a_coeffs.len());

    let result = match (signature[0], signature[1], signature[2]) {
        (3, 0, 0) => {
            let a = Multivector::<3, 0, 0>::from_coefficients(a_coeffs);
            let b = Multivector::<3, 0, 0>::from_coefficients(b_coeffs);
            let result = a * b;
            json!({
                "coefficients": result.as_slice(),
                "scalar": result.scalar_part(),
                "magnitude": result.magnitude()
            })
        },
        (2, 0, 0) => {
            let a = Multivector::<2, 0, 0>::from_coefficients(a_coeffs);
            let b = Multivector::<2, 0, 0>::from_coefficients(b_coeffs);
            let result = a * b;
            json!({
                "coefficients": result.as_slice(),
                "scalar": result.scalar_part(),
                "magnitude": result.magnitude()
            })
        },
        _ => {
            return Err(anyhow::anyhow!("Unsupported signature for geometric product: {:?}", signature));
        }
    };

    Ok(json!({
        "success": true,
        "result": result,
        "operation": "geometric_product"
    }))
}

/// Apply rotation using rotor
pub async fn rotor_rotation(params: Value) -> Result<Value> {
    let vector: Vec<f64> = params["vector"]
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("vector must be an array"))?
        .iter()
        .map(|v| v.as_f64().ok_or_else(|| anyhow::anyhow!("Invalid vector component")))
        .collect::<Result<Vec<_>>>()?;

    let axis: Vec<f64> = params["axis"]
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("axis must be an array"))?
        .iter()
        .map(|v| v.as_f64().ok_or_else(|| anyhow::anyhow!("Invalid axis component")))
        .collect::<Result<Vec<_>>>()?;

    let angle = params["angle"]
        .as_f64()
        .ok_or_else(|| anyhow::anyhow!("angle must be a number"))?;

    if vector.len() != 3 || axis.len() != 3 {
        return Err(anyhow::anyhow!("Vector and axis must be 3D"));
    }

    info!("Applying rotation: angle={:.3}°, axis=[{:.3}, {:.3}, {:.3}]",
          angle.to_degrees(), axis[0], axis[1], axis[2]);

    // Create vector as multivector
    let v = Multivector::<3, 0, 0>::from_coefficients(vec![0.0, vector[0], vector[1], vector[2], 0.0, 0.0, 0.0, 0.0]);

    // Create normalized axis
    let axis_mag = (axis[0] * axis[0] + axis[1] * axis[1] + axis[2] * axis[2]).sqrt();
    let norm_axis = [axis[0] / axis_mag, axis[1] / axis_mag, axis[2] / axis_mag];

    // Create rotor from axis and angle
    let half_angle = angle / 2.0;
    let cos_half = half_angle.cos();
    let sin_half = half_angle.sin();

    // Rotor = cos(θ/2) - sin(θ/2) * (axis as bivector)
    let rotor = Multivector::<3, 0, 0>::from_coefficients(vec![
        cos_half,                          // scalar
        0.0, 0.0, 0.0,                    // vector parts (0 for rotor)
        -sin_half * norm_axis[2],         // e12 (z rotation)
        sin_half * norm_axis[1],          // e13 (y rotation)
        -sin_half * norm_axis[0],         // e23 (x rotation)
        0.0                               // trivector
    ]);

    // Apply rotation: R * v * R†
    let rotor_conj = rotor.reverse();
    let rotated = rotor * v * rotor_conj;

    // Extract vector part
    let result_vector = vec![rotated.get(1), rotated.get(2), rotated.get(3)];

    Ok(json!({
        "success": true,
        "original_vector": vector,
        "rotated_vector": result_vector,
        "rotation_axis": axis,
        "rotation_angle_rad": angle,
        "rotation_angle_deg": angle.to_degrees(),
        "rotor_coefficients": rotor.as_slice()
    }))
}