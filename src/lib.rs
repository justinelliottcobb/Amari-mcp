/*!
# Amari MCP Server Library

Config-driven MCP server that provides ground-truth API reference
for Rust library ecosystems. Eliminates hallucinations by parsing
source code directly and serving accurate type signatures, documentation,
and module structure via the Model Context Protocol.

## Architecture

```text
manifest (TOML) → parser (syn) → ApiIndex → MCP tools → Claude Code
```

The server is library-agnostic: point it at any Rust workspace via a
manifest file and it will index the full public API surface.
*/

pub mod config;
pub mod mcp_pmcp;
pub mod parser;
pub mod tools;
