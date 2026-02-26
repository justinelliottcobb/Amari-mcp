# Amari-MCP Roadmap

## Completed

### v0.2.0 -- Resurrection (Current)

- Config-driven architecture with TOML manifests
- `syn`-based source parser with rayon parallelism
- Phantom-typed `ApiIndex<Unvalidated>` / `ApiIndex<Validated>` lifecycle
- 7 MCP reference tools (api_search, type_info, module_overview, feature_map,
  dependency_graph, browse_docs, usage_examples)
- `check` CLI subcommand for CI integration
- Integration tests against live Amari source (19 crates, 579 modules, 5,796 items)
- 66 tests (60 unit + 6 integration)
- Source path resolution works from any working directory (absolute manifest paths)

## Planned

### v0.3.0 -- Polish and CI Integration

- Add `amari-mcp check` job to Amari's CI workflow to catch API drift
- Snapshot tests with `insta` for regression stability
- Incremental index updates (re-parse only changed files)
- Improved error messages for common manifest misconfiguration

### v0.4.0 -- Sibling Libraries

- Create manifests for Mishima and Minuet libraries
- Tool to auto-generate manifest from a Cargo workspace
- Multi-library mode: serve multiple library indexes from a single server
- Cross-library search (find types across all indexed libraries)

### v0.5.0 -- Enhanced Reference

- Trait implementation discovery (which types implement which traits)
- Type hierarchy visualization
- Method resolution order for complex trait hierarchies
- Generic constraint analysis
- Deprecation tracking via `#[deprecated]` attributes

### Future Considerations

- **Math operation tools**: Restore computational tools (geometric algebra,
  tropical algebra, etc.) as a separate MCP server that depends on Amari
  at the Rust level
- **wgpu integration**: GPU-accelerated batch operations for large-scale
  computations
- **Live index**: Watch mode that re-indexes on source file changes
- **rustdoc JSON**: Explore using nightly rustdoc JSON output as an alternative
  or complement to syn parsing (trades portability for richer type resolution)
