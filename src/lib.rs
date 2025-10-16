/*!
# Amari MCP Server Library

Model Context Protocol server library for the Amari mathematical computing ecosystem.

Provides tools for:
- Geometric algebra operations and Cayley table caching
- Tropical algebra computations
- Automatic differentiation
- Cellular automata evolution
- Information geometry calculations
- GPU-accelerated batch processing
- Database caching and precomputation

## Features

- **Core**: Basic MCP server functionality
- **Database**: PostgreSQL caching and Cayley table precomputation
- **GPU**: GPU acceleration support

## Example

```rust,no_run
use amari_mcp::mcp_pmcp;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let server = mcp_pmcp::create_amari_mcp_server(
        false, // GPU disabled
        #[cfg(feature = "database")]
        false, // Database disabled
    ).await?;

    server.run_stdio().await?;
    Ok(())
}
```
*/

pub mod mcp_pmcp;
pub mod mcp_stub;
pub mod server;
pub mod tools;
pub mod utils;

#[cfg(feature = "database")]
pub mod database;

// Re-export commonly used types
pub use mcp_pmcp::create_amari_mcp_server;
pub use tools::cayley_tables;
