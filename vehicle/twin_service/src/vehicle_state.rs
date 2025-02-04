use std::fmt;

use log::error;
use vehicle_msgs::battery::BatteryData;
use vehicle_msgs::current_location::CurrentLocation;
use vehicle_msgs::exterior::Exterior;
use vehicle_msgs::speed::Speed;
use vehicle_msgs::state::LockState;
use vehicle_msgs::tires::Tires;
use vehicle_msgs::trip_data::TripData;
use vehicle_msgs::vehicle_cloud_events::TirePressure;
use vehicle_msgs::vehicle_cloud_events::{
    BatteryEvent, CurrentLocationEvent, ExteriorEvent, SpeedEvent, SystemStateEvent,
    TirePressureEvent, TripDataEvent,
};
use vehicle_msgs::vehicle_msgs::{
    Vehicle, VehicleChassisAxleRow1WheelLeftTire, VehicleChassisAxleRow2WheelLeftTire,
    VehicleCurrentLocation, VehicleExterior,
};

pub trait VehicleMessage {
    fn update_state(self, state: &mut Vehicle);
}

// VehicleMessage implementations
impl VehicleMessage for BatteryData {
    fn update_state(self, state: &mut Vehicle) {
        if let Some(powertrain) = &mut state.powertrain {
            if let Some(traction_battery) = &mut powertrain.traction_battery {
                if let Some(charging) = &mut traction_battery.charging {
                    // Update charging-related fields
                    charging.is_charging = self.is_charging;
                    charging.is_discharging = self.is_discharging;
                    charging.time_to_complete = self.time_to_fully_charge;
                } else {
                    error!("Missing 'charging' field in traction_battery");
                }

                // Update estimated range
                traction_battery.range = self.estimated_range;

                if let Some(state_of_charge) = &mut traction_battery.state_of_charge {
                    // Update state of charge
                    state_of_charge.displayed = self.battery_level;
                } else {
                    error!("Missing 'state_of_charge' field in traction_battery");
                }

                // Update state of health
                traction_battery.state_of_health = self.state_of_health;

                if let Some(temperature) = &mut traction_battery.temperature {
                    // Update temperature
                    temperature.average = self.temperature;
                } else {
                    error!("Missing 'temperature' field in traction_battery");
                }
            } else {
                error!("Missing 'traction_battery' field in powertrain");
            }
        } else {
            error!("Missing 'powertrain' field in Vehicle");
        }
    }
}

impl VehicleMessage for CurrentLocation {
    fn update_state(self, state: &mut Vehicle) {
        state.current_location = Some(VehicleCurrentLocation {
            latitude: self.latitude,
            longitude: self.longitude,
            altitude: self.altitude,
            ..state.current_location.clone().unwrap_or_default()
        });
    }
}

impl VehicleMessage for Exterior {
    fn update_state(self, state: &mut Vehicle) {
        state.exterior = Some(VehicleExterior {
            air_temperature: self.air_temperature,
            humidity: self.humidity,
            light_intensity: self.light_intensity,
        })
    }
}

impl VehicleMessage for Speed {
    fn update_state(self, state: &mut Vehicle) {
        state.speed = self.value;
    }
}

