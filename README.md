# Amari MCP Server

Config-driven MCP server that provides ground-truth API reference for Rust
library ecosystems. Eliminates hallucinations by parsing source code directly
with `syn` and serving accurate type signatures, documentation, and module
structure via the Model Context Protocol.

Currently configured for the [Amari](https://github.com/justinelliottcobb/Amari)
mathematical computing library (21 crates, ~161K lines), but designed to work
with any Rust workspace via a TOML manifest file.

## Quick Start

```bash
# Build
cargo build --release

# Validate that the library source is parseable
cargo run --release -- check

# Start the MCP server
cargo run --release -- serve
```

### Configure Claude Code

Add to your Claude Code MCP settings (use absolute paths):

```json
{
  "amari-mcp": {
    "command": "/absolute/path/to/amari-mcp/target/release/amari-mcp",
    "args": ["--manifest", "/absolute/path/to/amari-mcp/manifests/amari.toml"]
  }
}
```

Or add it via the CLI:

```bash
claude mcp add amari-mcp -- /path/to/amari-mcp/target/release/amari-mcp \
  --manifest /path/to/amari-mcp/manifests/amari.toml
```

The `--manifest` path must be absolute since the server may be launched from
any working directory.

## MCP Tools

| Tool | Description |
|------|-------------|
| `api_search` | Search types, functions, traits by name with kind/crate filters |
| `type_info` | Full type details: signature, fields, methods, trait impls, docs |
| `module_overview` | List all public items in a crate or module |
| `feature_map` | Which Cargo features enable which crates and types |
| `dependency_graph` | Inter-crate dependency relationships |
| `browse_docs` | Module-level and item-level documentation |
| `usage_examples` | Extract code examples from doc comments |

## CLI

```
amari-mcp [OPTIONS] [COMMAND]

Commands:
  serve   Start the MCP server (default)
  check   Validate that the manifest and source are parseable

Options:
  -m, --manifest <PATH>  Path to library manifest [default: manifests/amari.toml]
      --log-level <LVL>  Log level [default: info]
```

### Check Mode

The `check` subcommand parses all configured crates and reports statistics:

```
$ cargo run -- check
Library: amari
Parsed 19 crates, 579 modules, 5796 items
  amari-core (312 items)
  amari-tropical (129 items)
  amari-dual (169 items)
  ...
  amari-dynamics (973 items) [feature: dynamics]

Check passed.
```

Use this in CI to catch API drift between the library and the MCP server.

## Indexing a Different Library

Create a manifest file describing your library's workspace:

```toml
[library]
name = "mylib"
display_name = "My Library"
version = "1.0.0"
description = "Description of the library"
source_path = "../path/to/mylib"  # relative to this manifest file

[workspace]
root_cargo_toml = "Cargo.toml"
umbrella_crate = "src/lib.rs"

[crates.default]
members = ["mylib-core", "mylib-utils"]

[crates.optional]
gpu = "mylib-gpu"

[crates.internal]
members = []

[aliases]
mylib-core = "core"
```

Then run:

```bash
amari-mcp --manifest manifests/mylib.toml serve
```

See [DESIGN.md](DESIGN.md) for the full manifest format specification.

## Development

```bash
# Run all tests (66 tests: 60 unit + 6 integration)
cargo test

# Clippy with warnings as errors
cargo clippy -- -D warnings

# Format check
cargo fmt --check
```

Pre-commit hooks enforce fmt, clippy, and test on every commit.

## Architecture

```
manifest (TOML) --> parser (syn) --> ApiIndex<Validated> --> MCP tools --> Claude Code
```

- **Config module**: Loads TOML manifests, resolves paths, maps crates to features
- **Parser**: `syn`-based AST walking with rayon parallelism (~1s for 19 crates)
- **Index**: Phantom-typed state machine (`Unvalidated` -> `Validated`)
- **Tools**: 7 MCP handlers sharing `Arc<SharedState>` over the validated index

See [DESIGN.md](DESIGN.md) for detailed architecture and [ROADMAP.md](ROADMAP.md)
for planned work.

## License

MIT OR Apache-2.0

## Related Projects

- [Amari](https://github.com/justinelliottcobb/Amari) - Advanced mathematical computing library
- [Model Context Protocol](https://modelcontextprotocol.io/) - MCP specification
- [pmcp](https://crates.io/crates/pmcp) - Rust MCP SDK
