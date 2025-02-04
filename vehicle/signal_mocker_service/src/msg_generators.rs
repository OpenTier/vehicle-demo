// This code was developed by OPENTIER - FZCO.
use crate::config::*;
use crate::define_generator;
use crate::generators::{MessageGenerator, SignalGenerator};
use chrono::Local;
use std::collections::HashMap;
use vehicle_msgs::battery::BatteryData;
use vehicle_msgs::current_location::CurrentLocation;
use vehicle_msgs::exterior::Exterior;
use vehicle_msgs::speed::Speed;
use vehicle_msgs::tires::{TirePressure, Tires};
use vehicle_msgs::trip_data::TripData;

// BatteryDataGenerator
define_generator!(BatteryDataGenerator);
impl MessageGenerator<BatteryData> for BatteryDataGenerator {
    fn generate(&mut self) -> BatteryData {
        let is_charging = self
            .signal_generator
            .get_next_signal_bool("IsCharging")
            .unwrap_or(false);
        let is_discharging = self
            .signal_generator
            .get_next_signal_bool("IsDischarging")
            .unwrap_or(false);
        let time_to_fully_charge = self
            .signal_generator
            .get_next_signal_value("TimeToFullyCharge")
            .unwrap_or(0.0) as u32;
        let estimated_range = self
            .signal_generator
            .get_next_signal_value("EstimatedRange")
            .unwrap_or(0.0) as u32;
        let battery_level = self
            .signal_generator
            .get_next_signal_value("BatteryLevel")
            .unwrap_or(0.0) as f32;
        let state_of_health = self
            .signal_generator
            .get_next_signal_value("StateOfHealth")
            .unwrap_or(0.0) as f32;
        let temperature = self
            .signal_generator
            .get_next_signal_value("Temperature")
            .unwrap_or(0.0) as f32;

        BatteryData {
            is_charging,
            is_discharging,
            time_to_fully_charge,
            estimated_range,
            battery_level,
            state_of_health,
            temperature,
        }
    }
}

// ExteriorGenerator
define_generator!(ExteriorGenerator);
impl MessageGenerator<Exterior> for ExteriorGenerator {
    fn generate(&mut self) -> Exterior {
        let air_temperature = self
            .signal_generator
            .get_next_signal_value("AirTemperature")
            .unwrap_or(0.0) as f32;
        let humidity = self
            .signal_generator
            .get_next_signal_value("Humidity")
            .unwrap_or(0.0) as f32;
        let light_intensity = self
            .signal_generator
            .get_next_signal_value("LightIntensity")
            .unwrap_or(0.0) as f32;

        Exterior {
            air_temperature,
            humidity,
            light_intensity,
        }
    }
}

// SpeedGenerator
define_generator!(SpeedGenerator);
impl MessageGenerator<Speed> for SpeedGenerator {
    fn generate(&mut self) -> Speed {
        let value = self
            .signal_generator
            .get_next_signal_value("Value")
            .unwrap_or(0.0) as f32;

        Speed { value }
    }
}

// TirePressureGenerator
define_generator!(TirePressureGenerator);
impl MessageGenerator<TirePressure> for TirePressureGenerator {
    fn generate(&mut self) -> TirePressure {
        let is_pressure_low = self
            .signal_generator
            .get_next_signal_bool("IsPressureLow")
            .unwrap_or(false);
        let pressure = self
            .signal_generator
            .get_next_signal_value("Pressure")
            .unwrap_or(0.0) as u32;
        let temperature = self
            .signal_generator
            .get_next_signal_value("Temperature")
            .unwrap_or(0.0) as f32;

        TirePressure {
            is_pressure_low,
            pressure,
            temperature,
        }
    }
}

// Tires
define_generator!(
    TiresGenerator,
    front_tire_generator: TirePressureGenerator,
    rear_tire_generator: TirePressureGenerator
);
impl MessageGenerator<Tires> for TiresGenerator {
    fn generate(&mut self) -> Tires {
        let front_tire = Some(self.front_tire_generator.generate());
        let rear_tire = Some(self.rear_tire_generator.generate());

        Tires {
            front_tire,
            rear_tire,
        }
    }
}

// TripDataGenerator
define_generator!(TripDataGenerator);

impl MessageGenerator<TripData> for TripDataGenerator {
    fn generate(&mut self) -> TripData {
        let start_time = Local::now().to_rfc3339();

        let traveled_distance = self
            .signal_generator
            .get_next_signal_value("TraveledDistance")
            .unwrap_or(0.0) as f32;

        let traveled_distance_since_start = self
            .signal_generator
            .get_next_signal_value("TraveledDistanceSinceStart")
            .unwrap_or(0.0) as f32;

        let trip_duration = self
            .signal_generator
            .get_next_signal_value("TripDuration")
            .unwrap_or(0.0) as f32;

        let trip_meter_reading = self
            .signal_generator
            .get_next_signal_value("TripMeterReading")
            .unwrap_or(0.0) as f32;

        let average_speed = self
            .signal_generator
            .get_next_signal_value("AverageSpeed")
            .unwrap_or(0.0) as f32;

        TripData {
            traveled_distance,
            traveled_distance_since_start,
            trip_duration,
            trip_meter_reading,
            average_speed,
            start_time,
        }
    }
}

// CurrentLocationGenerator
define_generator!(CurrentLocationGenerator);
impl MessageGenerator<CurrentLocation> for CurrentLocationGenerator {
    fn generate(&mut self) -> CurrentLocation {
        let altitude = self
            .signal_generator
            .get_next_signal_value("Altitude")
            .unwrap_or(0.0);

        let latitude = self
            .signal_generator
            .get_next_signal_value("Latitude")
            .unwrap_or(0.0);

        let longitude = self
            .signal_generator
            .get_next_signal_value("Longitude")
            .unwrap_or(0.0);

        let timestamp = Local::now().to_rfc3339();

        CurrentLocation {
            altitude,
            latitude,
            longitude,
            timestamp,
        }
    }
}
