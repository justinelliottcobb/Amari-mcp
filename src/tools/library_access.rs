use anyhow::Result;
use serde_json::{json, Value};
use std::fs;
use std::path::Path;
use tracing::info;

/// Browse Amari library documentation and API information
pub async fn browse_docs(params: Value) -> Result<Value> {
    let module = params["module"].as_str().unwrap_or("core");
    let query = params.get("query").and_then(|q| q.as_str());

    info!("📚 Browsing Amari documentation for module: {}", module);

    // Get the amari library path relative to this MCP
    let amari_path = "../amari";

    match module {
        "core" => browse_core_docs(amari_path, query).await,
        "tropical" => browse_tropical_docs(amari_path, query).await,
        "dual" => browse_dual_docs(amari_path, query).await,
        "fusion" => browse_fusion_docs(amari_path, query).await,
        "automata" => browse_automata_docs(amari_path, query).await,
        "network" => browse_network_docs(amari_path, query).await,
        "info_geom" => browse_info_geom_docs(amari_path, query).await,
        "enumerative" => browse_enumerative_docs(amari_path, query).await,
        "relativistic" => browse_relativistic_docs(amari_path, query).await,
        _ => Ok(json!({
            "success": false,
            "error": format!("Unknown module: {}", module),
            "available_modules": ["core", "tropical", "dual", "fusion", "automata", "network", "info_geom", "enumerative", "relativistic"]
        }))
    }
}

/// Analyze Amari source code structure and patterns
pub async fn analyze_code(params: Value) -> Result<Value> {
    let target = params["target"].as_str().unwrap_or("structure");
    let module = params.get("module").and_then(|m| m.as_str());

    info!("🔍 Analyzing Amari code: {}", target);

    let amari_path = "../amari";

    match target {
        "structure" => analyze_project_structure(amari_path).await,
        "exports" => analyze_module_exports(amari_path, module).await,
        "dependencies" => analyze_dependencies(amari_path, module).await,
        "examples" => find_examples(amari_path, module).await,
        "tests" => find_tests(amari_path, module).await,
        _ => Ok(json!({
            "success": false,
            "error": format!("Unknown analysis target: {}", target),
            "available_targets": ["structure", "exports", "dependencies", "examples", "tests"]
        }))
    }
}

/// Generate project scaffolding for Amari applications
pub async fn scaffold_project(params: Value) -> Result<Value> {
    let project_type = params["type"].as_str().unwrap_or("basic");
    let name = params["name"].as_str().unwrap_or("amari-app");
    let features = params.get("features").and_then(|f| f.as_array());

    info!("🏗️ Scaffolding Amari project: {} ({})", name, project_type);

    match project_type {
        "basic" => generate_basic_project(name, features).await,
        "library" => generate_library_project(name, features).await,
        "gpu" => generate_gpu_project(name, features).await,
        "web" => generate_web_project(name, features).await,
        _ => Ok(json!({
            "success": false,
            "error": format!("Unknown project type: {}", project_type),
            "available_types": ["basic", "library", "gpu", "web"]
        }))
    }
}

/// Generate code snippets and examples using Amari
pub async fn generate_code(params: Value) -> Result<Value> {
    let operation = params["operation"].as_str().unwrap_or("multivector");
    let context = params.get("context").and_then(|c| c.as_str()).unwrap_or("basic");

    info!("💻 Generating Amari code for: {}", operation);

    match operation {
        "multivector" => generate_multivector_example(context).await,
        "geometric_product" => generate_geometric_product_example(context).await,
        "tropical_algebra" => generate_tropical_example(context).await,
        "automatic_diff" => generate_autodiff_example(context).await,
        "network_analysis" => generate_network_example(context).await,
        "cellular_automata" => generate_automata_example(context).await,
        "information_geometry" => generate_info_geom_example(context).await,
        _ => Ok(json!({
            "success": false,
            "error": format!("Unknown operation: {}", operation),
            "available_operations": ["multivector", "geometric_product", "tropical_algebra", "automatic_diff", "network_analysis", "cellular_automata", "information_geometry"]
        }))
    }
}

/// Search for patterns and idioms in Amari codebase
pub async fn search_patterns(params: Value) -> Result<Value> {
    let pattern = params["pattern"].as_str().unwrap_or("");
    let scope = params.get("scope").and_then(|s| s.as_str()).unwrap_or("all");

    info!("🔎 Searching for pattern: {} in scope: {}", pattern, scope);

    let amari_path = "../amari";
    search_codebase_patterns(amari_path, pattern, scope).await
}

// Implementation functions

async fn browse_core_docs(amari_path: &str, query: Option<&str>) -> Result<Value> {
    let core_path = format!("{}/amari-core/src", amari_path);

    if !Path::new(&core_path).exists() {
        return Ok(json!({
            "success": false,
            "error": "Amari core module not found",
            "note": "Make sure the Amari library is accessible from this MCP server"
        }));
    }

    // Read the main lib.rs to understand the core API
    let lib_path = format!("{}/lib.rs", core_path);
    let content = fs::read_to_string(&lib_path)?;

    let exports = extract_public_items(&content);
    let documentation = extract_module_docs(&content);

    Ok(json!({
        "success": true,
        "module": "core",
        "path": core_path,
        "documentation": documentation,
        "public_exports": exports,
        "query": query,
        "main_types": ["Multivector", "Bivector", "Vector", "Scalar"],
        "key_features": ["Geometric algebra operations", "Clifford algebra implementation", "High-precision arithmetic"]
    }))
}

async fn browse_tropical_docs(amari_path: &str, query: Option<&str>) -> Result<Value> {
    let tropical_path = format!("{}/amari-tropical/src", amari_path);

    if !Path::new(&tropical_path).exists() {
        return Ok(json!({
            "success": false,
            "error": "Amari tropical module not found"
        }));
    }

    let lib_path = format!("{}/lib.rs", tropical_path);
    let content = fs::read_to_string(&lib_path)?;

    let exports = extract_public_items(&content);
    let documentation = extract_module_docs(&content);

    Ok(json!({
        "success": true,
        "module": "tropical",
        "path": tropical_path,
        "documentation": documentation,
        "public_exports": exports,
        "query": query,
        "main_types": ["TropicalNumber", "TropicalMatrix", "TropicalMultivector"],
        "key_features": ["Min-plus algebra", "Max-plus operations", "Shortest path algorithms"]
    }))
}

async fn browse_dual_docs(amari_path: &str, query: Option<&str>) -> Result<Value> {
    let dual_path = format!("{}/amari-dual/src", amari_path);

    if !Path::new(&dual_path).exists() {
        return Ok(json!({
            "success": false,
            "error": "Amari dual module not found"
        }));
    }

    let lib_path = format!("{}/lib.rs", dual_path);
    let content = fs::read_to_string(&lib_path)?;

    let exports = extract_public_items(&content);
    let documentation = extract_module_docs(&content);

    Ok(json!({
        "success": true,
        "module": "dual",
        "path": dual_path,
        "documentation": documentation,
        "public_exports": exports,
        "query": query,
        "main_types": ["DualNumber", "DualMultivector"],
        "key_features": ["Automatic differentiation", "Forward-mode AD", "Gradient computation"]
    }))
}

async fn browse_fusion_docs(amari_path: &str, query: Option<&str>) -> Result<Value> {
    let fusion_path = format!("{}/amari-fusion/src", amari_path);

    if !Path::new(&fusion_path).exists() {
        return Ok(json!({
            "success": false,
            "error": "Amari fusion module not found"
        }));
    }

    let lib_path = format!("{}/lib.rs", fusion_path);
    let content = fs::read_to_string(&lib_path)?;

    let exports = extract_public_items(&content);
    let documentation = extract_module_docs(&content);

    Ok(json!({
        "success": true,
        "module": "fusion",
        "path": fusion_path,
        "documentation": documentation,
        "public_exports": exports,
        "query": query,
        "main_types": ["TropicalDualClifford"],
        "key_features": ["Unified algebraic operations", "Cross-module integration", "Neural network optimization"]
    }))
}

