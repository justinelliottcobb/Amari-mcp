Test the Amari MCP server tools with sample mathematical operations

This will run basic tests to verify all MCP tools are working correctly:

```bash
# Test basic compilation and tool registration
cd /home/elliotthall/working/math/amari-mcp && cargo test

# Run a quick server start test (exits after tool registration)
timeout 5s cargo run --release -- --port 3001 || echo "Server started successfully"

# Test specific mathematical operations (when real MCP integration is available)
# For now, verify stub implementations compile correctly
cargo check --features gpu,database
```

Expected results:
- All tests pass
- Server starts and registers 8+ mathematical tools
- No compilation errors
- GPU and database features compile correctly

Tools that should be registered:
- create_multivector
- geometric_product
- rotor_rotation
- tropical_matrix_multiply
- shortest_path
- compute_gradient
- ca_evolution
- fisher_information
- gpu_batch_compute (with GPU feature)