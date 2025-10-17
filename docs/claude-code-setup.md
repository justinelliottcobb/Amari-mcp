# Claude Code Setup Guide

## Quick Start: Local Development

### 1. Build and Run

```bash
# In your amari-mcp directory
cargo build --release

# Run basic MCP server
./target/release/amari-mcp

# Or run with GPU acceleration
cargo build --release --features gpu
./target/release/amari-mcp --gpu
```

### 2. Claude Code Configuration

In Claude Code, connect to your local MCP server using stdio transport:

#### Basic Configuration
```json
{
  "mcpServers": {
    "amari": {
      "command": "/path/to/amari-mcp/target/release/amari-mcp",
      "args": []
    }
  }
}
```

#### With GPU Acceleration
```json
{
  "mcpServers": {
    "amari": {
      "command": "/path/to/amari-mcp/target/release/amari-mcp",
      "args": ["--gpu"],
      "env": {
        "RUST_LOG": "info"
      }
    }
  }
}
```

### 3. Available Tools

Once connected, you'll have access to these tools in Claude Code:

#### Core Mathematical Operations
- `create_multivector` - Create multivectors from coefficients
- `geometric_product` - Compute geometric products
- `rotor_rotation` - Apply rotor rotations
- `get_cayley_table` - On-demand Cayley table computation
- `tropical_matrix_multiply` - Min-plus operations
- `shortest_path` - Graph algorithms via tropical algebra
- `compute_gradient` - Automatic differentiation
- `ca_evolution` - Cellular automata evolution
- `fisher_information` - Information geometry calculations

#### GPU Acceleration (with `--gpu` flag)
- `gpu_batch_compute` - High-performance batch operations

## Production Deployment Options

### Option 1: Local Binary (Recommended)

**Pros:**
- Simple setup - no database configuration needed
- Immediate setup
- Full performance
- No network latency
- Easy debugging

**Setup:**
1. Build release binary: `cargo build --release`
2. Configure Claude Code to use local path
3. Run when needed

### Option 2: systemd Service (Linux)

For always-on local service:

```ini
# /etc/systemd/system/amari-mcp.service
[Unit]
Description=Amari MCP Server
After=network.target

[Service]
Type=simple
User=amari
WorkingDirectory=/opt/amari-mcp
ExecStart=/opt/amari-mcp/amari-mcp
Restart=always
Environment=RUST_LOG=info

[Install]
WantedBy=multi-user.target
```

### Option 3: Docker Container

```dockerfile
# Dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release --features gpu

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/amari-mcp /usr/local/bin/
CMD ["amari-mcp"]
```

### Option 4: Cloud Deployment

#### Railway/Render/Fly.io
```yaml
# fly.toml
app = "amari-mcp"

[build]
  dockerfile = "Dockerfile"

[[services]]
  internal_port = 8080
  processes = ["app"]
  protocol = "tcp"
```

**Note:** MCP servers use stdio transport, so they work best as local binaries or containerized services accessed via SSH.

## Configuration Examples

### Basic Configuration
```json
{
  "mcpServers": {
    "amari": {
      "command": "/usr/local/bin/amari-mcp"
    }
  }
}
```

### GPU Acceleration Enabled
```json
{
  "mcpServers": {
    "amari": {
      "command": "/usr/local/bin/amari-mcp",
      "args": ["--gpu", "--log-level", "info"],
      "env": {
        "RUST_LOG": "amari_mcp=info"
      }
    }
  }
}
```

### Remote Server Configuration
```json
{
  "mcpServers": {
    "amari": {
      "command": "ssh",
      "args": [
        "user@remote-server",
        "/opt/amari-mcp/amari-mcp"
      ]
    }
  }
}
```

## Performance Optimization

### GPU Acceleration
```bash
# Enable GPU features (requires CUDA/ROCm)
cargo build --release --features gpu
./amari-mcp --gpu
```

### On-demand Computation
The server is optimized for interactive use with fast on-demand computation:
- Cayley tables computed in milliseconds
- No database setup required
- Immediate startup
- Stateless design

## Troubleshooting

### Server Won't Start
```bash
# Check build
cargo build --release

# Try basic run with verbose logging
RUST_LOG=debug ./target/release/amari-mcp
```

### GPU Acceleration Issues
```bash
# Check GPU availability
cargo run --features gpu -- --gpu

# Fall back to CPU if GPU not available
cargo run --release
```

### Connection Issues
1. Check that the binary path is correct in Claude Code config
2. Verify the binary is executable: `chmod +x target/release/amari-mcp`
3. Check logs: `RUST_LOG=debug ./amari-mcp`
4. Ensure no firewall blocking (though MCP uses stdio, not network)

### Performance Issues
1. Use `--gpu` flag for large computations
2. Consider batch operations with `gpu_batch_compute`
3. Monitor CPU/memory usage for large mathematical operations

### Common Errors
- **"Binary not found"**: Check path in Claude Code configuration
- **"GPU acceleration not enabled"**: Add `--gpu` flag and verify drivers
- **"Permission denied"**: Make binary executable and check user permissions

## Security Considerations

### Local Development
- MCP uses stdio transport (secure by default)
- No network exposure - communication only via stdin/stdout
- No database credentials to manage

### Production Deployment
- Run as dedicated user with minimal privileges
- Keep binary updated with latest Amari and security patches
- Use containerization for isolation if needed
- For remote deployment, use SSH with proper key management

## Design Philosophy

This MCP server follows a **simple, stateless design philosophy**:

- **No database setup required** - just build and run
- **Fast startup** - no migrations or complex initialization
- **On-demand computation** - operations computed when requested
- **Interactive optimization** - designed for Claude Code sessions
- **Minimal dependencies** - focused on core mathematical operations

This makes it perfect for development workflows where you want mathematical tools available instantly without complex setup procedures.