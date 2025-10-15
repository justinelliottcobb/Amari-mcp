# Claude Code Integration Guide

## Quick Start: Local Development

### 1. Build and Run Locally

```bash
# In your amari-mcp directory
cargo build --release --features database,gpu

# Run with database (optional)
export DATABASE_URL="postgresql://user:password@localhost/amari_mcp"
./target/release/amari-mcp --database-url=$DATABASE_URL

# Or run without database
./target/release/amari-mcp
```

### 2. Claude Code Configuration

In Claude Code, you can connect to your local MCP server using the stdio transport:

```json
{
  "mcpServers": {
    "amari": {
      "command": "/path/to/amari-mcp/target/release/amari-mcp",
      "args": ["--database-url", "postgresql://user:password@localhost/amari_mcp"],
      "env": {
        "RUST_LOG": "info"
      }
    }
  }
}
```

### 3. Available Tools

Once connected, you'll have access to these tools in Claude Code:

#### Geometric Algebra
- `create_multivector` - Create multivectors from coefficients
- `geometric_product` - Compute geometric products
- `rotor_rotation` - Apply rotor rotations
- `get_cayley_table` - Fast Cayley table lookup/computation

#### Mathematical Operations
- `tropical_matrix_multiply` - Min-plus operations
- `shortest_path` - Graph algorithms via tropical algebra
- `compute_gradient` - Automatic differentiation
- `ca_evolution` - Cellular automata evolution
- `fisher_information` - Information geometry calculations

#### Database Features (if enabled)
- `precompute_cayley_tables` - Precompute essential tables
- `cayley_precompute_status` - View cache status
- `save_computation` / `load_computation` - Result caching

## Production Deployment Options

### Option 1: Local Binary (Recommended for Development)

**Pros:**
- Immediate setup
- Full performance
- No network latency
- Easy debugging

**Setup:**
1. Build release binary
2. Configure Claude Code to use local path
3. Run when needed

### Option 2: systemd Service (Linux)

For always-on local service:

```ini
# /etc/systemd/system/amari-mcp.service
[Unit]
Description=Amari MCP Server
After=network.target postgresql.service

[Service]
Type=simple
User=amari
WorkingDirectory=/opt/amari-mcp
ExecStart=/opt/amari-mcp/amari-mcp --database-url=postgresql://amari:password@localhost/amari_mcp
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
RUN cargo build --release --features database

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl3 ca-certificates
COPY --from=builder /app/target/release/amari-mcp /usr/local/bin/
EXPOSE 3000
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
  http_checks = []
  internal_port = 3000
  processes = ["app"]
  protocol = "tcp"
```

#### AWS Lambda (Serverless)
Not recommended due to MCP's persistent connection requirements.

## Database Setup

### Local PostgreSQL

```bash
# Install PostgreSQL
sudo apt install postgresql postgresql-contrib  # Ubuntu
brew install postgresql                         # macOS

# Create database
sudo -u postgres createdb amari_mcp
sudo -u postgres psql -c "CREATE USER amari WITH PASSWORD 'secure_password';"
sudo -u postgres psql -c "GRANT ALL PRIVILEGES ON DATABASE amari_mcp TO amari;"

# Set connection string
export DATABASE_URL="postgresql://amari:secure_password@localhost/amari_mcp"
```

### Cloud Database

```bash
# Example: Digital Ocean Managed PostgreSQL
export DATABASE_URL="postgresql://user:password@db-cluster.db.ondigitalocean.com:25060/amari_mcp?sslmode=require"
```

## Claude Code Configuration Examples

### Basic Configuration (No Database)
```json
{
  "mcpServers": {
    "amari": {
      "command": "/usr/local/bin/amari-mcp"
    }
  }
}
```

### Full Configuration (With Database & GPU)
```json
{
  "mcpServers": {
    "amari": {
      "command": "/usr/local/bin/amari-mcp",
      "args": [
        "--database-url", "postgresql://user:password@localhost/amari_mcp",
        "--gpu",
        "--log-level", "info"
      ],
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
        "/opt/amari-mcp/amari-mcp --database-url=postgresql://..."
      ]
    }
  }
}
```

## Performance Optimization

### Precompute Cayley Tables
```bash
# One-time setup for fast lookups
./amari-mcp --database-url=$DATABASE_URL precompute-cayley

# Check status
./amari-mcp --database-url=$DATABASE_URL cayley-status
```

### GPU Acceleration
```bash
# Enable GPU features (requires CUDA/ROCm)
./amari-mcp --gpu --database-url=$DATABASE_URL
```

## Troubleshooting

### Connection Issues
1. Check that the binary is executable
2. Verify database connection string
3. Check firewall/port settings
4. Review logs: `RUST_LOG=debug ./amari-mcp`

### Performance Issues
1. Precompute Cayley tables for common signatures
2. Enable GPU acceleration if available
3. Use database caching for expensive computations
4. Monitor memory usage with large computations

### Common Errors
- **"Database not configured"**: Add `--database-url` parameter
- **"GPU acceleration not enabled"**: Add `--gpu` flag and check drivers
- **"Permission denied"**: Check binary permissions and user access

## Security Considerations

### Local Development
- MCP uses stdio transport (secure by default)
- No network exposure
- Database credentials in environment variables

### Production Deployment
- Use strong database passwords
- Enable SSL for database connections
- Run as dedicated user with minimal privileges
- Regular security updates for dependencies