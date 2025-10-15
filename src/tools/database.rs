#[cfg(feature = "database")]
use anyhow::Result;
#[cfg(feature = "database")]
use serde_json::{Value, json};
#[cfg(feature = "database")]
use tracing::info;

#[cfg(feature = "database")]
/// Save computation result to database
pub async fn save_computation(params: Value) -> Result<Value> {
    let name = params["name"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("name must be a string"))?;

    let computation_type = params["type"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("type must be a string"))?;

    let result = &params["result"];

    let metadata = params["metadata"].as_object();

    info!("Saving computation '{}' of type '{}'", name, computation_type);

    // TODO: Implement database storage
    // This would use the SqlxPgPool to save to PostgreSQL

    Ok(json!({
        "success": true,
        "name": name,
        "type": computation_type,
        "saved_at": chrono::Utc::now().to_rfc3339(),
        "note": "Database storage implementation pending"
    }))
}

#[cfg(feature = "database")]
/// Load computation result from database
pub async fn load_computation(params: Value) -> Result<Value> {
    let name = params["name"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("name must be a string"))?;

    info!("Loading computation '{}'", name);

    // TODO: Implement database loading
    // This would use the SqlxPgPool to load from PostgreSQL

    Ok(json!({
        "success": false,
        "name": name,
        "error": "Computation not found",
        "note": "Database loading implementation pending"
    }))
}

#[cfg(not(feature = "database"))]
pub async fn save_computation(_params: Value) -> Result<Value> {
    Err(anyhow::anyhow!("Database feature not enabled"))
}

#[cfg(not(feature = "database"))]
pub async fn load_computation(_params: Value) -> Result<Value> {
    Err(anyhow::anyhow!("Database feature not enabled"))
}