async fn browse_automata_docs(amari_path: &str, query: Option<&str>) -> Result<Value> {
    let automata_path = format!("{}/amari-automata/src", amari_path);

    if !Path::new(&automata_path).exists() {
        return Ok(json!({
            "success": false,
            "error": "Amari automata module not found"
        }));
    }

    let lib_path = format!("{}/lib.rs", automata_path);
    let content = fs::read_to_string(&lib_path)?;

    let exports = extract_public_items(&content);
    let documentation = extract_module_docs(&content);

    Ok(json!({
        "success": true,
        "module": "automata",
        "path": automata_path,
        "documentation": documentation,
        "public_exports": exports,
        "query": query,
        "key_features": ["Cellular automata evolution", "Geometric CA rules", "Complex system modeling"]
    }))
}

async fn browse_network_docs(amari_path: &str, query: Option<&str>) -> Result<Value> {
    let network_path = format!("{}/amari-network/src", amari_path);

    if !Path::new(&network_path).exists() {
        return Ok(json!({
            "success": false,
            "error": "Amari network module not found"
        }));
    }

    let lib_path = format!("{}/lib.rs", network_path);
    let content = fs::read_to_string(&lib_path)?;

    let exports = extract_public_items(&content);
    let documentation = extract_module_docs(&content);

    Ok(json!({
        "success": true,
        "module": "network",
        "path": network_path,
        "documentation": documentation,
        "public_exports": exports,
        "query": query,
        "main_types": ["GeometricNetwork", "GeometricEdge", "NodeMetadata", "Community", "PropagationAnalysis"],
        "key_features": ["Geometric network analysis", "Community detection", "Propagation modeling"]
    }))
}

async fn browse_info_geom_docs(amari_path: &str, query: Option<&str>) -> Result<Value> {
    let info_geom_path = format!("{}/amari-info-geom/src", amari_path);

    if !Path::new(&info_geom_path).exists() {
        return Ok(json!({
            "success": false,
            "error": "Amari info-geom module not found"
        }));
    }

    let lib_path = format!("{}/lib.rs", info_geom_path);
    let content = fs::read_to_string(&lib_path)?;

    let exports = extract_public_items(&content);
    let documentation = extract_module_docs(&content);

    Ok(json!({
        "success": true,
        "module": "info_geom",
        "path": info_geom_path,
        "documentation": documentation,
        "public_exports": exports,
        "query": query,
        "main_types": ["FisherInformationMatrix", "DuallyFlatManifold", "SimpleAlphaConnection"],
        "key_features": ["Information geometry", "Fisher information", "Statistical manifolds"]
    }))
}

async fn browse_enumerative_docs(amari_path: &str, query: Option<&str>) -> Result<Value> {
    let enum_path = format!("{}/amari-enumerative/src", amari_path);

    if !Path::new(&enum_path).exists() {
        return Ok(json!({
            "success": false,
            "error": "Amari enumerative module not found"
        }));
    }

    let lib_path = format!("{}/lib.rs", enum_path);
    let content = fs::read_to_string(&lib_path)?;

    let exports = extract_public_items(&content);
    let documentation = extract_module_docs(&content);

    Ok(json!({
        "success": true,
        "module": "enumerative",
        "path": enum_path,
        "documentation": documentation,
        "public_exports": exports,
        "query": query,
        "key_features": ["Enumerative geometry", "Combinatorial structures", "Geometric counting"]
    }))
}

async fn browse_relativistic_docs(amari_path: &str, query: Option<&str>) -> Result<Value> {
    let rel_path = format!("{}/amari-relativistic/src", amari_path);

    if !Path::new(&rel_path).exists() {
        return Ok(json!({
            "success": false,
            "error": "Amari relativistic module not found"
        }));
    }

    let lib_path = format!("{}/lib.rs", rel_path);
    let content = fs::read_to_string(&lib_path)?;

    let exports = extract_public_items(&content);
    let documentation = extract_module_docs(&content);

    Ok(json!({
        "success": true,
        "module": "relativistic",
        "path": rel_path,
        "documentation": documentation,
        "public_exports": exports,
        "query": query,
        "key_features": ["Relativistic physics", "Spacetime geometry", "Physics simulations"]
    }))
}

async fn analyze_project_structure(amari_path: &str) -> Result<Value> {
    if !Path::new(amari_path).exists() {
        return Ok(json!({
            "success": false,
            "error": "Amari library not found at expected path"
        }));
    }

    // Read the workspace Cargo.toml
    let cargo_path = format!("{}/Cargo.toml", amari_path);
    let cargo_content = fs::read_to_string(&cargo_path)?;

    // Extract workspace members
    let members = extract_workspace_members(&cargo_content);

    Ok(json!({
        "success": true,
        "analysis_type": "structure",
        "workspace_path": amari_path,
        "modules": members,
        "structure": {
            "core": "amari-core - Fundamental geometric algebra operations",
            "tropical": "amari-tropical - Min-plus/max-plus algebra",
            "dual": "amari-dual - Automatic differentiation",
            "fusion": "amari-fusion - Unified algebraic operations",
            "automata": "amari-automata - Cellular automata evolution",
            "network": "amari-network - Geometric network analysis",
            "info_geom": "amari-info-geom - Information geometry",
            "enumerative": "amari-enumerative - Enumerative geometry",
            "relativistic": "amari-relativistic - Relativistic physics",
            "gpu": "amari-gpu - GPU acceleration (optional)"
        }
    }))
}

async fn analyze_module_exports(amari_path: &str, module: Option<&str>) -> Result<Value> {
    let module = module.unwrap_or("all");

    if module == "all" {
        // Analyze the main lib.rs exports
        let lib_path = format!("{}/src/lib.rs", amari_path);
        let content = fs::read_to_string(&lib_path)?;

        let exports = extract_public_items(&content);
        let reexports = extract_reexports(&content);

        Ok(json!({
            "success": true,
            "analysis_type": "exports",
            "module": "main",
            "public_exports": exports,
            "reexports": reexports,
            "note": "These are the main exports available when using 'use amari::*;'"
        }))
    } else {
        // Analyze specific module
        let module_path = format!("{}/amari-{}/src/lib.rs", amari_path, module);

        if !Path::new(&module_path).exists() {
            return Ok(json!({
                "success": false,
                "error": format!("Module '{}' not found", module)
            }));
        }

        let content = fs::read_to_string(&module_path)?;
        let exports = extract_public_items(&content);

        Ok(json!({
            "success": true,
            "analysis_type": "exports",
            "module": module,
            "path": module_path,
            "public_exports": exports
        }))
    }
}

async fn analyze_dependencies(amari_path: &str, module: Option<&str>) -> Result<Value> {
    let cargo_path = if let Some(m) = module {
        format!("{}/amari-{}/Cargo.toml", amari_path, m)
    } else {
        format!("{}/Cargo.toml", amari_path)
    };

    if !Path::new(&cargo_path).exists() {
        return Ok(json!({
            "success": false,
            "error": format!("Cargo.toml not found for module: {:?}", module)
        }));
    }

    let content = fs::read_to_string(&cargo_path)?;
    let dependencies = extract_dependencies(&content);

    Ok(json!({
        "success": true,
        "analysis_type": "dependencies",
        "module": module.unwrap_or("main"),
        "dependencies": dependencies
    }))
}

async fn find_examples(amari_path: &str, module: Option<&str>) -> Result<Value> {
    let examples_path = if let Some(m) = module {
        format!("{}/amari-{}/examples", amari_path, m)
    } else {
        format!("{}/examples", amari_path)
    };

    let mut examples = Vec::new();

    if Path::new(&examples_path).exists() {
        if let Ok(entries) = fs::read_dir(&examples_path) {
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    if name.ends_with(".rs") {
                        examples.push(name.to_string());
                    }
                }
            }
        }
    }

    Ok(json!({
        "success": true,
        "analysis_type": "examples",
        "module": module.unwrap_or("main"),
        "examples_path": examples_path,
        "examples": examples
    }))
}

async fn find_tests(amari_path: &str, module: Option<&str>) -> Result<Value> {
    let tests_path = if let Some(m) = module {
        format!("{}/amari-{}/tests", amari_path, m)
    } else {
        format!("{}/tests", amari_path)
    };

    let mut tests = Vec::new();

    if Path::new(&tests_path).exists() {
        if let Ok(entries) = fs::read_dir(&tests_path) {
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    if name.ends_with(".rs") {
                        tests.push(name.to_string());
                    }
                }
            }
        }
    }

    Ok(json!({
        "success": true,
        "analysis_type": "tests",
        "module": module.unwrap_or("main"),
        "tests_path": tests_path,
        "tests": tests
    }))
}

