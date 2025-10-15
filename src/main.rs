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

#[derive(Parser)]
enum Command {
    /// Start the MCP server (default)
    Serve,

    /// Precompute and store essential Cayley tables
    #[cfg(feature = "database")]
    PrecomputeCayley {
        /// Force recomputation even if tables exist
        #[arg(long)]
        force: bool,
    },

    /// Show Cayley table precomputation status
    #[cfg(feature = "database")]
    CayleyStatus,

    /// Clear all precomputed Cayley tables
    #[cfg(feature = "database")]
    CayleyClear {
        /// Skip confirmation prompt
        #[arg(long)]
        yes: bool,
    },
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

    // Handle subcommands
    match cli.command.as_ref().unwrap_or(&Command::Serve) {
        Command::Serve => {
            // Set up database pool for Cayley table lookups if available
            #[cfg(feature = "database")]
            if let Some(pool) = &db_pool {
                tools::cayley_tables::set_database_pool(pool.clone());
            }

            // Create and start the MCP server using pmcp
            let server = mcp_pmcp::create_amari_mcp_server(
                cli.gpu,
                #[cfg(feature = "database")]
                db_pool.is_some(),
            ).await?;

            info!("🌐 Starting MCP server with stdio transport");
            #[cfg(feature = "database")]
            info!("💡 Cayley tables will use {} lookup",
                if db_pool.is_some() { "ZERO-LATENCY database" } else { "on-demand computation" });

            #[cfg(not(feature = "database"))]
            info!("💡 Cayley tables will use on-demand computation");

            // Run the server with stdio transport (MCP standard)
            server.run_stdio().await?;
        }

        #[cfg(feature = "database")]
        Command::PrecomputeCayley { force } => {
            if let Some(pool) = db_pool {
                info!("🧮 Starting Cayley table precomputation (force: {})", force);
                let result = tools::cayley_precompute::precompute_essential_tables(&pool).await?;
                println!("{}", serde_json::to_string_pretty(&result)?);
            } else {
                return Err(anyhow::anyhow!("Database URL required for precomputation. Use --database-url"));
            }
        }

        #[cfg(feature = "database")]
        Command::CayleyStatus => {
            if let Some(pool) = db_pool {
                info!("📊 Getting Cayley table precomputation status");
                let result = tools::cayley_precompute::get_precomputation_status(&pool).await?;
                println!("{}", serde_json::to_string_pretty(&result)?);
            } else {
                return Err(anyhow::anyhow!("Database URL required for status check. Use --database-url"));
            }
        }

        #[cfg(feature = "database")]
        Command::CayleyClear { yes } => {
            if let Some(pool) = db_pool {
                if !yes {
                    print!("Are you sure you want to clear all precomputed Cayley tables? [y/N]: ");
                    use std::io::{self, Write};
                    io::stdout().flush()?;
                    let mut input = String::new();
                    io::stdin().read_line(&mut input)?;
                    if !input.trim().to_lowercase().starts_with('y') {
                        info!("Cancelled");
                        return Ok(());
                    }
                }
                info!("🗑️  Clearing all precomputed Cayley tables");
                let result = tools::cayley_precompute::clear_precomputed_tables(&pool).await?;
                println!("{}", serde_json::to_string_pretty(&result)?);
            } else {
                return Err(anyhow::anyhow!("Database URL required for clearing tables. Use --database-url"));
            }
        }
    }

    Ok(())
}