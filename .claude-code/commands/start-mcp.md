Start the Amari MCP server with real Model Context Protocol implementation

```bash
# Navigate to MCP server directory and start (CPU only)
cd /home/elliotthall/working/math/amari-mcp && cargo run --release
```

**Or with GPU acceleration:**
```bash
# Start with GPU support
cd /home/elliotthall/working/math/amari-mcp && cargo run --release --features gpu -- --gpu
```

**Real MCP Protocol** - This starts the production-ready MCP server with:
- **stdio transport** (industry standard for MCP)
- **pmcp SDK** - High-quality Rust MCP implementation
- **Stateless design** - no database setup required
- **Fast startup** - immediate availability
- **All mathematical tools** registered and ready
- **JSON-RPC over stdio** for Claude Code integration

The server provides tools for:
- Geometric algebra operations (multivectors, products, rotors)
- Tropical algebra computations (min-plus, shortest paths)
- Automatic differentiation (gradients, chain rule)
- Cellular automata evolution (geometric CA)
- Information geometry calculations (Fisher information)
- On-demand Cayley table computation
- GPU-accelerated batch processing (with --gpu flag)