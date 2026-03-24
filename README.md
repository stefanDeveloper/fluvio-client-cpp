<h1 align="center">Fluvio Client for C++</h1>
<div align="center">
 <strong>
   Native C++ binding for the Fluvio streaming platform.
 </strong>
</div>
<br />

[![Build](https://github.com/stefanDeveloper/fluvio-client-cpp/actions/workflows/release.yml/badge.svg)](https://github.com/stefanDeveloper/fluvio-client-cpp/actions/workflows/release.yml)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://github.com/stefanDeveloper/fluvio-client-cpp/blob/main/LICENSE)
[![vcpkg](https://img.shields.io/badge/vcpkg-supported-blue.svg)](https://github.com/microsoft/vcpkg)

## Documentation

The client API documentation is written in standard Markdown and generated dynamically into C++ headers. You can find the full API overview in [docs/API.md](docs/API.md).

## Installation

You can install the client effortlessly without compiling the heavy Rust toolchain by using `vcpkg`.

```bash
vcpkg install fluvio-client-cpp
```

In your `CMakeLists.txt`, simply find the package and link it:

```cmake
find_package(fluvio_client_cpp CONFIG REQUIRED)
target_link_libraries(main PRIVATE fluvio_client_cpp::fluvio_client_cpp)
```

# Example Usage

## Creating a Topic

```cpp
#include "fluvio-client-cpp-sys/src/lib.rs.h"

int main() {
    auto admin = fluvio_admin_connect();
    admin_create_topic(*admin, "a_topic", 1, 1);
    return 0;
}
```

## Producer

```cpp
#include "fluvio-client-cpp-sys/src/lib.rs.h"
#include "rust/cxx.h"
#include <string>

int main() {
    auto client = fluvio_connect();
    auto producer = create_producer(*client, "my-topic");

    std::string payload = "FOOBAR";
    uint8_t key[] = {};

    producer_send(*producer, 
        rust::Slice<const uint8_t>(key, 0), 
        rust::Slice<const uint8_t>(reinterpret_cast<const uint8_t*>(payload.data()), payload.size())
    );
    
    producer_flush(*producer);
    return 0;
}
```

## Consumer

```cpp
#include "fluvio-client-cpp-sys/src/lib.rs.h"
#include <iostream>

int main() {
    auto client = fluvio_connect();
    auto consumer = partition_consumer(*client, "my-topic", 0);
    auto stream = consumer_stream(*consumer, 0); // Offset::beginning

    for (int i = 0; i < 1; i++) {
        auto rec = stream_next(*stream);
        auto val = record_value(*rec);
        std::string payload(val.begin(), val.end());
        std::cout << payload << std::endl;
    }

    return 0;
}
```

# Developer Notes

This project uses [CXX](https://cxx.rs) to safely wrap the underlying asynchronous Rust Fluvio crate into native synchronous C++ headers.

For binary distribution, GitHub Actions compiles the Rust library (`libfluvio_client_cpp_sys.a`) and creates a release tarball. The included `vcpkg-port` simply downloads this artifact, bypassing Rust compilation entirely for the end user.

To compile from source locally, ensure you have Rust installed and run:

```bash
cargo build --release
```

To run the integrated test suite locally against your Fluvio cluster, use CTest:

```bash
cmake -B build
cmake --build build
cd build
ctest --output-on-failure
```
