# Fluvio C++ Client Examples

Welcome to the Fluvio C++ Client examples! These examples provide a quick and easy way to learn how to write native C++ applications that interact with a Fluvio streaming cluster.

## Prerequisites

Before building the examples, ensure you have the following:
- A modern C++ compiler (supporting C++17 or later)
- [CMake](https://cmake.org/download/) (version 3.20+)
- [vcpkg](https://vcpkg.io/en/getting-started.html) for dependency management
- A running [Fluvio cluster](https://www.fluvio.io/docs/get-started/) (local or InfinyOn Cloud)

## What's Included?

We provide two simple applications to demonstrate the core features of the client:

- **Producer (`producer.cpp`)**: Connects to the Fluvio cluster as an admin to ensure a topic named `example-topic` exists. It then creates a producer and sends a mock JSON payload representing sensor data.
- **Consumer (`consumer.cpp`)**: Connects to the Fluvio cluster, opens a stream on `example-topic`, and parses the incoming JSON data using `nlohmann::json`.

## Building the Examples

The examples use `vcpkg` to pull in third-party libraries like `fmt` for modern logging and `nlohmann-json` for JSON parsing. The Fluvio C++ library itself is downloaded automatically via CMake's `FetchContent` using the prebuilt GitHub release binaries.

1. Make sure you have the `VCPKG_ROOT` environment variable set to your vcpkg installation path:
   ```bash
   export VCPKG_ROOT=/path/to/your/vcpkg
   ```

2. Configure the CMake project:
   ```bash
   cmake -S . -B build -G Ninja \
     -DCMAKE_BUILD_TYPE=Release \
     -DCMAKE_TOOLCHAIN_FILE=$VCPKG_ROOT/scripts/buildsystems/vcpkg.cmake
   ```
   *(Note: You can omit `-G Ninja` if you haven't installed `ninja` and prefer the default Makefile generator).*

3. Build the executables:
   ```bash
   cmake --build build --parallel
   ```

## Running the Examples

Make sure your Fluvio cluster is running and your current environment has access to it.

1. Start the consumer. It will connect to the cluster and wait for messages:
   ```bash
   ./build/consumer_example
   ```

2. Open a second terminal window/tab and run the producer:
   ```bash
   ./build/producer_example
   ```

**Expected Output:**

The producer will output:
```text
Starting Fluvio Producer Example...
Created 'example-topic'.
Sending JSON: {"sensor":"temp-01","status":"active","value":24.5}
Record successfully sent to Fluvio!
```

The consumer will receive the data, parse it, and output:
```text
Starting Fluvio Consumer Example...
Waiting for messages...
Received Raw Bytes: {"sensor":"temp-01","status":"active","value":24.5}
Parsed JSON successfully: Sensor=temp-01 Value=24.5
```

Congratulations! You've successfully streamed data using C++!