impl VehicleMessage for Tires {
    fn update_state(self, state: &mut Vehicle) {
        // Ensure row1 and row2 exist
        if let Some(chassis) = &mut state.chassis {
            if let Some(axle) = &mut chassis.axle {
                // Handle front tire (row1)
                if let Some(row1) = &mut axle.row1 {
                    if let Some(wheel) = &mut row1.wheel {
                        if let Some(left_wheel) = &mut wheel.left {
                            if let Some(tire) = &mut left_wheel.tire {
                                if let Some(front_tire) = &self.front_tire {
                                    tire.pressure = front_tire.pressure;
                                    tire.temperature = front_tire.temperature;
                                    tire.is_pressure_low = front_tire.is_pressure_low;
                                }
                            } else {
                                // Initialize tire if it's None
                                left_wheel.tire = Some(VehicleChassisAxleRow1WheelLeftTire {
                                    pressure: self.front_tire.unwrap_or_default().pressure,
                                    temperature: self.front_tire.unwrap_or_default().temperature,
                                    is_pressure_low: self
                                        .front_tire
                                        .unwrap_or_default()
                                        .is_pressure_low,
                                });
                            }
                        }
                    }
                }

                // Handle rear tire (row2)
                if let Some(row2) = &mut axle.row2 {
                    if let Some(wheel) = &mut row2.wheel {
                        if let Some(left_wheel) = &mut wheel.left {
                            if let Some(tire) = &mut left_wheel.tire {
                                if let Some(rear_tire) = &self.rear_tire {
                                    tire.pressure = rear_tire.pressure;
                                    tire.temperature = rear_tire.temperature;
                                    tire.is_pressure_low = rear_tire.is_pressure_low;
                                }
                            } else {
                                // Initialize tire if it's None
                                left_wheel.tire = Some(VehicleChassisAxleRow2WheelLeftTire {
                                    pressure: self.rear_tire.unwrap_or_default().pressure,
                                    temperature: self.rear_tire.unwrap_or_default().temperature,
                                    is_pressure_low: self
                                        .rear_tire
                                        .unwrap_or_default()
                                        .is_pressure_low,
                                });
                            }
                        } else {
                            error!("Wheel is not initialized");
                        }
                    } else {
                        error!("Wheel is not initialized");
                    }
                } else {
                    error!("Row2 is not initialized");
                }
            } else {
                error!("Axle is not initialized");
            }
        } else {
            error!("Chassis is not initialized");
        }
    }
}

impl VehicleMessage for TripData {
    fn update_state(self, state: &mut Vehicle) {
        state.start_time = self.start_time;
        state.traveled_distance = self.traveled_distance;
        state.traveled_distance_since_start = self.traveled_distance_since_start;
        state.trip_duration = self.trip_duration;
        state.trip_meter_reading = self.trip_meter_reading;
        state.average_speed = self.average_speed;
    }
}

impl VehicleMessage for LockState {
    fn update_state(self, state: &mut Vehicle) {
        let system_state_str = match self.state {
            0 => "UNDEFINED",
            1 => "LOCK",
            2 => "OFF",
            3 => "ACC",
            4 => "ON",
            5 => "START",
            _ => "UNDEFINED",
        };

        state.low_voltage_system_state = system_state_str.to_string();
    }
}

pub enum LowVoltageSystemState {
    UNDEFINED,
    LOCK,
    OFF,
    ACC,
    ON,
    START,
}

impl fmt::Display for LowVoltageSystemState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            LowVoltageSystemState::UNDEFINED => "UNDEFINED",
            LowVoltageSystemState::LOCK => "LOCK",
            LowVoltageSystemState::OFF => "OFF",
            LowVoltageSystemState::ACC => "ACC",
            LowVoltageSystemState::ON => "ON",
            LowVoltageSystemState::START => "START",
        };
        write!(f, "{}", s)
    }
}

// Vehicle State
#[derive(Debug, Default)]
pub struct VehicleState {
    pub vehicle: Vehicle,
    pub vehicle_id: String,
}

