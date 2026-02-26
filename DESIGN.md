# Amari-MCP Design Document

## Overview

Amari-MCP is a config-driven MCP (Model Context Protocol) server that provides
ground-truth API reference for Rust library ecosystems. It parses source code
directly using `syn` and serves accurate type signatures, documentation, and
module structure to Claude Code via JSON-RPC over stdio.

The server is **library-agnostic**: point it at any Rust workspace via a TOML
manifest file and it will index the full public API surface.

## Problem Statement

When Claude Code works with large Rust libraries like Amari (21 crates, ~161K
lines), it frequently hallucinates API details: inventing methods that don't
exist, guessing wrong type signatures, and missing feature gates. A traditional
approach of including library source in context is too expensive at this scale.

Amari-MCP solves this by building a searchable index of the actual public API
and exposing it through MCP tools that Claude Code can call on demand.

## Architecture

```
manifest (TOML) --> parser (syn) --> ApiIndex<Validated> --> MCP tools --> Claude Code
```

### Key Design Decisions

1. **Amari is NOT a Rust dependency.** We parse source files from disk using
   `syn`. This avoids version coupling and means the MCP server never needs to
   be rebuilt when the target library changes -- just re-index.

2. **Config-driven via TOML manifests.** A manifest file describes the target
   library's workspace structure, crate membership, feature gates, and aliases.
   Swap the manifest to index a different library.

3. **Phantom-typed index lifecycle.** `ApiIndex<Unvalidated>` is produced by
   the parser; `ApiIndex<Validated>` is produced by `validate()`. MCP tool
   handlers require `ApiIndex<Validated>`, so the type system enforces that
   tools only operate on successfully-parsed data.

4. **Rayon-parallelized parsing.** Workspace crates are parsed in parallel via
   `rayon::par_iter()`. For Amari's 19 user-facing crates, the full index
   builds in under 1 second.

5. **No math operation tools.** This server focuses purely on API reference and
   development tooling. Math operations belong in the library itself or in
   future dedicated MCP servers.

## Manifest Format

Manifests live in `manifests/` and describe a target library:

```toml
[library]
name = "amari"
display_name = "Amari"
version = "0.18.1"
source_path = "../../amari"  # relative to manifest file

[workspace]
root_cargo_toml = "Cargo.toml"
umbrella_crate = "src/lib.rs"

[crates.default]
members = ["amari-core", "amari-tropical", ...]

[crates.optional]
measure = "amari-measure"
topology = "amari-topology"

[crates.internal]
members = ["amari-flynn-macros", "amari-wasm"]

[aliases]
amari-core = "core"
amari-tropical = "tropical"
```

- **default**: Always-available crates (no feature gate)
- **optional**: Feature-gated crates (key = feature name, value = crate dir)
- **internal**: Proc-macro crates and other non-user-facing crates (excluded)
- **aliases**: How the umbrella crate re-exports each sub-crate

## Parser Architecture

```
src/parser/
    mod.rs           -- build_index(manifest, manifest_path) with rayon parallelism
    workspace.rs     -- Cargo.toml parsing, dependency graph
    module_tree.rs   -- recursive mod declaration walking
    items.rs         -- syn::Visit-based item extraction
    docs.rs          -- doc comment extraction from syn attributes
    features.rs      -- #[cfg(feature)] gate extraction
    index.rs         -- ApiIndex, CrateInfo, ModuleInfo, ApiItem types
    display.rs       -- output formatting helpers
```

### Item Extraction

The core of the parser uses `syn`'s visitor pattern to walk parsed ASTs and
extract public items. Signatures are rendered back to strings via
`quote::ToTokens`, which faithfully preserves const generics, where clauses,
and lifetimes.

Extracted item kinds (the `ItemKind` sum type):
- Functions (async, unsafe)
- Structs (named fields, tuple, unit; with const generics)
- Enums (with variant info)
- Traits (with supertraits)
- Type aliases
- Constants
- Impl blocks (inherent and trait impls, pub methods)
- Re-exports (`pub use`)

### Module Tree Discovery

Starting from each crate's `src/lib.rs`, the parser follows `mod` declarations
to build the full module tree. It handles:
- `mod foo;` resolved to `foo.rs` or `foo/mod.rs`
- `#[path = "..."]` attributes
- `#[cfg(feature = "...")]` on module declarations
- Inline `mod foo { ... }` blocks

## MCP Tools

| Tool | Purpose |
|------|---------|
| `api_search` | Search types/functions by name, filter by kind/crate |
| `type_info` | Full type details: signature, fields, methods, trait impls |
| `module_overview` | List public items in a module with brief descriptions |
| `feature_map` | Which Cargo features enable which crates/types |
| `dependency_graph` | Inter-crate dependency relationships |
| `browse_docs` | Module-level and item-level documentation |
| `usage_examples` | Extract code examples from doc comments |

All tools hold `Arc<SharedState>` containing the validated index and manifest.
Built once at startup, read-only during serving.

## Transferability

To index a different Rust library:

1. Create a new manifest file (e.g., `manifests/mishima.toml`)
2. Fill in the library info, crate membership, and aliases
3. Run: `amari-mcp --manifest manifests/mishima.toml serve`

No code changes required. The parser, index, and tools are fully generic.

## Testing Strategy

- **Unit tests** (60): Inline in each module, written test-first per TDD
- **Integration tests** (6): Run against live Amari source, verify real-world
  parsing of 19 crates / 579 modules / 5,796 items
- **check mode**: CLI subcommand that builds and validates the index, printing
  per-crate statistics. Suitable for CI integration.
