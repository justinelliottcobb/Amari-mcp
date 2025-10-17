/*!
# Amari MCP Server Library

Model Context Protocol server library for the Amari mathematical computing ecosystem.

Provides tools for:
- Geometric algebra operations with on-demand Cayley table computation
- Tropical algebra computations
- Automatic differentiation
- Cellular automata evolution
- Information geometry calculations
- GPU-accelerated batch processing

## Features

- **Core**: Basic MCP server functionality with all mathematical operations
- **GPU**: GPU acceleration support for batch operations

## Example

```rust,no_run
use amari_mcp::mcp_pmcp;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let server = mcp_pmcp::create_amari_mcp_server(false).await?;
    server.run_stdio().await?;
    Ok(())
}
```
*/

pub mod mcp_pmcp;
pub mod tools;
pub mod utils;

// Database module removed - MCP servers should be simple and stateless

// Re-export commonly used types
pub use mcp_pmcp::create_amari_mcp_server;
pub use tools::cayley_tables;
