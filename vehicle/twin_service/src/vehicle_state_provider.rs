use crate::vehicle_state::VehicleState;
use common::topics::*;
use common::SubscriberTaskSpawner;
use log::trace;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use vehicle_msgs::battery::BatteryData;
use vehicle_msgs::current_location::CurrentLocation;
use vehicle_msgs::exterior::Exterior;
use vehicle_msgs::speed::Speed;
use vehicle_msgs::state::LockState;
use vehicle_msgs::tires::Tires;
use vehicle_msgs::trip_data::TripData;

pub struct VehicleStateProvider {
    state: Arc<Mutex<VehicleState>>,
}

impl VehicleStateProvider {
    pub fn new(state: Arc<Mutex<VehicleState>>) -> Self {
        Self { state }
    }

    pub async fn run(
        &self,
        session: Arc<zenoh::Session>,
    ) -> Result<Vec<JoinHandle<()>>, Box<dyn std::error::Error + Send + Sync>> {
        let (lock_tx, mut lock_rx) = mpsc::channel::<LockState>(32);
        let lock_task =
            SubscriberTaskSpawner::spawn_task(session.clone(), LOCK_STATE_TOPIC, lock_tx);

        let (battery_tx, mut battery_rx) = mpsc::channel::<BatteryData>(100);
        let battery_task =
            SubscriberTaskSpawner::spawn_task(session.clone(), BATTERY_STATE_TOPIC, battery_tx);

        let (exterior_tx, mut exterior_rx) = mpsc::channel::<Exterior>(100);
        let exterior_task =
            SubscriberTaskSpawner::spawn_task(session.clone(), EXTERIOR_TOPIC, exterior_tx);

        let (speed_tx, mut speed_rx) = mpsc::channel::<Speed>(100);
        let speed_task = SubscriberTaskSpawner::spawn_task(session.clone(), SPEED_TOPIC, speed_tx);

        let (trip_data_tx, mut trip_data_rx) = mpsc::channel::<TripData>(100);
        let trip_data_task =
            SubscriberTaskSpawner::spawn_task(session.clone(), TRIP_DATA_TOPIC, trip_data_tx);

        let (tires_tx, mut tires_rx) = mpsc::channel::<Tires>(100);
        let tires_task = SubscriberTaskSpawner::spawn_task(session.clone(), TIRES_TOPIC, tires_tx);

        let (current_location_tx, mut current_location_rx) = mpsc::channel::<CurrentLocation>(100);
        let current_location_task = SubscriberTaskSpawner::spawn_task(
            session.clone(),
            CURRENT_LOCATION_TOPIC,
            current_location_tx,
        );

        let state = Arc::clone(&self.state);

        let consumer_task = tokio::spawn(async move {
            loop {
                tokio::select! {
                    Some(lock_state) = lock_rx.recv() => {
                        trace!("Received LockState: {:?}", lock_state);
                        let mut state = state.lock().await;
                        state.update(lock_state).await;
                    }
                    Some(exterior) = exterior_rx.recv() => {
                        trace!("Received Exterior: {:?}", exterior);
                        let mut state = state.lock().await;
                        state.update(exterior).await;
                    },
                    Some(speed) = speed_rx.recv() => {
                        trace!("Received Speed: {:?}", speed);
                        let mut state = state.lock().await;
                        state.update(speed).await;
                    }
                    Some(trip_data) = trip_data_rx.recv() => {
                        trace!("Received TripData: {:?}", trip_data);
                        let mut state = state.lock().await;
                        state.update(trip_data).await;
                    }
                    Some(battery) = battery_rx.recv() => {
                        trace!("Received BatteryData: {:?}", battery);
                        let mut state = state.lock().await;
                        state.update(battery).await;
                    }
                    Some(tires) = tires_rx.recv() => {
                        trace!("Received Tires: {:?}", tires);
                        let mut state = state.lock().await;
                        state.update(tires).await;
                    }
                    Some(current_location) = current_location_rx.recv() => {
                        trace!("Received CurrentLocation: {:?}", current_location);
                        let mut state = state.lock().await;
                        state.update(current_location).await;
                    }
                    else => {
                        trace!("All channels closed.");
                        break;
                    }
                }
            }
        });

        Ok(vec![
            lock_task,
            battery_task,
            exterior_task,
            speed_task,
            trip_data_task,
            current_location_task,
            tires_task,
            consumer_task,
        ])
    }
}
