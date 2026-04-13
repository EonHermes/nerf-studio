//! NeRF Studio Server
//! 
//! Main entry point for the NeRF Studio web application.

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser)]
#[command(name = "nerf-studio")]
#[command(about = "Neural Radiance Fields Studio - Create 3D scenes from photos")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    
    /// Port to run the server on
    #[arg(short, long, default_value = "3000")]
    port: u16,
    
    /// Database URL
    #[arg(long, default_value = "sqlite:nerf_studio.db?mode=rwc")]
    database_url: String,
    
    /// Directory for uploaded images
    #[arg(long, default_value = "./uploads")]
    uploads_dir: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Run the web server
    Serve {
        /// Port to listen on
        #[arg(short, long, default_value = "3000")]
        port: u16,
    },
    
    /// Initialize the database
    Init {
        /// Database URL
        #[arg(long, default_value = "sqlite:nerf_studio.db?mode=rwc")]
        database_url: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "nerf_studio=info,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Serve { port }) => {
            run_server(port, &cli.database_url, &cli.uploads_dir).await?;
        }
        Some(Commands::Init { database_url }) => {
            init_database(&database_url).await?;
        }
        None => {
            // Default to serve mode
            run_server(cli.port, &cli.database_url, &cli.uploads_dir).await?;
        }
    }

    Ok(())
}

async fn run_server(port: u16, database_url: &str, uploads_dir: &str) -> Result<()> {
    info!("Starting NeRF Studio server on port {}", port);
    
    let state = nerf_studio::AppState::new(database_url, uploads_dir).await?;
    let app = nerf_studio::create_router(state);
    
    let addr = format!("0.0.0.0:{}", port);
    info!("Listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}

async fn init_database(database_url: &str) -> Result<()> {
    info!("Initializing database: {}", database_url);
    
    let pool = sqlx::SqlitePool::connect(database_url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    
    info!("Database initialized successfully");
    Ok(())
}
