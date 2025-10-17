# Amari MCP Server

**Comprehensive library access MCP server** for the Amari mathematical computing ecosystem. Designed to provide Claude Code with full access to the Amari library for developing mathematical applications, not just specific computations.

**Library-Focused** - Exposes the entire Amari ecosystem for application development with documentation browsing, code analysis, project scaffolding, and pattern generation tools.

## 🚀 Quick Start for Claude Code

```bash
# 1. Clone and build
git clone https://github.com/justinelliottcobb/amari-mcp.git
cd amari-mcp
cargo build --release

# 2. Run the MCP server
cargo run --release

# 3. Configure Claude Code
# Add the following to your Claude Code MCP settings:
{
  "amari-mcp": {
    "command": "/path/to/amari-mcp/target/release/amari-mcp",
    "args": []
  }
}
```

**That's it!** You'll now have access to advanced mathematical tools in Claude Code sessions.

## Features

### Mathematical Domains

- **Geometric Algebra**: Multivector operations, geometric products, rotor rotations
- **Tropical Algebra**: Min-plus operations, shortest path algorithms, matrix computations
- **Automatic Differentiation**: Forward-mode AD, gradient computation, chain rule
- **Cellular Automata**: Geometric CA evolution, rule-based systems
- **Information Geometry**: Fisher information matrices, statistical manifolds
- **GPU Acceleration**: Batch processing, parallel computations (optional)

### MCP Tools Available

All tools implemented with `pmcp` SDK and real MCP protocol over stdio transport.

#### Library Development Tools
- `browse_docs` - Browse Amari module documentation and API information
- `analyze_code` - Analyze source code structure, exports, and dependencies
- `scaffold_project` - Generate project templates (basic, library, GPU, WebAssembly)
- `generate_code` - Create code examples for specific Amari operations
- `search_patterns` - Search for patterns and idioms in the Amari codebase

#### Core Mathematical Operations
- `create_multivector` - Create multivectors from coefficients and signatures
- `geometric_product` - Compute geometric products with metric signatures
- `rotor_rotation` - Apply rotations using rotors and bivector exponentials
- `tropical_matrix_multiply` - Min-plus matrix operations for optimization
- `shortest_path` - Graph shortest paths via tropical algebra
- `compute_gradient` - Forward-mode automatic differentiation
- `ca_evolution` - Evolve geometric cellular automata with custom rules
- `fisher_information` - Compute Fisher information matrices
- `get_cayley_table` - On-demand Cayley table computation

#### GPU Acceleration (with `--gpu` flag)
- `gpu_batch_compute` - Batch operations on GPU for large datasets

## Installation

```bash
git clone https://github.com/justinelliottcobb/amari-mcp.git
cd amari-mcp
cargo build --release
```

### Build Options

```bash
# Basic installation (CPU only)
cargo build --release

# With GPU support
cargo build --release --features gpu
```

## Usage

### Basic MCP Server

```bash
# Start MCP server with stdio transport
cargo run --release
```

### With GPU Acceleration

```bash
# GPU-accelerated MCP server
cargo run --release --features gpu -- --gpu
```

**Key Features:**
- ✅ **Real MCP Protocol** with `pmcp` SDK
- ✅ **stdio transport** (JSON-RPC over stdin/stdout)
- ✅ **Claude Code compatible** out-of-the-box
- ✅ **Stateless & Simple** - no database setup required
- ✅ **On-demand computation** - fast mathematical operations

## MCP Tool Examples

### Library Development Tools

```python
# Browse documentation for a specific module
docs = await mcp_client.call_tool("browse_docs", {
    "module": "core",
    "query": "Multivector"
})

# Analyze project structure
structure = await mcp_client.call_tool("analyze_code", {
    "target": "structure"
})

# Generate a new Amari application
project = await mcp_client.call_tool("scaffold_project", {
    "type": "basic",
    "name": "my-amari-app",
    "features": ["gpu", "serde"]
})

# Generate code examples for geometric operations
code = await mcp_client.call_tool("generate_code", {
    "operation": "multivector",
    "context": "physics"
})

# Search for patterns in the codebase
patterns = await mcp_client.call_tool("search_patterns", {
    "pattern": "geometric_product",
    "scope": "core"
})
```

### Geometric Algebra