impl VehicleState {
    pub async fn update<C: VehicleMessage + Send + 'static>(&mut self, component: C) {
        component.update_state(&mut self.vehicle);
    }

    pub fn change_state(&mut self, new_state: LowVoltageSystemState) {
        // TODO: Implement the logic to verify if the state can be changed?
        self.vehicle.low_voltage_system_state = new_state.to_string();
    }

    pub fn is_valid_command(&self, command: &VehicleCommand) -> bool {
        // TODO: for now always return true
        match command {
            VehicleCommand::Lock => true,
            VehicleCommand::Unlock => true,
            VehicleCommand::LightOn => true,
            VehicleCommand::LightOff => true,
            VehicleCommand::HornOn => true,
            VehicleCommand::HornOff => true,
            VehicleCommand::EngineOn => true,
            VehicleCommand::EngineOff => true,
        }
    }

    pub fn vehicle_id(&self) -> String {
        self.vehicle_id.clone()
    }

    pub fn to_battery_event(&self) -> Option<BatteryEvent> {
        if let Some(powertrain) = &self.vehicle.powertrain {
            if let Some(traction_battery) = &powertrain.traction_battery {
                let battery_event = BatteryEvent {
                    vehicle_id: self.vehicle_id(),
                    battery_level: traction_battery.state_of_charge.as_ref()?.displayed,
                    is_charging: traction_battery.charging.as_ref()?.is_charging,
                    is_discharging: traction_battery.charging.as_ref()?.is_discharging,
                    time_to_fully_charge: traction_battery.charging.as_ref()?.time_to_complete,
                    estimated_range: traction_battery.range,
                    state_of_health: traction_battery.state_of_health,
                    temperature: traction_battery.temperature.as_ref()?.average,
                };
                return Some(battery_event);
            }
        }
        error!("Failed to extract battery data from the vehicle state.");
        None
    }

    pub fn to_tires_event(&self) -> Option<TirePressureEvent> {
        if let Some(chassis) = &self.vehicle.chassis {
            if let Some(axle) = &chassis.axle {
                let front_tire = axle
                    .row1
                    .as_ref()?
                    .wheel
                    .as_ref()?
                    .left
                    .as_ref()?
                    .tire
                    .as_ref()?;
                let rear_tire = axle
                    .row2
                    .as_ref()?
                    .wheel
                    .as_ref()?
                    .left
                    .as_ref()?
                    .tire
                    .as_ref()?;

                return Some(TirePressureEvent {
                    vehicle_id: self.vehicle_id(),
                    front_tire: Some(TirePressure {
                        is_pressure_low: front_tire.is_pressure_low,
                        pressure: front_tire.pressure,
                        temperature: front_tire.temperature,
                    }),
                    rear_tire: Some(TirePressure {
                        is_pressure_low: rear_tire.is_pressure_low,
                        pressure: rear_tire.pressure,
                        temperature: rear_tire.temperature,
                    }),
                });
            } else {
                error!("Axle is not initialized");
                return None;
            }
        }
        error!("Failed to extract tire data from the vehicle state.");
        None
    }

    pub fn to_speed_event(&self) -> Option<SpeedEvent> {
        Some(SpeedEvent {
            vehicle_id: self.vehicle_id(),
            speed: self.vehicle.speed,
        })
    }

    pub fn to_exterior_event(&self) -> Option<ExteriorEvent> {
        if let Some(exterior) = &self.vehicle.exterior {
            return Some(ExteriorEvent {
                vehicle_id: self.vehicle_id(),
                air_temperature: exterior.air_temperature,
                humidity: exterior.humidity,
                light_intensity: exterior.light_intensity,
            });
        }
        error!("Failed to extract exterior data from the vehicle state.");
        None
    }

    pub fn to_current_location_event(&self) -> Option<CurrentLocationEvent> {
        if let Some(current_location) = &self.vehicle.current_location {
            return Some(CurrentLocationEvent {
                vehicle_id: self.vehicle_id(),
                latitude: current_location.latitude,
                longitude: current_location.longitude,
                altitude: current_location.altitude,
                timestamp: current_location.timestamp.clone(),
            });
        }
        error!("Failed to extract current location data from the vehicle state.");
        None
    }

    pub fn to_trip_data_event(&self) -> Option<TripDataEvent> {
        Some(TripDataEvent {
            vehicle_id: self.vehicle_id(),
            start_time: self.vehicle.start_time.clone(),
            traveled_distance: self.vehicle.traveled_distance,
            traveled_distance_since_start: self.vehicle.traveled_distance_since_start,
            trip_duration: self.vehicle.trip_duration,
            trip_meter_reading: self.vehicle.trip_meter_reading,
            average_speed: self.vehicle.average_speed,
        })
    }

    pub fn to_state_event(&self) -> Option<SystemStateEvent> {
        Some(SystemStateEvent {
            vehicle_id: self.vehicle_id(),
            system_state: self.vehicle.low_voltage_system_state.clone(),
        })
    }
}

#[derive(Debug)]
pub enum VehicleCommand {
    Lock,
    Unlock,
    HornOn,
    HornOff,
    LightOn,
    LightOff,
    EngineOn,
    EngineOff,
}
