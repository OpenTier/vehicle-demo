pub mod vehicle_msgs {
    include!(concat!(env!("OUT_DIR"), "/vehicle_msgs.rs"));
}

pub mod vehicle_cloud_events {
    include!(concat!(env!("OUT_DIR"), "/vehicle_cloud_events.rs"));
}

pub mod vehicle_commands {
    include!(concat!(env!("OUT_DIR"), "/vehicle_commands.rs"));
}

pub mod state {
    include!(concat!(env!("OUT_DIR"), "/intra.lock_state.rs"));
}

pub mod speed {
    include!(concat!(env!("OUT_DIR"), "/intra.speed.rs"));
}

pub mod tires {
    include!(concat!(env!("OUT_DIR"), "/intra.tire.rs"));
}

pub mod trip_data {
    include!(concat!(env!("OUT_DIR"), "/intra.trip_data.rs"));
}

pub mod battery {
    include!(concat!(env!("OUT_DIR"), "/intra.battery.rs"));
}

pub mod exterior {
    include!(concat!(env!("OUT_DIR"), "/intra.exterior.rs"));
}

pub mod current_location {
    include!(concat!(env!("OUT_DIR"), "/intra.current_location.rs"));
}