async fn generate_basic_project(name: &str, features: Option<&Vec<Value>>) -> Result<Value> {
    let features_list = features
        .map(|f| f.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>())
        .unwrap_or_default();

    let cargo_toml = format!(r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
amari = "0.9"
tokio = {{ version = "1.0", features = ["full"] }}
anyhow = "1.0"
{}

[[bin]]
name = "main"
path = "src/main.rs"
"#, name, generate_feature_deps(&features_list));

    let main_rs = format!(r#"use amari::{{core::Multivector, AmariResult}};

#[tokio::main]
async fn main() -> AmariResult<()> {{
    println!("🧮 {} - Powered by Amari", "{}");

    // Create a simple multivector in 3D space
    let mv = Multivector::new([1.0, 2.0, 3.0, 4.0], [3, 0, 0]);
    println!("Created multivector: {{:?}}", mv);

    // Add your Amari-based logic here

    Ok(())
}}
"#, name, name);

    Ok(json!({
        "success": true,
        "project_type": "basic",
        "name": name,
        "features": features_list,
        "files": {
            "Cargo.toml": cargo_toml,
            "src/main.rs": main_rs
        },
        "next_steps": [
            "cargo init to create the project structure",
            "Copy the generated Cargo.toml",
            "Copy the generated src/main.rs",
            "Run 'cargo run' to test the basic setup"
        ]
    }))
}

async fn generate_library_project(name: &str, features: Option<&Vec<Value>>) -> Result<Value> {
    let features_list = features
        .map(|f| f.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>())
        .unwrap_or_default();

    let cargo_toml = format!(r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"
description = "A library built with Amari mathematical computing"

[dependencies]
amari = "0.9"
thiserror = "1.0"
{}

[dev-dependencies]
tokio = {{ version = "1.0", features = ["full"] }}
approx = "0.5"

[lib]
name = "{}"
path = "src/lib.rs"
"#, name, generate_feature_deps(&features_list), name.replace('-', "_"));

    let lib_rs = format!(r#"//! {} - A library built with Amari
//!
//! This library provides mathematical computing capabilities
//! using the Amari ecosystem.

use amari::{{AmariError, AmariResult}};
use thiserror::Error;

/// Error type for {}
#[derive(Error, Debug)]
pub enum {}Error {{
    #[error("Amari computation error")]
    Amari(#[from] AmariError),

    #[error("Invalid input: {{0}}")]
    InvalidInput(String),
}}

/// Result type for {} operations
pub type {}Result<T> = Result<T, {}Error>;

/// Main library interface
pub struct {}Library {{
    // Add your library state here
}}

impl {}Library {{
    /// Create a new instance
    pub fn new() -> {}Result<Self> {{
        Ok(Self {{}})
    }}

    /// Example method using Amari
    pub fn compute_example(&self) -> {}Result<f64> {{
        // Add your Amari-based computation here
        Ok(42.0)
    }}
}}

impl Default for {}Library {{
    fn default() -> Self {{
        Self::new().expect("Failed to create default library instance")
    }}
}}

#[cfg(test)]
mod tests {{
    use super::*;

    #[test]
    fn test_library_creation() {{
        let lib = {}Library::new();
        assert!(lib.is_ok());
    }}

    #[test]
    fn test_example_computation() {{
        let lib = {}Library::new().unwrap();
        let result = lib.compute_example();
        assert!(result.is_ok());
    }}
}}
"#,
        name,
        name,
        name.to_uppercase().replace('-', "_"),
        name,
        name.replace('-', "_"),
        name.to_uppercase().replace('-', "_"),
        name.replace('-', "_"),
        name.replace('-', "_"),
        name.replace('-', "_"),
        name.replace('-', "_"),
        name.replace('-', "_"),
        name.replace('-', "_"),
        name.replace('-', "_")
    );

    Ok(json!({
        "success": true,
        "project_type": "library",
        "name": name,
        "features": features_list,
        "files": {
            "Cargo.toml": cargo_toml,
            "src/lib.rs": lib_rs
        },
        "next_steps": [
            "cargo init --lib to create the library structure",
            "Copy the generated Cargo.toml",
            "Copy the generated src/lib.rs",
            "Run 'cargo test' to verify the setup",
            "Start implementing your library logic"
        ]
    }))
}

async fn generate_gpu_project(name: &str, features: Option<&Vec<Value>>) -> Result<Value> {
    let mut features_list = features
        .map(|f| f.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>())
        .unwrap_or_default();

    // Ensure GPU feature is included
    if !features_list.contains(&"gpu") {
        features_list.push("gpu");
    }

    let cargo_toml = format!(r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
amari = {{ version = "0.9", features = ["gpu"] }}
tokio = {{ version = "1.0", features = ["full"] }}
anyhow = "1.0"
{}

[[bin]]
name = "main"
path = "src/main.rs"
"#, name, generate_feature_deps(&features_list));

    let main_rs = format!(r#"use amari::{{AmariResult}};

#[tokio::main]
async fn main() -> AmariResult<()> {{
    println!("🚀 {} - GPU-Accelerated Amari Computing", "{}");

    // Initialize GPU context if available
    #[cfg(feature = "gpu")]
    {{
        println!("GPU acceleration enabled");
        // Add GPU-specific initialization here
    }}

    #[cfg(not(feature = "gpu"))]
    {{
        println!("Running in CPU mode");
    }}

    // Add your GPU-accelerated Amari logic here

    Ok(())
}}
"#, name, name);

    Ok(json!({
        "success": true,
        "project_type": "gpu",
        "name": name,
        "features": features_list,
        "files": {
            "Cargo.toml": cargo_toml,
            "src/main.rs": main_rs
        },
        "gpu_requirements": [
            "CUDA or ROCm drivers installed",
            "Compatible GPU hardware",
            "Build with: cargo build --features gpu"
        ],
        "next_steps": [
            "Ensure GPU drivers are installed",
            "cargo init to create the project structure",
            "Copy the generated files",
            "Build with: cargo build --features gpu",
            "Run with: cargo run --features gpu"
        ]
    }))
}

async fn generate_web_project(name: &str, features: Option<&Vec<Value>>) -> Result<Value> {
    let features_list = features
        .map(|f| f.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>())
        .unwrap_or_default();

    let cargo_toml = format!(r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
amari = "0.9"
wasm-bindgen = "0.2"
js-sys = "0.3"
web-sys = "0.3"
console_error_panic_hook = "0.1"
wee_alloc = "0.4"
{}

[dependencies.web-sys]
version = "0.3"
features = [
  "console",
]
"#, name, generate_feature_deps(&features_list));

    let lib_rs = format!(r#"use wasm_bindgen::prelude::*;
use amari::{{core::Multivector, AmariResult}};

// Import the `console.log` function from the browser
#[wasm_bindgen]
extern "C" {{
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}}

// Define a macro for easier console logging
macro_rules! console_log {{
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}}

#[wasm_bindgen]
pub struct AmariWeb {{
    // Web-specific state
}}

#[wasm_bindgen]
impl AmariWeb {{
    #[wasm_bindgen(constructor)]
    pub fn new() -> AmariWeb {{
        console_error_panic_hook::set_once();
        console_log!("🌐 {} initialized with Amari", "{}");

        AmariWeb {{}}
    }}

    #[wasm_bindgen]
    pub fn compute_multivector(&self, coeffs: &[f64]) -> Vec<f64> {{
        // Example: Create and manipulate a multivector
        let mv = Multivector::new(coeffs.to_vec(), [3, 0, 0]);
        console_log!("Created multivector with {{}} coefficients", coeffs.len());

        // Return the coefficients (this is just an example)
        coeffs.to_vec()
    }}

    #[wasm_bindgen]
    pub fn version(&self) -> String {{
        format!("{} v0.1.0 (Amari)", "{}")
    }}
}}

#[wasm_bindgen(start)]
pub fn main() {{
    console_log!("🧮 {} WebAssembly module loaded", "{}");
}}
"#, name, name, name, name, name, name);

    let index_html = format!(r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>{}</title>
    <style>
        body {{
            font-family: Arial, sans-serif;
            margin: 40px;
            background: #f5f5f5;
        }}
        .container {{
            max-width: 800px;
            margin: 0 auto;
            background: white;
            padding: 20px;
            border-radius: 8px;
            box-shadow: 0 2px 10px rgba(0,0,0,0.1);
        }}
        button {{
            background: #007cba;
            color: white;
            border: none;
            padding: 10px 20px;
            border-radius: 4px;
            cursor: pointer;
            margin: 5px;
        }}
        button:hover {{
            background: #005a87;
        }}
        #output {{
            background: #f8f8f8;
            border: 1px solid #ddd;
            padding: 10px;
            margin-top: 20px;
            border-radius: 4px;
            white-space: pre-wrap;
        }}
    </style>
</head>
<body>
    <div class="container">
        <h1>🧮 {}</h1>
        <p>WebAssembly application powered by Amari mathematical computing.</p>

        <button id="test-multivector">Test Multivector</button>
        <button id="get-version">Get Version</button>

        <div id="output"></div>
    </div>

    <script type="module">
        import init, {{ AmariWeb }} from './pkg/{}.js';

        async function run() {{
            await init();

            const amari = new AmariWeb();
            const output = document.getElementById('output');

            document.getElementById('test-multivector').addEventListener('click', () => {{
                const result = amari.compute_multivector([1.0, 2.0, 3.0, 4.0]);
                output.textContent += `Multivector computation result: ${{result}}\\n`;
            }});

            document.getElementById('get-version').addEventListener('click', () => {{
                const version = amari.version();
                output.textContent += `Version: ${{version}}\\n`;
            }});

            output.textContent = 'Ready! Click buttons to test Amari functionality.\\n';
        }}

        run();
    </script>
</body>
</html>
"#, name, name, name.replace('-', "_"));

    Ok(json!({
        "success": true,
        "project_type": "web",
        "name": name,
        "features": features_list,
        "files": {
            "Cargo.toml": cargo_toml,
            "src/lib.rs": lib_rs,
            "index.html": index_html
        },
        "build_instructions": [
            "Install wasm-pack: curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh",
            "Build WASM: wasm-pack build --target web",
            "Serve files: python -m http.server 8000",
            "Open http://localhost:8000 in browser"
        ],
        "next_steps": [
            "cargo init --lib to create the library structure",
            "Copy the generated files",
            "Follow build instructions to compile to WebAssembly",
            "Test in browser"
        ]
    }))
}

async fn generate_multivector_example(context: &str) -> Result<Value> {
    let code = match context {
        "basic" => r#"use amari::core::{Multivector, Signature};

fn basic_multivector_example() {
    // Create a 3D geometric algebra (3,0,0)
    let signature = Signature::new(3, 0, 0);

    // Create multivectors
    let mv1 = Multivector::new(vec![1.0, 2.0, 3.0, 4.0], signature);
    let mv2 = Multivector::new(vec![2.0, 1.0, 0.0, 1.0], signature);

    println!("MV1: {:?}", mv1);
    println!("MV2: {:?}", mv2);

    // Basic operations will be available through the core API
    // Check Amari documentation for specific operation methods
}"#,
        "advanced" => r#"use amari::core::{Multivector, Signature};

fn advanced_multivector_example() {
    // Create a spacetime algebra (1,3,0) for relativistic computations
    let spacetime = Signature::new(1, 3, 0);

    // Create a multivector representing a spacetime event
    let event = Multivector::new(
        vec![1.0, 0.5, 0.3, 0.2, 0.0, 0.0, 0.0, 0.0], // 8 coefficients for 4D
        spacetime
    );

    // Create a rotation/boost multivector
    let rotor = Multivector::new(
        vec![0.8, 0.0, 0.0, 0.0, 0.6, 0.0, 0.0, 0.0],
        spacetime
    );

    println!("Spacetime event: {:?}", event);
    println!("Rotor: {:?}", rotor);

    // Advanced operations would use geometric product and other GA operations
    // See Amari core documentation for specific methods
}"#,
        "physics" => r#"use amari::core::{Multivector, Signature};

fn physics_multivector_example() {
    // Create 3D Euclidean space (3,0,0)
    let space3d = Signature::new(3, 0, 0);

    // Represent a vector in 3D space
    let position = Multivector::new(
        vec![0.0, 1.0, 2.0, 3.0, 0.0, 0.0, 0.0, 0.0], // scalar + 3 vectors
        space3d
    );

    // Represent angular momentum as a bivector
    let angular_momentum = Multivector::new(
        vec![0.0, 0.0, 0.0, 0.0, 1.0, 0.5, 0.3, 0.0], // bivector components
        space3d
    );

    println!("Position: {:?}", position);
    println!("Angular momentum: {:?}", angular_momentum);

    // Physics computations would use geometric product for
    // rotations, electromagnetic field calculations, etc.
}"#,
        _ => r#"use amari::core::{Multivector, Signature};

fn default_multivector_example() {
    // Simple 2D example
    let signature = Signature::new(2, 0, 0);
    let mv = Multivector::new(vec![1.0, 1.0, 1.0, 1.0], signature);
    println!("2D Multivector: {:?}", mv);
}"#
    };

    Ok(json!({
        "success": true,
        "operation": "multivector",
        "context": context,
        "code": code,
        "explanation": "Creates and manipulates multivectors in geometric algebra",
        "key_concepts": ["Geometric algebra", "Clifford algebra", "Multivectors", "Signatures"]
    }))
}

async fn generate_geometric_product_example(context: &str) -> Result<Value> {
    let code = match context {
        "basic" => r#"use amari::core::{Multivector, Signature};

fn geometric_product_example() {
    let sig = Signature::new(3, 0, 0);

    // Create two vectors
    let v1 = Multivector::new(vec![0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], sig);
    let v2 = Multivector::new(vec![0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0], sig);

    // Geometric product combines dot and wedge products
    // v1 * v2 = v1 · v2 + v1 ∧ v2
    // Check Amari core documentation for the specific method name

    println!("Vector 1: {:?}", v1);
    println!("Vector 2: {:?}", v2);
    // Product computation would be: let product = v1.geometric_product(&v2);
}"#,
        "rotation" => r#"use amari::core::{Multivector, Signature};

fn rotation_with_geometric_product() {
    let sig = Signature::new(3, 0, 0);

    // Create a rotor (unit quaternion equivalent in GA)
    let rotor = Multivector::new(
        vec![0.8, 0.0, 0.0, 0.0, 0.6, 0.0, 0.0, 0.0], // cos(θ/2) + sin(θ/2)B
        sig
    );

    // Vector to rotate
    let vector = Multivector::new(
        vec![0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], // x-axis vector
        sig
    );

    // Rotation: rotated = rotor * vector * rotor_reverse
    // This requires geometric product operations
    println!("Original vector: {:?}", vector);
    println!("Rotor: {:?}", rotor);
}"#,
        _ => r#"use amari::core::{Multivector, Signature};

fn simple_geometric_product() {
    let sig = Signature::new(2, 0, 0);

    let mv1 = Multivector::new(vec![1.0, 1.0, 1.0, 0.0], sig);
    let mv2 = Multivector::new(vec![1.0, 0.0, 1.0, 1.0], sig);

    // Geometric product computation
    println!("MV1: {:?}", mv1);
    println!("MV2: {:?}", mv2);
}"#
    };

    Ok(json!({
        "success": true,
        "operation": "geometric_product",
        "context": context,
        "code": code,
        "explanation": "Computes geometric products in Clifford algebra",
        "key_concepts": ["Geometric product", "Clifford algebra", "Rotations", "Geometric transformations"]
    }))
}

async fn generate_tropical_example(context: &str) -> Result<Value> {
    let code = match context {
        "basic" => r#"use amari::tropical::{TropicalNumber, TropicalMatrix};

fn basic_tropical_algebra() {
    // Tropical numbers use min-plus or max-plus algebra
    let a = TropicalNumber::new(5.0);
    let b = TropicalNumber::new(3.0);

    // Tropical addition is min operation
    let sum = a.add(&b);  // min(5, 3) = 3

    // Tropical multiplication is regular addition
    let product = a.multiply(&b);  // 5 + 3 = 8

    println!("a: {:?}", a);
    println!("b: {:?}", b);
    println!("a ⊕ b: {:?}", sum);
    println!("a ⊙ b: {:?}", product);
}"#,
        "matrix" => r#"use amari::tropical::TropicalMatrix;

fn tropical_matrix_operations() {
    // Create tropical matrices for shortest path problems
    let matrix_a = TropicalMatrix::new(vec![
        vec![0.0, 2.0, f64::INFINITY],
        vec![1.0, 0.0, 3.0],
        vec![f64::INFINITY, 1.0, 0.0],
    ]);

    let matrix_b = TropicalMatrix::new(vec![
        vec![0.0, 1.0, 4.0],
        vec![2.0, 0.0, 1.0],
        vec![1.0, 3.0, 0.0],
    ]);

    // Tropical matrix multiplication finds shortest paths
    // let result = matrix_a.multiply(&matrix_b);

    println!("Matrix A: {:?}", matrix_a);
    println!("Matrix B: {:?}", matrix_b);
}"#,
        "shortest_path" => r#"use amari::tropical::TropicalMatrix;

fn shortest_path_example() {
    // Graph represented as adjacency matrix with edge weights
    let graph = TropicalMatrix::new(vec![
        vec![0.0,     2.0,     f64::INFINITY, 4.0],
        vec![f64::INFINITY, 0.0, 3.0,     1.0],
        vec![1.0,     f64::INFINITY, 0.0, 5.0],
        vec![f64::INFINITY, f64::INFINITY, 2.0, 0.0],
    ]);

    // Powers of the matrix give shortest paths of different lengths
    // let paths_length_2 = graph.power(2);
    // let all_shortest_paths = graph.closure(); // Floyd-Warshall equivalent

    println!("Graph adjacency matrix: {:?}", graph);
    println!("Use tropical algebra to find shortest paths between all node pairs");
}"#,
        _ => r#"use amari::tropical::TropicalNumber;

fn tropical_basics() {
    let x = TropicalNumber::new(2.0);
    let y = TropicalNumber::new(5.0);

    println!("Tropical addition (min): {:?}", x.add(&y));
    println!("Tropical multiplication (+): {:?}", x.multiply(&y));
}"#
    };

    Ok(json!({
        "success": true,
        "operation": "tropical_algebra",
        "context": context,
        "code": code,
        "explanation": "Min-plus and max-plus algebra operations",
        "key_concepts": ["Tropical algebra", "Min-plus semiring", "Shortest paths", "Graph algorithms"]
    }))
}

async fn generate_autodiff_example(context: &str) -> Result<Value> {
    let code = match context {
        "basic" => r#"use amari::dual::{DualNumber, DualMultivector};

fn basic_automatic_differentiation() {
    // Dual numbers for automatic differentiation
    let x = DualNumber::new(3.0, 1.0); // value=3, derivative=1
    let y = DualNumber::new(2.0, 0.0); // value=2, derivative=0

    // Compute f(x,y) = x² + 2xy + y²
    let x_squared = x.multiply(&x);
    let two_xy = x.multiply(&y).multiply(&DualNumber::new(2.0, 0.0));
    let y_squared = y.multiply(&y);

    let result = x_squared.add(&two_xy).add(&y_squared);

    println!("f(3,2) = {:?}", result.value());
    println!("∂f/∂x at (3,2) = {:?}", result.derivative());
}"#,
        "multivector" => r#"use amari::dual::DualMultivector;
use amari::core::Signature;

fn dual_multivector_example() {
    let sig = Signature::new(3, 0, 0);

    // Dual multivector for computing derivatives of geometric operations
    let mv = DualMultivector::new(
        vec![1.0, 2.0, 3.0, 4.0], // primal coefficients
        vec![0.0, 1.0, 0.0, 0.0], // dual coefficients (derivative direction)
        sig
    );

    // Geometric operations on dual multivectors automatically compute derivatives
    // let squared = mv.geometric_product(&mv);

    println!("Dual multivector: {:?}", mv);
    println!("Use for differentiating geometric algebra operations");
}"#,
        "optimization" => r#"use amari::dual::DualNumber;

fn gradient_descent_example() {
    // Use automatic differentiation for optimization
    fn objective_function(x: DualNumber, y: DualNumber) -> DualNumber {
        // f(x,y) = (x-1)² + (y-2)² + 3
        let x_term = x.subtract(&DualNumber::new(1.0, 0.0)).square();
        let y_term = y.subtract(&DualNumber::new(2.0, 0.0)).square();
        x_term.add(&y_term).add(&DualNumber::new(3.0, 0.0))
    }

    let mut x = 0.0;
    let mut y = 0.0;
    let learning_rate = 0.1;

    for iteration in 0..10 {
        // Compute gradient with respect to x
        let x_dual = DualNumber::new(x, 1.0); // derivative w.r.t. x
        let y_dual = DualNumber::new(y, 0.0);
        let grad_x = objective_function(x_dual, y_dual).derivative();

        // Compute gradient with respect to y
        let x_dual = DualNumber::new(x, 0.0);
        let y_dual = DualNumber::new(y, 1.0); // derivative w.r.t. y
        let grad_y = objective_function(x_dual, y_dual).derivative();

        // Update parameters
        x -= learning_rate * grad_x;
        y -= learning_rate * grad_y;

        println!("Iteration {}: x={:.3}, y={:.3}", iteration, x, y);
    }
}"#,
        _ => r#"use amari::dual::DualNumber;

fn simple_autodiff() {
    let x = DualNumber::new(2.0, 1.0);
    let result = x.square(); // f(x) = x²

    println!("f(2) = {:?}", result.value());       // 4
    println!("f'(2) = {:?}", result.derivative()); // 4 (derivative of x² is 2x)
}"#
    };

    Ok(json!({
        "success": true,
        "operation": "automatic_diff",
        "context": context,
        "code": code,
        "explanation": "Forward-mode automatic differentiation using dual numbers",
        "key_concepts": ["Automatic differentiation", "Dual numbers", "Gradient computation", "Optimization"]
    }))
}

async fn generate_network_example(context: &str) -> Result<Value> {
    let code = match context {
        "basic" => r#"use amari::network::{GeometricNetwork, GeometricEdge, NodeMetadata};

fn basic_network_analysis() {
    // Create a geometric network
    let mut network = GeometricNetwork::new();

    // Add nodes with geometric properties
    let node1 = network.add_node(NodeMetadata::new(vec![1.0, 0.0, 0.0])); // 3D position
    let node2 = network.add_node(NodeMetadata::new(vec![0.0, 1.0, 0.0]));
    let node3 = network.add_node(NodeMetadata::new(vec![0.0, 0.0, 1.0]));

    // Add edges with geometric weights
    network.add_edge(GeometricEdge::new(node1, node2, 1.0));
    network.add_edge(GeometricEdge::new(node2, node3, 1.5));
    network.add_edge(GeometricEdge::new(node3, node1, 2.0));

    println!("Network created with {} nodes", network.node_count());
    println!("Network has {} edges", network.edge_count());
}"#,
        "community" => r#"use amari::network::{GeometricNetwork, Community};

fn community_detection_example() {
    let mut network = GeometricNetwork::new();

    // Build a network with clear community structure
    // Community 1: nodes 0,1,2
    let n0 = network.add_node_with_position([0.0, 0.0, 0.0]);
    let n1 = network.add_node_with_position([1.0, 0.0, 0.0]);
    let n2 = network.add_node_with_position([0.5, 1.0, 0.0]);

    // Community 2: nodes 3,4,5
    let n3 = network.add_node_with_position([5.0, 0.0, 0.0]);
    let n4 = network.add_node_with_position([6.0, 0.0, 0.0]);
    let n5 = network.add_node_with_position([5.5, 1.0, 0.0]);

    // Dense intra-community connections
    network.connect_geometric(n0, n1, 0.5);
    network.connect_geometric(n1, n2, 0.5);
    network.connect_geometric(n2, n0, 0.5);

    network.connect_geometric(n3, n4, 0.5);
    network.connect_geometric(n4, n5, 0.5);
    network.connect_geometric(n5, n3, 0.5);

    // Sparse inter-community connection
    network.connect_geometric(n2, n3, 3.0);

    // Detect communities using geometric properties
    // let communities = network.detect_communities();

    println!("Network built for community detection");
}"#,
        "propagation" => r#"use amari::network::{GeometricNetwork, PropagationAnalysis};

fn propagation_analysis_example() {
    let mut network = GeometricNetwork::new();

    // Create a network topology
    let center = network.add_node_with_position([0.0, 0.0, 0.0]);
    let mut surrounding_nodes = Vec::new();

    // Create a star topology
    for i in 0..6 {
        let angle = i as f64 * std::f64::consts::PI / 3.0;
        let x = angle.cos();
        let y = angle.sin();
        let node = network.add_node_with_position([x, y, 0.0]);
        surrounding_nodes.push(node);

        // Connect to center with distance-based weights
        let distance = (x*x + y*y).sqrt();
        network.connect_geometric(center, node, distance);
    }

    // Analyze propagation from center node
    let propagation = PropagationAnalysis::new(&network);
    // let influence_map = propagation.compute_influence(center, 5); // 5 time steps

    println!("Propagation analysis setup complete");
    println!("Center node: {:?}", center);
    println!("Surrounding nodes: {:?}", surrounding_nodes);
}"#,
        _ => r#"use amari::network::GeometricNetwork;

fn simple_network() {
    let mut network = GeometricNetwork::new();

    let a = network.add_node_simple();
    let b = network.add_node_simple();

    network.connect_simple(a, b, 1.0);

    println!("Simple network: {} nodes, {} edges",
             network.node_count(), network.edge_count());
}"#
    };

    Ok(json!({
        "success": true,
        "operation": "network_analysis",
        "context": context,
        "code": code,
        "explanation": "Geometric network analysis and community detection",
        "key_concepts": ["Geometric networks", "Community detection", "Propagation analysis", "Graph theory"]
    }))
}

async fn generate_automata_example(context: &str) -> Result<Value> {
    let code = match context {
        "basic" => r#"use amari::automata::{CellularAutomaton, GeometricRule};

fn basic_cellular_automaton() {
    // Create a 1D cellular automaton with geometric rules
    let size = 100;
    let mut ca = CellularAutomaton::new_1d(size);

    // Initialize with a single active cell in the center
    ca.set_cell(size / 2, 1.0);

    // Define a geometric rule using multivectors
    let rule = GeometricRule::new(
        |neighborhood| {
            // Simple rule: cell becomes average of neighbors
            let sum: f64 = neighborhood.iter().sum();
            sum / neighborhood.len() as f64
        }
    );

    // Evolve for 50 generations
    for generation in 0..50 {
        ca.apply_rule(&rule);

        if generation % 10 == 0 {
            println!("Generation {}: {:?}", generation, ca.get_state_summary());
        }
    }
}"#,
        "2d" => r#"use amari::automata::{CellularAutomaton2D, GeometricRule2D};

fn geometric_2d_automaton() {
    let width = 50;
    let height = 50;
    let mut ca = CellularAutomaton2D::new(width, height);

    // Initialize with a cross pattern
    for i in 0..width {
        ca.set_cell(i, height / 2, 1.0);
    }
    for j in 0..height {
        ca.set_cell(width / 2, j, 1.0);
    }

    // Geometric rule based on distance and angular relationships
    let rule = GeometricRule2D::new(|neighborhood, center_pos| {
        let (x, y) = center_pos;
        let mut total = 0.0;
        let mut count = 0;

        // Weight neighbors by geometric distance
        for (nx, ny, value) in neighborhood {
            let dx = nx as f64 - x as f64;
            let dy = ny as f64 - y as f64;
            let distance = (dx*dx + dy*dy).sqrt();

            if distance > 0.0 {
                total += value / distance;
                count += 1;
            }
        }

        if count > 0 { total / count as f64 } else { 0.0 }
    });

    // Evolve the system
    for step in 0..100 {
        ca.apply_rule(&rule);

        if step % 20 == 0 {
            println!("Step {}: Active cells = {}", step, ca.count_active_cells());
        }
    }
}"#,
        "multivector" => r#"use amari::automata::{MultivectorCA, GeometricEvolution};
use amari::core::{Multivector, Signature};

fn multivector_cellular_automaton() {
    let sig = Signature::new(2, 0, 0); // 2D geometric algebra
    let mut ca = MultivectorCA::new(20, 20, sig);

    // Initialize cells with multivector states
    for i in 5..15 {
        for j in 5..15 {
            let mv = Multivector::new(
                vec![1.0, 0.1, 0.1, 0.05], // scalar + vector + bivector
                sig
            );
            ca.set_cell_multivector(i, j, mv);
        }
    }

    // Evolution rule using geometric algebra operations
    let rule = GeometricEvolution::new(|neighborhood_mvs| {
        // Compute geometric mean of neighboring multivectors
        let mut sum = Multivector::zero(sig);
        let mut count = 0;

        for mv in neighborhood_mvs {
            if !mv.is_zero() {
                // Use geometric product for combining multivectors
                // sum = sum.geometric_product(&mv);
                count += 1;
            }
        }

        if count > 0 {
            // Normalize the result
            // sum.normalize()
            sum
        } else {
            Multivector::zero(sig)
        }
    });

    // Evolve the multivector field
    for generation in 0..50 {
        ca.evolve_with_rule(&rule);

        if generation % 10 == 0 {
            println!("Generation {}: System energy = {:.3}",
                     generation, ca.total_energy());
        }
    }
}"#,
        _ => r#"use amari::automata::CellularAutomaton;

fn simple_ca() {
    let mut ca = CellularAutomaton::new_1d(50);
    ca.randomize();

    for _ in 0..10 {
        ca.step();
    }

    println!("CA evolved for 10 steps");
}"#
    };

    Ok(json!({
        "success": true,
        "operation": "cellular_automata",
        "context": context,
        "code": code,
        "explanation": "Cellular automata with geometric algebra evolution rules",
        "key_concepts": ["Cellular automata", "Geometric rules", "Multivector fields", "Complex systems"]
    }))
}

async fn generate_info_geom_example(context: &str) -> Result<Value> {
    let code = match context {
        "basic" => r#"use amari::info_geom::{FisherInformationMatrix, DuallyFlatManifold};

fn basic_information_geometry() {
    // Create a Fisher information matrix for a simple statistical model
    let parameters = vec![1.0, 0.5, 0.2]; // model parameters

    // Compute Fisher information matrix
    let fisher_matrix = FisherInformationMatrix::new(&parameters, |params| {
        // Log-likelihood function for your statistical model
        // This is a placeholder - replace with your actual model
        let sum: f64 = params.iter().map(|x| x * x).sum();
        -0.5 * sum // negative log-likelihood
    });

    // Get the metric tensor (Fisher information matrix)
    // let metric = fisher_matrix.compute_metric();

    println!("Fisher information matrix computed for {} parameters", parameters.len());
    println!("Parameters: {:?}", parameters);
}"#,
        "manifold" => r#"use amari::info_geom::{DuallyFlatManifold, SimpleAlphaConnection};

fn dually_flat_manifold_example() {
    // Create a dually flat manifold (e.g., exponential family)
    let manifold = DuallyFlatManifold::new(3); // 3-dimensional parameter space

    // Define natural parameters
    let theta = vec![1.0, 0.5, -0.2];

    // Compute expectation parameters (dual coordinates)
    // let eta = manifold.theta_to_eta(&theta);

    // Define alpha-connection (interpolates between exponential and mixture connections)
    let alpha = 0.0; // α = 0 gives exponential connection
    let connection = SimpleAlphaConnection::new(alpha);

    // Compute geodesics and parallel transport
    // let geodesic = manifold.geodesic(&theta, &eta, 10); // 10 points along geodesic

    println!("Dually flat manifold created");
    println!("Natural parameters: {:?}", theta);
    println!("α-connection parameter: {}", alpha);
}"#,
        "optimization" => r#"use amari::info_geom::{FisherInformationMatrix, NaturalGradient};

fn natural_gradient_optimization() {
    // Parameters of a statistical model (e.g., neural network weights)
    let mut parameters = vec![0.1, -0.2, 0.3, 0.15];

    // Objective function (negative log-likelihood)
    let objective = |params: &[f64]| -> f64 {
        // Placeholder for your loss function
        params.iter().map(|x| x * x).sum::<f64>()
    };

    // Gradient of the objective function
    let gradient = |params: &[f64]| -> Vec<f64> {
        // Placeholder for your gradient computation
        params.iter().map(|x| 2.0 * x).collect()
    };

    let learning_rate = 0.01;

    for iteration in 0..100 {
        // Compute Fisher information matrix
        let fisher = FisherInformationMatrix::new(&parameters, objective);

        // Compute ordinary gradient
        let grad = gradient(&parameters);

        // Compute natural gradient (Fisher^{-1} * gradient)
        // let natural_grad = fisher.compute_natural_gradient(&grad);

        // Update parameters using natural gradient
        // for (param, nat_grad) in parameters.iter_mut().zip(natural_grad.iter()) {
        //     *param -= learning_rate * nat_grad;
        // }

        if iteration % 20 == 0 {
            let loss = objective(&parameters);
            println!("Iteration {}: Loss = {:.6}", iteration, loss);
        }
    }
}"#,
        _ => r#"use amari::info_geom::FisherInformationMatrix;

fn simple_fisher_info() {
    let params = vec![1.0, 2.0];
    let fisher = FisherInformationMatrix::new(&params, |p| p[0]*p[0] + p[1]*p[1]);

    println!("Fisher information computed");
}"#
    };

    Ok(json!({
        "success": true,
        "operation": "information_geometry",
        "context": context,
        "code": code,
        "explanation": "Information geometry and statistical manifolds",
        "key_concepts": ["Fisher information", "Statistical manifolds", "Natural gradients", "Dually flat manifolds"]
    }))
}

async fn search_codebase_patterns(amari_path: &str, pattern: &str, scope: &str) -> Result<Value> {
    if pattern.is_empty() {
        return Ok(json!({
            "success": false,
            "error": "Pattern cannot be empty"
        }));
    }

    let search_paths = match scope {
        "core" => vec![format!("{}/amari-core/src", amari_path)],
        "tropical" => vec![format!("{}/amari-tropical/src", amari_path)],
        "dual" => vec![format!("{}/amari-dual/src", amari_path)],
        "tests" => vec![
            format!("{}/tests", amari_path),
            format!("{}/amari-core/tests", amari_path),
            format!("{}/amari-tropical/tests", amari_path),
        ],
        "examples" => vec![
            format!("{}/examples", amari_path),
            format!("{}/amari-core/examples", amari_path),
        ],
        "all" | _ => vec![
            format!("{}/src", amari_path),
            format!("{}/amari-core/src", amari_path),
            format!("{}/amari-tropical/src", amari_path),
            format!("{}/amari-dual/src", amari_path),
            format!("{}/amari-fusion/src", amari_path),
        ],
    };

    let mut results = Vec::new();

    for search_path in search_paths {
        if Path::new(&search_path).exists() {
            // This is a simplified pattern search - in practice you'd use a proper
            // text search library like ripgrep or implement recursive file search
            results.push(json!({
                "path": search_path,
                "status": "would_search_here",
                "note": "Actual implementation would search for pattern in Rust files"
            }));
        }
    }

    Ok(json!({
        "success": true,
        "pattern": pattern,
        "scope": scope,
        "search_results": results,
        "note": "This is a placeholder implementation. Real version would search file contents."
    }))
}

// Helper functions for parsing Rust code

fn extract_public_items(content: &str) -> Vec<String> {
    let mut items = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("pub fn ") {
            if let Some(fn_name) = extract_function_name(trimmed) {
                items.push(format!("fn {}", fn_name));
            }
        } else if trimmed.starts_with("pub struct ") {
            if let Some(struct_name) = extract_struct_name(trimmed) {
                items.push(format!("struct {}", struct_name));
            }
        } else if trimmed.starts_with("pub enum ") {
            if let Some(enum_name) = extract_enum_name(trimmed) {
                items.push(format!("enum {}", enum_name));
            }
        } else if trimmed.starts_with("pub use ") {
            items.push(trimmed.to_string());
        }
    }

    items
}

fn extract_module_docs(content: &str) -> Vec<String> {
    let mut docs = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("//!") {
            docs.push(trimmed.trim_start_matches("//!").trim().to_string());
        } else if trimmed.starts_with("///") {
            docs.push(trimmed.trim_start_matches("///").trim().to_string());
        }
    }

    docs
}

fn extract_workspace_members(content: &str) -> Vec<String> {
    let mut members = Vec::new();
    let mut in_members = false;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("members = [") {
            in_members = true;
            continue;
        }
        if in_members {
            if trimmed == "]" {
                break;
            }
            if let Some(member) = trimmed.strip_prefix('"').and_then(|s| s.strip_suffix('"').or_else(|| s.strip_suffix("\","))) {
                members.push(member.to_string());
            }
        }
    }

    members
}

fn extract_reexports(content: &str) -> Vec<String> {
    let mut reexports = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("pub use ") {
            reexports.push(trimmed.to_string());
        }
    }

    reexports
}

fn extract_dependencies(content: &str) -> Vec<String> {
    let mut deps = Vec::new();
    let mut in_deps = false;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed == "[dependencies]" {
            in_deps = true;
            continue;
        }
        if in_deps {
            if trimmed.starts_with('[') {
                break;
            }
            if let Some(eq_pos) = trimmed.find('=') {
                let dep_name = trimmed[..eq_pos].trim();
                if !dep_name.is_empty() {
                    deps.push(dep_name.to_string());
                }
            }
        }
    }

    deps
}

fn extract_function_name(line: &str) -> Option<String> {
    // Extract function name from "pub fn name(" pattern
    if let Some(start) = line.find("pub fn ") {
        let after_fn = &line[start + 7..];
        if let Some(paren_pos) = after_fn.find('(') {
            return Some(after_fn[..paren_pos].trim().to_string());
        }
    }
    None
}

fn extract_struct_name(line: &str) -> Option<String> {
    // Extract struct name from "pub struct Name" pattern
    if let Some(start) = line.find("pub struct ") {
        let after_struct = &line[start + 11..];
        let name = after_struct.split_whitespace().next()?;
        return Some(name.to_string());
    }
    None
}

fn extract_enum_name(line: &str) -> Option<String> {
    // Extract enum name from "pub enum Name" pattern
    if let Some(start) = line.find("pub enum ") {
        let after_enum = &line[start + 9..];
        let name = after_enum.split_whitespace().next()?;
        return Some(name.to_string());
    }
    None
}

fn generate_feature_deps(features: &[&str]) -> String {
    let mut deps = Vec::new();

    for feature in features {
        match *feature {
            "gpu" => deps.push("# GPU features already included in amari"),
            "serde" => deps.push("serde = { version = \"1.0\", features = [\"derive\"] }"),
            "async" => deps.push("tokio = { version = \"1.0\", features = [\"full\"] }"),
            "plotting" => deps.push("plotters = \"0.3\""),
            "wasm" => deps.push(r#"wasm-bindgen = "0.2"
js-sys = "0.3"
web-sys = "0.3""#),
            _ => {}
        }
    }

    if deps.is_empty() {
        "# Add additional dependencies as needed".to_string()
    } else {
        deps.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_browse_docs_unknown_module() {
        let params = json!({
            "module": "unknown_module"
        });

        let result = browse_docs(params).await.unwrap();
        assert_eq!(result["success"], false);
        assert!(result["error"].as_str().unwrap().contains("Unknown module"));
        assert!(result["available_modules"].is_array());
    }

    #[tokio::test]
    async fn test_browse_docs_valid_module() {
        let params = json!({
            "module": "core"
        });

        let result = browse_docs(params).await.unwrap();
        // Since the Amari library might not be available in test environment,
        // we check that the function handles it gracefully
        assert!(result["success"].is_boolean());
    }

    #[tokio::test]
    async fn test_analyze_code_unknown_target() {
        let params = json!({
            "target": "unknown_target"
        });

        let result = analyze_code(params).await.unwrap();
        assert_eq!(result["success"], false);
        assert!(result["error"].as_str().unwrap().contains("Unknown analysis target"));
        assert!(result["available_targets"].is_array());
    }

    #[tokio::test]
    async fn test_scaffold_project_unknown_type() {
        let params = json!({
            "type": "unknown_type",
            "name": "test-project"
        });

        let result = scaffold_project(params).await.unwrap();
        assert_eq!(result["success"], false);
        assert!(result["error"].as_str().unwrap().contains("Unknown project type"));
        assert!(result["available_types"].is_array());
    }

    #[tokio::test]
    async fn test_scaffold_project_basic() {
        let params = json!({
            "type": "basic",
            "name": "my-test-app",
            "features": ["serde", "async"]
        });

        let result = scaffold_project(params).await.unwrap();
        assert_eq!(result["success"], true);
        assert_eq!(result["project_type"], "basic");
        assert_eq!(result["name"], "my-test-app");
        assert!(result["files"]["Cargo.toml"].is_string());
        assert!(result["files"]["src/main.rs"].is_string());
        assert!(result["next_steps"].is_array());

        // Check that the generated content includes the project name
        let cargo_toml = result["files"]["Cargo.toml"].as_str().unwrap();
        assert!(cargo_toml.contains("my-test-app"));

        let main_rs = result["files"]["src/main.rs"].as_str().unwrap();
        assert!(main_rs.contains("my-test-app"));
    }

    #[tokio::test]
    async fn test_scaffold_project_library() {
        let params = json!({
            "type": "library",
            "name": "my-lib"
        });

        let result = scaffold_project(params).await.unwrap();
        assert_eq!(result["success"], true);
        assert_eq!(result["project_type"], "library");
        assert!(result["files"]["src/lib.rs"].is_string());

        let lib_rs = result["files"]["src/lib.rs"].as_str().unwrap();
        assert!(lib_rs.contains("my-lib"));
        assert!(lib_rs.contains("Error"));
        assert!(lib_rs.contains("#[cfg(test)]"));
    }

    #[tokio::test]
    async fn test_scaffold_project_gpu() {
        let params = json!({
            "type": "gpu",
            "name": "gpu-app"
        });

        let result = scaffold_project(params).await.unwrap();
        assert_eq!(result["success"], true);
        assert_eq!(result["project_type"], "gpu");
        assert!(result["features"].as_array().unwrap().contains(&json!("gpu")));
        assert!(result["gpu_requirements"].is_array());

        let cargo_toml = result["files"]["Cargo.toml"].as_str().unwrap();
        assert!(cargo_toml.contains("features = [\"gpu\"]"));
    }

    #[tokio::test]
    async fn test_scaffold_project_web() {
        let params = json!({
            "type": "web",
            "name": "web-app"
        });

        let result = scaffold_project(params).await.unwrap();
        assert_eq!(result["success"], true);
        assert_eq!(result["project_type"], "web");
        assert!(result["files"]["index.html"].is_string());
        assert!(result["build_instructions"].is_array());

        let lib_rs = result["files"]["src/lib.rs"].as_str().unwrap();
        assert!(lib_rs.contains("wasm_bindgen"));
        assert!(lib_rs.contains("AmariWeb"));
    }

    #[tokio::test]
    async fn test_generate_code_unknown_operation() {
        let params = json!({
            "operation": "unknown_operation"
        });

        let result = generate_code(params).await.unwrap();
        assert_eq!(result["success"], false);
        assert!(result["error"].as_str().unwrap().contains("Unknown operation"));
        assert!(result["available_operations"].is_array());
    }

    #[tokio::test]
    async fn test_generate_code_multivector() {
        let params = json!({
            "operation": "multivector",
            "context": "basic"
        });

        let result = generate_code(params).await.unwrap();
        assert_eq!(result["success"], true);
        assert_eq!(result["operation"], "multivector");
        assert_eq!(result["context"], "basic");
        assert!(result["code"].is_string());
        assert!(result["explanation"].is_string());
        assert!(result["key_concepts"].is_array());

        let code = result["code"].as_str().unwrap();
        assert!(code.contains("Multivector"));
        assert!(code.contains("Signature"));
    }

    #[tokio::test]
    async fn test_generate_code_tropical_algebra() {
        let params = json!({
            "operation": "tropical_algebra",
            "context": "matrix"
        });

        let result = generate_code(params).await.unwrap();
        assert_eq!(result["success"], true);
        assert_eq!(result["operation"], "tropical_algebra");
        assert_eq!(result["context"], "matrix");

        let code = result["code"].as_str().unwrap();
        assert!(code.contains("TropicalMatrix"));
    }

    #[tokio::test]
    async fn test_generate_code_automatic_diff() {
        let params = json!({
            "operation": "automatic_diff",
            "context": "optimization"
        });

        let result = generate_code(params).await.unwrap();
        assert_eq!(result["success"], true);

        let code = result["code"].as_str().unwrap();
        assert!(code.contains("DualNumber"));
        assert!(code.contains("gradient"));
    }

    #[tokio::test]
    async fn test_search_patterns_empty_pattern() {
        let params = json!({
            "pattern": "",
            "scope": "all"
        });

        let result = search_patterns(params).await.unwrap();
        assert_eq!(result["success"], false);
        assert!(result["error"].as_str().unwrap().contains("Pattern cannot be empty"));
    }

    #[tokio::test]
    async fn test_search_patterns_valid() {
        let params = json!({
            "pattern": "geometric_product",
            "scope": "core"
        });

        let result = search_patterns(params).await.unwrap();
        assert_eq!(result["success"], true);
        assert_eq!(result["pattern"], "geometric_product");
        assert_eq!(result["scope"], "core");
        assert!(result["search_results"].is_array());
    }

    #[test]
    fn test_extract_public_items() {
        let content = r#"
pub fn test_function() {}
pub struct TestStruct {}
pub enum TestEnum {}
pub use other::module;
fn private_function() {}
"#;

        let items = extract_public_items(content);
        assert_eq!(items.len(), 4);
        assert!(items.contains(&"fn test_function".to_string()));
        assert!(items.contains(&"struct TestStruct".to_string()));
        assert!(items.contains(&"enum TestEnum".to_string()));
        assert!(items.contains(&"pub use other::module;".to_string()));
    }

    #[test]
    fn test_extract_module_docs() {
        let content = r#"
//! This is a module doc
//! Second line
/// Function documentation
fn test() {}
"#;

        let docs = extract_module_docs(content);
        assert_eq!(docs.len(), 3);
        assert!(docs.contains(&"This is a module doc".to_string()));
        assert!(docs.contains(&"Second line".to_string()));
        assert!(docs.contains(&"Function documentation".to_string()));
    }

    #[test]
    fn test_extract_workspace_members() {
        let content = r#"
[workspace]
members = [
    "amari-core",
    "amari-tropical",
    "amari-dual",
]
"#;

        let members = extract_workspace_members(content);
        assert_eq!(members.len(), 3);
        assert!(members.contains(&"amari-core".to_string()));
        assert!(members.contains(&"amari-tropical".to_string()));
        assert!(members.contains(&"amari-dual".to_string()));
    }

    #[test]
    fn test_extract_dependencies() {
        let content = r#"
[dependencies]
serde = "1.0"
tokio = { version = "1.0", features = ["full"] }
anyhow = "1.0"

[dev-dependencies]
test-dep = "0.1"
"#;

        let deps = extract_dependencies(content);
        assert_eq!(deps.len(), 3);
        assert!(deps.contains(&"serde".to_string()));
        assert!(deps.contains(&"tokio".to_string()));
        assert!(deps.contains(&"anyhow".to_string()));
        // Should not include dev-dependencies
        assert!(!deps.contains(&"test-dep".to_string()));
    }

    #[test]
    fn test_generate_feature_deps() {
        let features = vec!["serde", "gpu", "wasm"];
        let deps = generate_feature_deps(&features);

        assert!(deps.contains("serde"));
        assert!(deps.contains("wasm-bindgen"));
        assert!(deps.contains("GPU features"));
    }

    #[test]
    fn test_extract_function_name() {
        assert_eq!(extract_function_name("pub fn test_func() {"), Some("test_func".to_string()));
        assert_eq!(extract_function_name("pub fn complex_func(param: Type) -> Result<()> {"), Some("complex_func".to_string()));
        assert_eq!(extract_function_name("fn private_func() {"), None);
    }

    #[test]
    fn test_extract_struct_name() {
        assert_eq!(extract_struct_name("pub struct TestStruct {"), Some("TestStruct".to_string()));
        assert_eq!(extract_struct_name("pub struct GenericStruct<T> {"), Some("GenericStruct<T>".to_string()));
        assert_eq!(extract_struct_name("struct PrivateStruct {"), None);
    }

    #[test]
    fn test_extract_enum_name() {
        assert_eq!(extract_enum_name("pub enum TestEnum {"), Some("TestEnum".to_string()));
        assert_eq!(extract_enum_name("pub enum GenericEnum<T> {"), Some("GenericEnum<T>".to_string()));
        assert_eq!(extract_enum_name("enum PrivateEnum {"), None);
    }
}