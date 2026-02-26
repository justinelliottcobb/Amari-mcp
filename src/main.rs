use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser)]
#[command(
    author,
    version,
    about = "Config-driven MCP server for Rust library API reference"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,

    /// Path to library manifest file
    #[arg(short, long, default_value = "manifests/amari.toml")]
    manifest: PathBuf,

    /// Log level
    #[arg(long, default_value = "info")]
    log_level: String,
}

#[derive(Parser)]
enum Command {
    /// Start the MCP server (default)
    Serve,
    /// Validate that the manifest and source are parseable
    Check,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logging — stderr only so stdout is clean for MCP JSON-RPC
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("amari_mcp={}", cli.log_level).into()),
        )
        .with(tracing_subscriber::fmt::layer().with_writer(std::io::stderr))
        .init();

    info!("Loading manifest from {:?}", cli.manifest);
    let manifest = amari_mcp::config::LibraryManifest::load(&cli.manifest)?;
    info!("Loaded manifest for {}", manifest.library.display_name);

    match cli.command.as_ref().unwrap_or(&Command::Serve) {
        Command::Serve => {
            let index = amari_mcp::parser::build_index(&manifest, &cli.manifest)?;
            let validated = index.validate()?;
            info!("Index validated successfully");

            amari_mcp::mcp_pmcp::create_mcp_server(validated, manifest, cli.manifest.clone())
                .await?;
        }
        Command::Check => {
            let index = amari_mcp::parser::build_index(&manifest, &cli.manifest)?;
            let parse_error_count = index.parse_errors.len();

            match index.validate() {
                Ok(validated) => {
                    let stats = validated.stats();
                    println!("Library: {}", validated.library_name);
                    println!(
                        "Parsed {} crates, {} modules, {} items",
                        stats.crate_count, stats.module_count, stats.item_count
                    );
                    for crate_info in &validated.crates {
                        let item_count: usize = count_crate_items(&crate_info.modules);
                        let feature_tag = crate_info
                            .feature_gate
                            .as_ref()
                            .map(|f| format!(" [feature: {f}]"))
                            .unwrap_or_default();
                        println!("  {} ({} items){feature_tag}", crate_info.name, item_count);
                    }
                    if parse_error_count > 0 {
                        println!("\n{parse_error_count} parse warning(s) (index still usable)");
                    }
                    println!("\nCheck passed.");
                }
                Err(report) => {
                    for error in &report.errors {
                        eprintln!("ERROR: {error}");
                    }
                    std::process::exit(1);
                }
            }
        }
    }

    Ok(())
}

fn count_crate_items(modules: &[amari_mcp::parser::index::ModuleInfo]) -> usize {
    modules
        .iter()
        .map(|m| m.items.len() + count_crate_items(&m.submodules))
        .sum()
}
