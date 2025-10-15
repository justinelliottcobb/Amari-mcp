# Amari MCP Server

Model Context Protocol server for the Amari mathematical computing library, providing access to advanced geometric algebra, tropical algebra, automatic differentiation, and GPU-accelerated computations.

## Features

### 🧮 Mathematical Domains

- **Geometric Algebra**: Multivector operations, geometric products, rotor rotations
- **Tropical Algebra**: Min-plus operations, shortest path algorithms, matrix computations
- **Automatic Differentiation**: Forward-mode AD, gradient computation, chain rule
- **Cellular Automata**: Geometric CA evolution, rule-based systems
- **Information Geometry**: Fisher information matrices, statistical manifolds
- **GPU Acceleration**: Batch processing, parallel computations

### 🚀 MCP Tools

#### Geometric Algebra
- `create_multivector` - Create multivectors from coefficients
- `geometric_product` - Compute geometric products
- `rotor_rotation` - Apply rotations using rotors

#### Tropical Algebra
- `tropical_matrix_multiply` - Tropical matrix operations
- `shortest_path` - Graph shortest paths via tropical algebra

#### Automatic Differentiation
- `compute_gradient` - Forward-mode AD gradients
- `jacobian_matrix` - Compute Jacobian matrices

#### Cellular Automata
- `ca_evolution` - Evolve geometric cellular automata

#### Information Geometry
- `fisher_information` - Compute Fisher information matrices

#### GPU Acceleration
- `gpu_batch_compute` - Batch operations on GPU

#### Database (Optional)
- `save_computation` - Save results to PostgreSQL
- `load_computation` - Load saved computations

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

### Basic Server

```bash
./target/release/amari-mcp --port 3000
```

### With GPU Acceleration

```bash
./target/release/amari-mcp --port 3000 --gpu
```

### With Database

```bash
export DATABASE_URL="postgresql://user:password@localhost/amari_mcp"
./target/release/amari-mcp --port 3000 --database-url $DATABASE_URL
```

### Full Configuration

```bash
export DATABASE_URL="postgresql://user:password@localhost/amari_mcp"
./target/release/amari-mcp \
  --host 0.0.0.0 \
  --port 3000 \
  --gpu \
  --database-url $DATABASE_URL \
  --log-level debug
```

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

## Database Setup (Optional)

If using the database feature:

1. Install PostgreSQL
2. Create database:
   ```sql
   CREATE DATABASE amari_mcp;
   ```
3. Set environment variable:
   ```bash
   export DATABASE_URL="postgresql://user:password@localhost/amari_mcp"
   ```
4. Run migrations automatically on startup

## Dependencies

- **Amari v0.9.5**: Core mathematical library
- **MCP**: Model Context Protocol framework
- **Tokio**: Async runtime
- **SQLx** (optional): PostgreSQL database support
- **WGPU** (optional): GPU acceleration

## Development

### Running Tests

```bash
cargo test
```

### With GPU Tests

```bash
cargo test --features gpu
```

### Database Tests

```bash
export DATABASE_URL="postgresql://localhost/amari_mcp_test"
cargo test --features database
```

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