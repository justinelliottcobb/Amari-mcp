Start the Amari MCP server with GPU acceleration for mathematical computing tools

```bash
# Navigate to MCP server directory and start with GPU support
cd /home/elliotthall/working/math/amari-mcp && cargo run --release --features gpu -- --port 3000 --gpu --log-level info
```

This starts the MCP server with:
- Port 3000 (configurable)
- GPU acceleration enabled
- All mathematical tools registered
- Comprehensive logging

The server provides tools for:
- Geometric algebra operations
- Tropical algebra computations
- Automatic differentiation
- Cellular automata evolution
- Information geometry calculations
- GPU-accelerated batch processing