# build stage
FROM rust:1.81.0 AS builder

WORKDIR /app

# install curl and unzip
RUN apt-get update && apt-get install -y \
    curl \
    unzip \
    protobuf-compiler

# copy Rust apps files
COPY ../../Cargo.toml /app/Cargo.toml
COPY ../../vehicle /app/vehicle

# copy protobuf files
COPY ../../proto/ /app/proto
COPY ../../vehicle-cloud-api/proto /app/vehicle-cloud-api/proto

# build Rust apps
RUN cargo build --release

# run stage
FROM ubuntu:22.04 AS runner

# setup working directory
WORKDIR /app

# install dependencies needed to run dashboard
RUN DEBIAN_FRONTEND=noninteractive apt-get update && apt-get install -y \
    libx11-xcb1 \
    libxkbcommon-x11-0 \
    x11-apps \
    libxcursor1 \
    libxi6 \
    libxcb-glx0 \
    libxcb-keysyms1 \
    libxcb-image0 \
    libxcb-shm0 \
    libxcb-icccm4 \
    libxcb-sync1 \
    libxcb-xfixes0 \
    libxcb-shape0 \
    libxcb-randr0 \
    libxcb-render-util0 \
    libxrender1 \
    libxcb1 \
    libfontconfig1 \
    libfreetype6 \
    libqt5gui5=5.15.* \
    libqt5core5a=5.15.* \
    libqt5widgets5=5.15.* \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

# copy the built binary to the new image
COPY --from=builder /app/target/release/update_client /usr/local/bin/
COPY --from=builder /app/target/release/dashboard /usr/local/bin/
COPY --from=builder /app/target/release/twin_service /usr/local/bin/
COPY --from=builder /app/target/release/signal_mocker_service /usr/local/bin/
# copy configurations
COPY --from=builder /app/vehicle/signal_mocker_service/config/mock_data.json5 /app/mock_data.json5
COPY --from=builder /app/vehicle/twin_service/config/twin_config.json5 /app/twin_config.json5
COPY --from=builder /app/vehicle/twin_service/config/vehicle_initial_state.json5 /app/vehicle_initial_state.json5

# copy run.sh to workdir
COPY docker/vehicle/run.sh /app/run.sh

# set run script as entrypoint
ENTRYPOINT ["/app/run.sh"]
