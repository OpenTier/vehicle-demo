use std::io::Result;

fn main() -> Result<()> {
    let mut config = prost_build::Config::new();

    config.type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]");
    config.compile_protos(
        &[
            // Vehicle model
            "../../vehicle-cloud-api/proto/vehicle_msgs.proto",
            // Cloud events
            "../../vehicle-cloud-api/proto/vehicle_cloud_events.proto",
            "../../vehicle-cloud-api/proto/vehicle_commands.proto",
            // Intra-vehicle events
            "../../proto/lock_state.proto",
            "../../proto/speed.proto",
            "../../proto/tire.proto",
            "../../proto/trip_data.proto",
            "../../proto/battery.proto",
            "../../proto/exterior.proto",
            "../../proto/current_location.proto",
        ],
        &["../../vehicle-cloud-api/proto", "../../proto/"],
    )?;
    Ok(())
}
