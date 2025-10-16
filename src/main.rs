use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;
use tracing::{info, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// mod mcp_stub;  // Legacy stub implementation, replaced by pmcp
// mod mcp_real;  // Disabled while implementing pmcp
mod mcp_pmcp;
// mod server;    // Legacy server implementation, replaced by pmcp
mod tools;
mod utils;

// Database module removed - MCP servers should be simple and stateless

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,

    /// Port to run the MCP server on
    #[arg(short, long, default_value = "3000")]
    port: u16,

    /// Host to bind the server to
    #[arg(long, default_value = "127.0.0.1")]
    host: String,

    /// Enable GPU acceleration (requires GPU feature)
    #[arg(long)]
    gpu: bool,

    /// Configuration file path
    #[arg(short, long)]
    config: Option<PathBuf>,

    /// Log level
    #[arg(long, default_value = "info")]
    log_level: String,
}

#[derive(Parser)]
enum Command {
    /// Start the MCP server (default)
    Serve,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("amari_mcp={}", cli.log_level).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("🚀 Starting Amari MCP Server");
    info!("   Host: {}", cli.host);
    info!("   Port: {}", cli.port);

    if cli.gpu {
        info!("   GPU acceleration: enabled");
    } else {
        warn!("   GPU acceleration: disabled (use --gpu to enable)");
    }

    // Database support removed - MCP servers should be simple and stateless

    // Handle subcommands
    match cli.command.as_ref().unwrap_or(&Command::Serve) {
        Command::Serve => {
            // Create and start the MCP server using pmcp
            let server = mcp_pmcp::create_amari_mcp_server(cli.gpu).await?;

            info!("Starting MCP server with stdio transport");
            info!("Cayley tables will use on-demand computation");

            // Run the server with stdio transport (MCP standard)
            server.run_stdio().await?;
        }
    }

    Ok(())
}
