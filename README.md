# Vehicle Demo

This repository contains example services that demonstrate a small vehicle software stack. The services communicate over Zenoh and can run on a local development machine or on a Raspberry Pi.

## Directory overview

- **docker/** - Dockerfiles and helper scripts.
- **proto/** - Protocol Buffer definitions for vehicle signals and states.
- **vehicle/** - Source code for the individual services.
    - **common/** - Shared helpers used by the other services.
    - **signal_mocker_service/** - Generates mock telemetry signals.
    - **twin_service/** - Manages vehicle state and persists updates.
    - **update_client/** - Simple client for over-the-air updates.
    - **vehicle_msgs/** - Generated Rust types from `proto/`.
- **vehicle-cloud-api/** - API definitions for cloud interactions.

## Getting started

1. Fetch submodules and dependencies

```sh
git submodule update --init --recursive
```

2. Build the container image

```sh
docker build -f docker/vehicle/Dockerfile -t vehicle-demo .
```

3. Run the services

```sh
docker run --rm -it vehicle-demo
```

You can also build the workspace directly using Cargo if the required Rust toolchain and dependencies are installed.

## License

This project is licensed under the Mozilla Public License 2.0. See the [LICENSE](LICENSE) file for details.
