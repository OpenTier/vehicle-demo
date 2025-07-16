// This code was developed by OpenTier GmbH.
use clap::Parser;
use common::topics::*;
use env_logger;
use json5;
use log::info;
use signal_mocker_service::config::SignalOrNestedMessage;
use signal_mocker_service::msg_generators::*;
use signal_mocker_service::{PublicationTaskSpawner, RootConfig};
use std::fs;
use std::sync::Arc;
use std::time::Duration;

#[macro_export]
macro_rules! spawn_generator_task {
    // Base generator without nested generators
    ($generator_type:ident, $message_name:expr, $topic_key:expr, $config:expr, $zenoh_session:expr) => {{
        let generator = $generator_type::new(signal_mocker_service::config::extract_signals(
            &$config.signal_mocker_service.messages[$message_name].signals,
        ));

        // Return the task handle from spawn_task
        PublicationTaskSpawner::spawn_task(
            $zenoh_session,
            $topic_key,
            generator,
            Duration::from_millis($config.signal_mocker_service.messages[$message_name].frequency),
        )
    }};

    // Generator with nested generators
    ($generator_type:ident, $message_name:expr, $topic_key:expr, $config:expr, $zenoh_session:expr, $( $nested_generator:expr ),* ) => {{
        let generator = $generator_type::new(
            signal_mocker_service::config::extract_signals(
                &$config.signal_mocker_service.messages[$message_name].signals
            ),
            $( $nested_generator ),*  // Pass all nested generators to the constructor
        );

        // Return the task handle from spawn_task
        PublicationTaskSpawner::spawn_task(
            $zenoh_session,
            $topic_key,
            generator,
            Duration::from_millis($config.signal_mocker_service.messages[$message_name].frequency),
        )
    }};
}

#[derive(Parser, Clone, PartialEq, Eq, Hash, Debug)]
struct Args {
    // Path to the JSON5 configuration file
    #[arg(short, long)]
    config: String,
}

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // parse args
    let args = Args::parse();

    // Read the JSON5 configuration file
    let config_str = fs::read_to_string(args.config)?;

    // Parse the JSON5 into our Rust structs
    let config: RootConfig = json5::from_str(&config_str)?;

    // Initialize logger
    env_logger::init();

    // create a zenoh session in peer mode
    let zenoh_config = zenoh::Config::default();
    let zenoh_session = Arc::new(zenoh::open(zenoh_config).await.unwrap());

    // spawn publication tasks
    let battery_data_pub_task = spawn_generator_task!(
        BatteryDataGenerator,
        "BatteryData",
        BATTERY_STATE_TOPIC,
        config,
        zenoh_session.clone()
    );

    let exterior_pub_task = spawn_generator_task!(
        ExteriorGenerator,
        "Exterior",
        EXTERIOR_TOPIC,
        config,
        zenoh_session.clone()
    );

    let speed_pub_task = spawn_generator_task!(
        SpeedGenerator,
        "Speed",
        SPEED_TOPIC,
        config,
        zenoh_session.clone()
    );

    let trip_data_pub_task = spawn_generator_task!(
        TripDataGenerator,
        "TripData",
        TRIP_DATA_TOPIC,
        config,
        zenoh_session.clone()
    );

    let current_location_pub_task = spawn_generator_task!(
        CurrentLocationGenerator,
        "CurrentLocation",
        CURRENT_LOCATION_TOPIC,
        config,
        zenoh_session.clone()
    );

    // Extract the nested signals for "FrontTire"
    let front_tire_signals = match config.signal_mocker_service.messages["Tires"]
        .signals
        .get("FrontTire")
    {
        Some(SignalOrNestedMessage::NestedMessage(map)) => map,
        Some(_) => panic!("Expected NestedMessage for FrontTire"),
        None => panic!("No entry found for key 'FrontTire'"),
    };

    // Create the nested generator for "FrontTire"
    let front_tire_generator = TirePressureGenerator::new(
        signal_mocker_service::config::extract_signals(&front_tire_signals),
    );

    // Extract the signals for "RearTire"
    let rear_tire_signals = match config.signal_mocker_service.messages["Tires"]
        .signals
        .get("RearTire")
    {
        Some(SignalOrNestedMessage::NestedMessage(map)) => map,
        Some(_) => panic!("Expected NestedMessage for RearTire"),
        None => panic!("No entry found for key 'RearTire'"),
    };

    // Create the nested generator for "FrontTire"
    let rear_tire_generator = TirePressureGenerator::new(
        signal_mocker_service::config::extract_signals(&rear_tire_signals),
    );

    // Spawn the TiresGenerator task
    let tires_data_pub_task = spawn_generator_task!(
        TiresGenerator,
        "TripData",
        TRIP_DATA_TOPIC,
        config,
        zenoh_session.clone(),
        front_tire_generator,
        rear_tire_generator
    );

    info!("Signal Mocker Service started");

    // wait for tasks to finish
    tokio::try_join!(
        battery_data_pub_task,
        exterior_pub_task,
        speed_pub_task,
        trip_data_pub_task,
        current_location_pub_task,
        tires_data_pub_task
    )?;

    Ok(())
}
