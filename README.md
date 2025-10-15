# Amari MCP Server

**Model Context Protocol server** for the Amari mathematical computing library, providing Claude Code and other AI assistants with access to advanced geometric algebra, tropical algebra, automatic differentiation, and GPU-accelerated computations.

**MCP Implementation** - Uses `pmcp` Rust SDK with stdio transport (industry standard)

## 🚀 Quick Start for Claude Code

```bash
# 1. Clone and setup
git clone https://github.com/justinelliottcobb/amari-mcp.git
cd amari-mcp

# 2. Run automated setup
./setup-claude-code.sh

# 3. Add generated config to Claude Code
# Copy contents of claude-code-config.json to your Claude Code MCP settings
```

**That's it!** You'll now have access to advanced mathematical tools in Claude Code sessions.

## Features

### Mathematical Domains

- **Geometric Algebra**: Multivector operations, geometric products, rotor rotations
- **Tropical Algebra**: Min-plus operations, shortest path algorithms, matrix computations
- **Automatic Differentiation**: Forward-mode AD, gradient computation, chain rule
- **Cellular Automata**: Geometric CA evolution, rule-based systems
- **Information Geometry**: Fisher information matrices, statistical manifolds
- **GPU Acceleration**: Batch processing, parallel computations

### MCP Tools (10+ Available)

All tools implemented with `pmcp` SDK and `async-trait` handlers

#### Geometric Algebra
- `create_multivector` - Create multivectors from coefficients and signatures
- `geometric_product` - Compute geometric products with metric signatures
- `rotor_rotation` - Apply rotations using rotors and bivector exponentials

#### Tropical Algebra
- `tropical_matrix_multiply` - Min-plus matrix operations for optimization
- `shortest_path` - Graph shortest paths via tropical algebra

#### Automatic Differentiation
- `compute_gradient` - Forward-mode AD with expression parsing

#### Cellular Automata
- `ca_evolution` - Evolve geometric cellular automata with custom rules

#### Information Geometry
- `fisher_information` - Compute Fisher information matrices for statistical manifolds

#### Cayley Tables (Database Feature)
- `get_cayley_table` - Fast database lookups for precomputed tables
- `precompute_cayley_tables` - Precompute essential geometric algebra signatures
- `cayley_precompute_status` - View precomputation status and performance stats

#### GPU Acceleration (Optional - `--gpu` flag)
- `gpu_batch_compute` - Batch operations on GPU

#### Database Caching (Optional - `--features database`)
- `save_computation` - Cache expensive results in PostgreSQL
- `load_computation` - Retrieve cached computational results

## Installation

```bash
git clone https://github.com/justinelliottcobb/amari-mcp.git
cd amari-mcp
cargo build --release
```

### Features

```bash
# Basic installation
cargo build --release

# With GPU support
cargo build --release --features gpu

# With database support
cargo build --release --features database

# Full features
cargo build --release --features gpu,database
```

## Usage

🚀 **Real MCP Protocol** - Uses stdio transport (industry standard)

### Basic MCP Server

```bash
# CPU-only server with stdio transport
cargo run --release
```

### With GPU Acceleration

```bash
# GPU-accelerated MCP server
cargo run --release -- --gpu
```

### With Database Caching

```bash
# PostgreSQL caching for expensive computations
export DATABASE_URL="postgresql://user:password@localhost/amari_mcp"
cargo run --release --features database -- --database-url $DATABASE_URL
```

### Full Configuration

```bash
# All features enabled
export DATABASE_URL="postgresql://user:password@localhost/amari_mcp"
cargo run --release --features gpu,database -- \
  --gpu \
  --database-url $DATABASE_URL
```

**Key Features:**
- ✅ **Real MCP Protocol** with `pmcp` SDK
- ✅ **stdio transport** (JSON-RPC over stdin/stdout)
- ✅ **Claude Code compatible** out-of-the-box
- ✅ **Production ready** for AI assistant integration