```python
# Create a multivector in 3D Euclidean space
result = await mcp_client.call_tool("create_multivector", {
    "coefficients": [1.0, 0.5, 0.3, 0.2, 0.1, 0.15, 0.25, 0.05],
    "signature": [3, 0, 0]
})

# Compute geometric product
product = await mcp_client.call_tool("geometric_product", {
    "a": [1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    "b": [0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0]
})

# Rotate vector using rotor
rotated = await mcp_client.call_tool("rotor_rotation", {
    "vector": [1.0, 0.0, 0.0],
    "axis": [0.0, 0.0, 1.0],
    "angle": 1.5708  # π/2 radians
})
```

### Tropical Algebra

```python
# Tropical matrix multiplication
result = await mcp_client.call_tool("tropical_matrix_multiply", {
    "matrix_a": [[0, 3, null], [2, 0, 1], [null, 4, 0]],
    "matrix_b": [[0, 1], [2, 0], [3, 2]]
})

# Shortest path in graph
paths = await mcp_client.call_tool("shortest_path", {
    "adjacency_matrix": [
        [0, 2, null, 1],
        [null, 0, 3, 2],
        [null, null, 0, 1],
        [null, null, null, 0]
    ],
    "source": 0
})
```

### Cayley Tables

```python
# Get Cayley table (computed on-demand)
cayley_table = await mcp_client.call_tool("get_cayley_table", {
    "signature": [3, 0, 0]  # 3D Euclidean space
})
# Fast on-demand computation optimized for interactive use
```

### GPU Acceleration

```python
# Batch GPU computation (requires --gpu flag)
gpu_result = await mcp_client.call_tool("gpu_batch_compute", {
    "operation": "geometric_product",
    "data": [...],  # Large batch of multivectors
    "batch_size": 1024
})
```

## Dependencies

- **Amari v0.9.5**: Core mathematical library
- **pmcp**: High-quality Rust MCP SDK
- **Tokio**: Async runtime
- **WGPU** (optional): GPU acceleration

## Development

### Building and Running

```bash
# Build with all features
cargo build --features gpu

# Run tests
cargo test

# Run the server
cargo run --release
```

### Test Suite

The project includes comprehensive tests covering all mathematical operations:

```bash
# Run all tests (17 tests covering core functionality)
cargo test

# Test with GPU features
cargo test --features gpu
```

Test coverage includes:
- Geometric algebra operations (multivectors, products, rotors)
- Tropical algebra computations (matrices, shortest paths)
- Automatic differentiation and gradients
- Cellular automata evolution
- Information geometry calculations
- GPU batch processing
- MCP tool handlers and error handling

## Architecture

```
amari-mcp/
├── src/
│   ├── main.rs           # CLI entry point
│   ├── mcp_pmcp.rs      # MCP server implementation with pmcp
│   ├── tools/           # MCP tool implementations
│   │   ├── geometric_stub.rs    # Geometric algebra operations
│   │   ├── tropical.rs          # Tropical algebra operations
│   │   ├── autodiff.rs          # Automatic differentiation
│   │   ├── cellular_automata.rs # CA evolution
│   │   ├── info_geometry.rs     # Information geometry
│   │   ├── cayley_tables.rs     # On-demand Cayley tables
│   │   └── gpu.rs               # GPU acceleration
│   └── utils.rs         # Helper functions
└── examples/            # Usage examples
```

## Design Philosophy

This MCP server provides **comprehensive library access** rather than just mathematical operations. It's designed to help you develop Amari-dependent applications by exposing the entire library ecosystem.

- **Library-focused**: Full access to Amari documentation, code structure, and patterns
- **Development-oriented**: Tools for scaffolding, code generation, and project analysis
- **On-demand computation**: All operations computed when requested (no caching complexity)
- **Educational**: Browse docs, understand APIs, learn patterns and idioms
- **Perfect for Claude Code**: Optimized for AI-assisted application development

## Contributing

1. Fork the repository
2. Create feature branch
3. Add tests for new functionality
4. Submit pull request

## License

MIT OR Apache-2.0

## Related Projects

- [Amari](https://github.com/justinelliottcobb/Amari) - Core mathematical library
- [Model Context Protocol](https://modelcontextprotocol.io/) - MCP specification