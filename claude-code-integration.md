# Claude Code Integration Guide

This document describes how to use the Amari MCP server within Claude Code sessions for mathematical computing tasks using the **real Model Context Protocol (MCP)** implementation.

## Quick Start for Claude Code Sessions

### 1. Start the MCP Server

The Amari MCP server now uses the **real MCP protocol** with stdio transport (industry standard):

```bash
# Basic server (CPU only)
cd /path/to/amari-mcp
cargo run --release

# With GPU acceleration
cargo run --release -- --gpu

# With database caching (requires PostgreSQL)
cargo run --release --features database -- --database-url "postgresql://user:pass@localhost/amari_mcp"

# All features enabled
cargo run --release --features gpu,database -- --gpu --database-url "postgresql://user:pass@localhost/amari_mcp"
```

### 2. MCP Protocol Communication

The server uses **stdio transport** (standard input/output) for MCP communication, not HTTP. This is the industry standard for Model Context Protocol servers:

```bash
# Server listens for MCP messages on stdin/stdout
# Claude Code and other MCP clients communicate via JSON-RPC over stdio
echo '{"jsonrpc": "2.0", "id": 1, "method": "tools/list"}' | cargo run --release
```

### 3. MCP Technology Implementation

**Real Model Context Protocol** - The Amari MCP server now uses a production-ready implementation:

- **SDK**: `pmcp` v1.8.0 - High-quality Rust SDK with full TypeScript compatibility
- **Transport**: `stdio` (standard input/output) - Industry standard for MCP servers
- **Protocol**: JSON-RPC over stdio - Fully compliant with MCP specification
- **Tool Handlers**: `async-trait` pattern for clean async tool execution
- **Error Handling**: Proper MCP error responses with structured feedback

### 4. Claude Code Usage Patterns

The MCP server provides tools that Claude Code can use to help with mathematical computing tasks in your Amari-dependent projects.

## Common Use Cases in Claude Code Sessions

### Geometric Algebra Development

When working on geometric algebra code:

```python
# Claude can help you verify calculations
result = mcp.call_tool("geometric_product", {
    "a": [1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],  # 1 + e1
    "b": [0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0],  # e2
    "signature": [3, 0, 0]
})
# Result should be [0, 0, 0, 0, 1, 0, 0, 0] (e12)
```

### Tropical Algebra Projects

For min-plus operations and shortest paths:

```python
# Verify tropical matrix multiplication
result = mcp.call_tool("tropical_matrix_multiply", {
    "matrix_a": [[0, 3, float('inf')], [2, 0, 1]],
    "matrix_b": [[0, 1], [2, 0], [3, 2]]
})
```

### Cellular Automata Development

When debugging CA evolution:

```python
# Test CA rules with small grids
result = mcp.call_tool("ca_evolution", {
    "initial_state": [...],  # Your initial multivector grid
    "rule": "geometric",
    "steps": 5,
    "grid_width": 4,
    "grid_height": 4
})
```

## Integration with Your Projects

### Project Structure Support

For projects depending on Amari:

```
your-project/
├── Cargo.toml          # Dependencies include amari = "0.9.5"
├── src/
│   ├── lib.rs
│   └── geometric/      # Your geometric algebra code
├── tests/
└── .claude-code/       # Claude Code configuration
    └── mcp-server.md   # This integration guide
```

### Development Workflow

1. **Start MCP Server**: `cargo run --release` in amari-mcp directory
2. **Open Claude Code**: In your project directory
3. **Verify Operations**: Ask Claude to test mathematical operations via MCP
4. **Debug Issues**: Use MCP tools to verify intermediate calculations
5. **Implement Features**: Claude can suggest implementations based on MCP results

### Testing Integration

Claude can help validate your implementations:

```rust
// In your tests - Claude can verify expected results
#[test]
fn test_geometric_product() {
    let a = Multivector::from_coefficients(vec![1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
    let b = Multivector::from_coefficients(vec![0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
    let result = a * b;

    // Claude can verify this matches MCP server result
    assert_eq!(result.coefficients(), vec![0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0]);
}
```

## Available MCP Tools

**✅ Real MCP Protocol Implementation** - All tools are now properly integrated with the Model Context Protocol using the `pmcp` Rust SDK.

### Core Mathematical Operations