## MCP Tool Examples

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
    "matrix_a": [[0, 3, float('inf')], [2, 0, 1], [float('inf'), 4, 0]],
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

### GPU Acceleration

```python
# Batch GPU computation
gpu_result = await mcp_client.call_tool("gpu_batch_compute", {
    "operation": "geometric_product",
    "data": [...],  # Large batch of multivectors
    "batch_size": 1024
})
```

### Cayley Table Precomputation

Precomputed Cayley tables provide fast access to expensive geometric algebra computations.

```python
# Get Cayley table from cache or compute on-demand
cayley_table = await mcp_client.call_tool("get_cayley_table", {
    "signature": [3, 0, 0]  # 3D Euclidean space
})
# Returns from database cache if available, otherwise computes

# Check precomputation status
status = await mcp_client.call_tool("cayley_precompute_status", {})
print(f"Tables cached: {status['total_precomputed']}")
print(f"Storage used: {status['total_storage_mb']} MB")
```

#### Management Commands

```bash
# Precompute essential geometric algebra signatures (~25 tables, ~20-50MB)
./amari-mcp --database-url=$DATABASE_URL precompute-cayley

# Check what's been precomputed
./amari-mcp --database-url=$DATABASE_URL cayley-status

# Clear all cached tables (for testing)
./amari-mcp --database-url=$DATABASE_URL cayley-clear --yes
```

**Performance Impact**:
- **Before**: 50-200ms computation per Cayley table
- **After**: <1ms database lookup
- **Storage**: ~20-50MB for essential signatures (3,0,0) through (5,0,0)

## Database Setup (Optional)

**Required for Cayley Table Caching**

1. Install PostgreSQL
2. Create database:
   ```sql
   CREATE DATABASE amari_mcp;
   ```
3. Set environment variable:
   ```bash
   export DATABASE_URL="postgresql://user:password@localhost/amari_mcp"
   ```
4. Migrations run automatically on startup:
   - `001_initial_schema.sql` - Basic computation caching
   - `002_cayley_tables.sql` - Cayley table precomputation system

5. Precompute essential tables for improved performance:
   ```bash
   ./amari-mcp --database-url=$DATABASE_URL precompute-cayley
   ```

## Dependencies

- **Amari v0.9.5**: Core mathematical library
- **MCP**: Model Context Protocol framework
- **Tokio**: Async runtime
- **SQLx** (optional): PostgreSQL database support
- **WGPU** (optional): GPU acceleration

## Development

### Building and Running

```bash
# Build with all features
cargo build --features database,gpu

# Run the server
cargo run --features database -- --database-url=postgresql://user:password@localhost/amari_mcp
```

### Test Suite

The project includes comprehensive tests:

- **Unit Tests**: 18 tests covering core functionality
- **Integration Tests**: Database migration and precomputation tests
- **CI/CD**: GitHub Actions with PostgreSQL service

```bash
# Quick unit tests (no database required)
cargo test --lib

# Database tests (requires PostgreSQL)
export TEST_DATABASE_URL="postgres://postgres:postgres@localhost:5432/amari_mcp_test"
cargo test --features database

# Run comprehensive test script
./test.sh
```

Test coverage includes:
- Cayley table computation and caching
- MCP tool handlers and error handling
- Database migrations and schema validation
- Performance and data integrity checks


## Architecture

```
amari-mcp/
├── src/
│   ├── main.rs           # CLI entry point
│   ├── server.rs         # MCP server implementation
│   ├── tools/            # MCP tool implementations
│   │   ├── geometric.rs  # Geometric algebra operations
│   │   ├── tropical.rs   # Tropical algebra operations
│   │   ├── autodiff.rs   # Automatic differentiation
│   │   ├── cellular_automata.rs  # CA evolution
│   │   ├── info_geometry.rs      # Information geometry
│   │   ├── gpu.rs        # GPU acceleration
│   │   └── database.rs   # Database operations
│   └── utils.rs          # Helper functions
├── migrations/           # Database migrations
└── examples/            # Usage examples
```

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