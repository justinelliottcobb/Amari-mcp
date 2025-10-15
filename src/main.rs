use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;
use tracing::{info, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod mcp_stub;
// mod mcp_real;  // Disabled while implementing pmcp
mod mcp_pmcp;
mod server;
mod tools;
mod utils;

#[cfg(feature = "database")]
mod database;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Port to run the MCP server on
    #[arg(short, long, default_value = "3000")]
    port: u16,

    /// Host to bind the server to
    #[arg(long, default_value = "127.0.0.1")]
    host: String,

    /// Enable GPU acceleration (requires GPU feature)
    #[arg(long)]
    gpu: bool,

    /// Database URL for persistent storage (requires database feature)
    #[cfg(feature = "database")]
    #[arg(long)]
    database_url: Option<String>,

    /// Configuration file path
    #[arg(short, long)]
    config: Option<PathBuf>,

    /// Log level
    #[arg(long, default_value = "info")]
    log_level: String,
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

    // Create database pool if configured
    #[cfg(feature = "database")]
    let db_pool = if let Some(ref db_url) = cli.database_url {
        info!("   Database: {}", db_url);
        let pool = sqlx::PgPool::connect(db_url).await?;
        sqlx::migrate!("./migrations").run(&pool).await?;
        info!("   Database connected and migrations applied");
        Some(pool)
    } else {
        None
    };

    // Create and start the MCP server using pmcp
    let server = mcp_pmcp::create_amari_mcp_server(
        cli.gpu,
        #[cfg(feature = "database")]
        db_pool.is_some(),
    ).await?;

    info!("🌐 Starting MCP server with stdio transport");

    // Run the server with stdio transport (MCP standard)
    server.run_stdio().await?;

    Ok(())
}