| Tool | Purpose | Input Format | Output Format |
|------|---------|-------------|---------------|
| `create_multivector` | Create multivector from coefficients | `{"coefficients": [f64], "signature": [usize]}` | JSON multivector object |
| `geometric_product` | Compute geometric product a * b | `{"a": [f64], "b": [f64], "signature": [usize]}` | JSON product result |
| `rotor_rotation` | Rotate vector using rotor | `{"vector": [f64], "axis": [f64], "angle": f64}` | JSON rotated vector |
| `tropical_matrix_multiply` | Min-plus matrix multiplication | `{"matrix_a": [[f64]], "matrix_b": [[f64]]}` | JSON tropical result |
| `shortest_path` | Graph shortest path computation | `{"adjacency_matrix": [[f64]], "source": usize}` | JSON distances array |
| `compute_gradient` | Forward-mode automatic differentiation | `{"expression": str, "variables": [str], "values": [f64]}` | JSON gradient vector |
| `ca_evolution` | Evolve geometric cellular automata | `{"initial_state": [[f64]], "rule": str, "steps": usize}` | JSON evolved state |
| `fisher_information` | Information geometry calculations | `{"distribution": str, "parameters": [f64], "data": [f64]}` | JSON Fisher matrix |
| `get_cayley_table` | Retrieve/compute Cayley tables | `{"signature": [usize], "force_recompute": bool}` | JSON Cayley table |

### GPU Acceleration (Optional - requires `--gpu` flag)

| Tool | Purpose | Input Format | Output Format |
|------|---------|-------------|---------------|
| `gpu_batch_compute` | High-performance batch GPU operations | `{"operation": str, "data": Value, "batch_size": usize}` | JSON GPU results |

### Database Caching (Optional - requires `--features database`)

| Tool | Purpose | Input Format | Output Format |
|------|---------|-------------|---------------|
| `save_computation` | Cache expensive computational results | `{"name": str, "type": str, "result": Value}` | JSON confirmation |
| `load_computation` | Retrieve cached computational results | `{"name": str}` | JSON cached result |

### Amari-Fusion Integration

| Tool | Purpose | Input Format | Output Format |
|------|---------|-------------|---------------|
| `get_cayley_table` | **✅ Implemented** - Retrieve/compute Cayley tables | `{"signature": [usize], "force_recompute": bool}` | JSON Cayley table |

## Error Handling

The MCP server returns structured errors:

```json
{
  "success": false,
  "error": "Matrix dimensions incompatible: 3x2 * 4x3",
  "suggestion": "Ensure first matrix columns = second matrix rows"
}
```

Claude can interpret these errors and suggest fixes.

## Performance Considerations

- **GPU Tools**: Use `--gpu` flag for acceleration on large datasets
- **Caching**: Database feature for Cayley tables and expensive operations
- **Batch Operations**: Prefer `gpu_batch_compute` for multiple similar operations

## Claude Code Session Examples

### Example 1: Debugging Geometric Algebra

```
User: "My geometric product implementation is giving wrong results"

Claude: "Let me verify using the MCP server..."
[Calls create_multivector and geometric_product tools]
Claude: "The MCP server shows the expected result should be [0,0,0,0,1,0,0,0]
but your code returns [0,0,0,0,-1,0,0,0]. You have a sign error in the e12
component calculation."
```

### Example 2: Tropical Algebra Verification

```
User: "Is my shortest path algorithm correct for this graph?"

Claude: "Let me check with the MCP tropical algebra tools..."
[Calls shortest_path tool]
Claude: "The MCP server confirms shortest path from node 0 to node 3 is 3.0
via path 0→1→3. Your algorithm's result of 4.0 suggests you're not handling
the edge weights correctly."
```

## Best Practices

1. **Keep MCP Server Running**: Start it at the beginning of your Claude Code session
2. **Use for Verification**: Great for checking mathematical correctness
3. **GPU for Large Data**: Enable GPU acceleration for performance testing
4. **Cache Expensive Operations**: Use database feature for Cayley tables
5. **Error Interpretation**: Let Claude help interpret MCP error messages

## Troubleshooting

### Server Won't Start
```bash
# Check if port is in use
lsof -i :3000

# Try different port
cargo run -- --port 3001
```

### GPU Acceleration Issues
```bash
# Check GPU availability
cargo run --features gpu -- --gpu

# Fall back to CPU
cargo run -- --port 3000
```

### Tool Errors
- Verify input format matches tool specifications
- Check coefficient array sizes for multivectors
- Ensure matrix dimensions are compatible for operations