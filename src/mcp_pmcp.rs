use anyhow::Result;
use pmcp::{Server, ServerCapabilities};
use std::path::PathBuf;
use tracing::info;

use crate::config::LibraryManifest;
use crate::parser::index::{ApiIndex, Validated};
use crate::tools::{
    api_search, browse_docs, dependency_graph, feature_map, module_overview, reload, type_info,
    usage_examples, SharedState,
};

/// Create and run the MCP server with the given validated index.
pub async fn create_mcp_server(
    index: ApiIndex<Validated>,
    manifest: LibraryManifest,
    manifest_path: PathBuf,
) -> Result<()> {
    let state = SharedState::new(index, manifest, manifest_path);

    info!("Registering 8 MCP tools");

    let server = Server::builder()
        .name("amari-mcp")
        .version(env!("CARGO_PKG_VERSION"))
        .capabilities(ServerCapabilities::default())
        .tool(
            "api_search",
            api_search::ApiSearchHandler {
                state: state.clone(),
            },
        )
        .tool(
            "type_info",
            type_info::TypeInfoHandler {
                state: state.clone(),
            },
        )
        .tool(
            "module_overview",
            module_overview::ModuleOverviewHandler {
                state: state.clone(),
            },
        )
        .tool(
            "feature_map",
            feature_map::FeatureMapHandler {
                state: state.clone(),
            },
        )
        .tool(
            "dependency_graph",
            dependency_graph::DependencyGraphHandler {
                state: state.clone(),
            },
        )
        .tool(
            "browse_docs",
            browse_docs::BrowseDocsHandler {
                state: state.clone(),
            },
        )
        .tool(
            "usage_examples",
            usage_examples::UsageExamplesHandler {
                state: state.clone(),
            },
        )
        .tool(
            "reload",
            reload::ReloadHandler {
                state: state.clone(),
            },
        )
        .build()
        .map_err(|e| anyhow::anyhow!("Failed to build MCP server: {e}"))?;

    info!("MCP server ready, starting stdio transport");
    server.run_stdio().await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn placeholder() {
        // Server creation requires stdio transport; tested via integration tests
    }
}
