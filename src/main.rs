use clap::Parser;
use tracing::{info, error};
use tracing_subscriber::FmtSubscriber;

/// LLM-assisted writing tool
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to a config file
    #[arg(long, value_name = "FILE")]
    config: Option<String>,
}

fn main() {
    // Initialize tracing subscriber
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let args = Args::parse();

    // Extract and handle arguments
    if let Some(config) = args.config {
        // Do something with the config file path
        info!("Using custom config file: {}", config);
    }

    // Some example logging
    info!("Starting super-barnacle application");
    // Simulate an error
    if let Err(err) = simulate_error() {
        error!("An error occurred: {}", err);
    }
    info!("Exiting super-barnacle application");
}

fn simulate_error() -> Result<(), &'static str> {
    // Simulate an error
    Err("Something went wrong")
}
