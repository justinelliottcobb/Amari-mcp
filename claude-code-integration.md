# Claude Code Integration Guide

This document describes how to use the Amari MCP server within Claude Code sessions for mathematical computing tasks.

## Quick Start for Claude Code Sessions

### 1. Start the MCP Server

```bash
# In your project terminal
cd /path/to/amari-mcp
cargo run --release --features gpu -- --port 3000 --gpu
```

### 2. Verify Server is Running

```bash
# Check if server started successfully
curl -s http://localhost:3000/health || echo "Use the MCP protocol instead of HTTP"
```

### 3. Claude Code Usage Patterns

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

### Core Mathematical Operations

| Tool | Purpose | Input | Output |
|------|---------|-------|--------|
| `create_multivector` | Create multivector from coefficients | coefficients, signature | multivector object |
| `geometric_product` | Compute a ∧ b | multivector a, b | product result |
| `rotor_rotation` | Rotate vector by rotor | vector, axis, angle | rotated vector |
| `tropical_matrix_multiply` | Min-plus matrix product | matrix_a, matrix_b | tropical result |
| `shortest_path` | Graph shortest paths | adjacency_matrix, source | distances |
| `compute_gradient` | Forward-mode AD | expression, variables, values | gradient |
| `ca_evolution` | Evolve cellular automata | initial_state, rule, steps | evolved state |
| `fisher_information` | Information geometry | distribution, parameters | Fisher matrix |

### GPU Acceleration (Optional)

| Tool | Purpose | Input | Output |
|------|---------|-------|--------|
| `gpu_batch_compute` | Batch GPU operations | operation, data, batch_size | GPU results |

### Future Tools (Planned)

| Tool | Purpose | Status |
|------|---------|--------|
| `cayley_table_lookup` | Cached Cayley table operations | Todo |
| `save_computation` | Cache expensive results | Database feature |
| `load_computation` | Retrieve cached results | Database feature |

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