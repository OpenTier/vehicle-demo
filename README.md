# Vehicle Demo

This is a demo of Vehicle embedded software, consisting of various services that can be run on a local machine or Raspberry Pi. It includes services for telemetry, remote control, and a service for mocking the data that comes from various vehicle peripherals.

## Directories and Files

- **docker/**: Contains Docker-related files for building and running the services.
    - **vehicle/**: Contains Dockerfile and run script for the whole Vehicle services.
    - **cross/rpi/**: Contains Dockerfile for cross-compiling code to Raspberry Pi.
- **proto/**: Contains Protocol Buffers (proto) definitions for various vehicle signals and states.
- **vehicle/**: Contains the source code for various vehicle-related services.
    - **common/**: Common utilities and modules.
    - **signal_mocker_service/**: Service for mocking vehicle signals.
    - **twin_service/**: Twin service for managing vehicle state.
    - **update_client/**: Client for updating vehicle software.
    - **vehicle_msgs/**: Definitions for vehicle messages.
- **vehicle-cloud-api/**: Contains API definitions and specifications for vehicle-cloud interactions.

## Getting Started

### Prerequisites

- Rust programming language
- Docker
- Protocol Buffers compiler (protoc)

### Building the Project
First, initialize and update the git submodules:
```sh
git submodule update --init --recursive
```

Then build the Docker image:
```sh
docker build -f docker/vehicle/Dockerfile -t vehicle-demo .
```

### Running the Project
Run the Docker container:
```sh
docker run --rm -it vehicle-demo
```
