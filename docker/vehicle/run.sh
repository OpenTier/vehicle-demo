#!/bin/bash

# Run Update Client
update_client &

# Run Twin Service
twin_service --twin-config /app/twin_config.json5 --vehicle-state-config /app/vehicle_initial_state.json5 &

# Run signal mocker service
signal_mocker_service --config /app/mock_data.json5
