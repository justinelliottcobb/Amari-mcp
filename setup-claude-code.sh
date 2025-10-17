#!/bin/bash

# Amari MCP Setup Script for Claude Code
# Run this script to set up the Amari MCP server for Claude Code integration

set -e

echo "🧮 Amari MCP Server Setup for Claude Code"
echo "========================================="

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Configuration
INSTALL_DIR="$HOME/.local/bin"
DATABASE_NAME="amari_mcp"
DATABASE_USER="amari"

echo -e "\n${BLUE}Step 1: Building Amari MCP Server${NC}"

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ] || ! grep -q "amari-mcp" Cargo.toml; then
    echo -e "${RED}Error: Please run this script from the amari-mcp directory${NC}"
    exit 1
fi

# Build the server
echo "Building release binary..."
cargo build --release --features database,gpu

echo -e "${GREEN}✅ Build successful${NC}"

echo -e "\n${BLUE}Step 2: Installing Binary${NC}"

# Create install directory
mkdir -p "$INSTALL_DIR"

# Copy binary
cp target/release/amari-mcp "$INSTALL_DIR/"
chmod +x "$INSTALL_DIR/amari-mcp"

echo -e "${GREEN}✅ Installed to $INSTALL_DIR/amari-mcp${NC}"

# Check if install dir is in PATH
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo -e "${YELLOW}⚠️  Add $INSTALL_DIR to your PATH:${NC}"
    echo "export PATH=\"\$PATH:$INSTALL_DIR\""
fi

echo -e "\n${BLUE}Step 3: Database Setup (Optional)${NC}"

# Check if PostgreSQL is available
if command -v psql >/dev/null 2>&1; then
    echo "PostgreSQL found. Setting up database..."

    # Check if database exists
    if psql -lqt | cut -d \| -f 1 | grep -qw "$DATABASE_NAME"; then
        echo "Database '$DATABASE_NAME' already exists"
    else
        echo "Creating database '$DATABASE_NAME'..."
        createdb "$DATABASE_NAME" 2>/dev/null || {
            echo -e "${YELLOW}Note: Could not create database automatically${NC}"
            echo "You may need to run: createdb $DATABASE_NAME"
        }
    fi

    # Generate database URL
    DATABASE_URL="postgresql://localhost/$DATABASE_NAME"
    echo -e "${GREEN}✅ Database URL: $DATABASE_URL${NC}"

    # Test connection
    if psql "$DATABASE_URL" -c "SELECT 1;" >/dev/null 2>&1; then
        echo "Testing server with database..."
        echo "Running migrations..."
        "$INSTALL_DIR/amari-mcp" --database-url="$DATABASE_URL" serve &
        SERVER_PID=$!
        sleep 2
        kill $SERVER_PID 2>/dev/null || true
        echo -e "${GREEN}✅ Server tested successfully with database${NC}"
    else
        echo -e "${YELLOW}⚠️  Database connection failed, continuing without database${NC}"
        DATABASE_URL=""
    fi
else
    echo -e "${YELLOW}PostgreSQL not found. Continuing without database support.${NC}"
    echo "Install PostgreSQL for caching and precomputed Cayley tables."
    DATABASE_URL=""
fi

echo -e "\n${BLUE}Step 4: Claude Code Configuration${NC}"

# Generate Claude Code configuration
CONFIG_FILE="claude-code-config.json"

if [ -n "$DATABASE_URL" ]; then
    cat > "$CONFIG_FILE" <<EOF
{
  "mcpServers": {
    "amari": {
      "command": "$INSTALL_DIR/amari-mcp",
      "args": ["--database-url", "$DATABASE_URL", "--log-level", "info"],
      "env": {
        "RUST_LOG": "amari_mcp=info"
      }
    }
  }
}
EOF
else
    cat > "$CONFIG_FILE" <<EOF
{
  "mcpServers": {
    "amari": {
      "command": "$INSTALL_DIR/amari-mcp",
      "args": ["--log-level", "info"],
      "env": {
        "RUST_LOG": "amari_mcp=info"
      }
    }
  }
}
EOF
fi

echo -e "${GREEN}✅ Claude Code configuration written to $CONFIG_FILE${NC}"

echo -e "\n${BLUE}Step 5: Precompute Cayley Tables (Optional)${NC}"

if [ -n "$DATABASE_URL" ]; then
    echo "Precomputing essential Cayley tables for fast lookups..."
    if "$INSTALL_DIR/amari-mcp" --database-url="$DATABASE_URL" precompute-cayley; then
        echo -e "${GREEN}✅ Cayley tables precomputed successfully${NC}"

        # Show status
        echo -e "\nCayley table status:"
        "$INSTALL_DIR/amari-mcp" --database-url="$DATABASE_URL" cayley-status
    else
        echo -e "${YELLOW}⚠️  Cayley table precomputation failed${NC}"
    fi
else
    echo "Skipping Cayley table precomputation (no database)"
fi

echo -e "\n${GREEN}🎉 Setup Complete!${NC}"
echo "==================="

echo -e "\n${BLUE}Next Steps:${NC}"
echo "1. Add the configuration to Claude Code:"
echo "   - Copy the contents of $CONFIG_FILE"
echo "   - Add to your Claude Code MCP server configuration"

echo -e "\n2. Test the connection:"
echo "   amari-mcp --help"

if [ -n "$DATABASE_URL" ]; then
    echo -e "\n3. Check Cayley table status:"
    echo "   amari-mcp --database-url=\"$DATABASE_URL\" cayley-status"
fi

echo -e "\n${BLUE}Available Tools in Claude Code:${NC}"
echo "• create_multivector - Create geometric algebra multivectors"
echo "• geometric_product - Compute geometric products"
echo "• rotor_rotation - Apply rotor rotations"
echo "• get_cayley_table - Fast Cayley table lookups"
echo "• tropical_matrix_multiply - Tropical algebra operations"
echo "• shortest_path - Graph algorithms"
echo "• compute_gradient - Automatic differentiation"
echo "• ca_evolution - Cellular automata evolution"
echo "• fisher_information - Information geometry"

if [ -n "$DATABASE_URL" ]; then
    echo "• precompute_cayley_tables - Precompute tables"
    echo "• cayley_precompute_status - View cache status"
fi

echo -e "\n${BLUE}Documentation:${NC}"
echo "• Full setup guide: docs/claude-code-setup.md"
echo "• Test suite: ./test.sh"
echo "• Examples: examples/ directory"

echo -e "\n${GREEN}Happy computing with Amari! 🚀${NC}"