use clap::Parser;
use env_logger;
use std::fs;
use std::sync::Arc;
use twin_service::twin::TwinService;

#[derive(Parser, Clone, PartialEq, Eq, Hash, Debug)]
struct Args {
    // Path to the JSON5 configuration file for the twin service
    #[arg(short, long)]
    twin_config: String,

    // Path to the JSON5 configuration file for the initial vehicle state
    #[arg(short, long)]
    vehicle_state_config: String,
}

#[tokio::main(flavor = "multi_thread", worker_threads = 3)]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // parse args
    let args = Args::parse();

    // Read the JSON5 configuration file
    let twin_config_str = fs::read_to_string(args.twin_config)?;

    // Parse the JSON5 into Rust structs
    let twin_service_config: twin_service::config::TwinServiceConfig =
        json5::from_str(&twin_config_str)?;

    // Read the JSON5 configuration file for the initial vehicle state
    let vehicle_state_config_str = fs::read_to_string(args.vehicle_state_config)?;
    // Parse the JSON5 into Rust structs
    let initial_state: vehicle_msgs::vehicle_msgs::Vehicle =
        json5::from_str(&vehicle_state_config_str)?;

    // initialize logger
    env_logger::init();

    // create a zenoh session
    let config = zenoh::Config::default();
    let session = Arc::new(zenoh::open(config).await.unwrap());

    TwinService::new(twin_service_config, initial_state)
        .run(session)
        .await?;

    Ok(())
}
