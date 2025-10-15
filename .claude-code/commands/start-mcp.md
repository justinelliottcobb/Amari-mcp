Start the Amari MCP server with real Model Context Protocol implementation

```bash
# Navigate to MCP server directory and start with GPU support
cd /home/elliotthall/working/math/amari-mcp && cargo run --release -- --gpu
```

**Real MCP Protocol** - This starts the production-ready MCP server with:
- **stdio transport** (industry standard for MCP)
- **pmcp SDK** - High-quality Rust MCP implementation
- **GPU acceleration** enabled (optional: remove --gpu for CPU-only)
- **All mathematical tools** registered and ready
- **JSON-RPC over stdio** for Claude Code integration

For database caching:
```bash
# With PostgreSQL database support
cargo run --release --features database -- --gpu --database-url "postgresql://user:pass@localhost/amari_mcp"
```

The server provides tools for:
- Geometric algebra operations
- Tropical algebra computations
- Automatic differentiation
- Cellular automata evolution
- Information geometry calculations
- GPU-accelerated batch processing