# Fluvio C++ Client API

The Fluvio C++ Client provides extension bindings for working with the Fluvio streaming platform natively in C++ via Rust `cxx`.

## 1. Administrative API
Creating a topic with default settings is as simple as:

```cpp
#include "fluvio-client-cpp-sys/src/lib.rs.h"
#include <iostream>

int main() {
    auto admin = fluvio_admin_connect();
    admin_create_topic(*admin, "a_topic", 1, 1);
    return 0;
}
```

## 2. Producing Data
Producing data to a topic in a Fluvio cluster:

```cpp
#include "fluvio-client-cpp-sys/src/lib.rs.h"
#include "rust/cxx.h"
#include <string>

int main() {
    auto client = fluvio_connect();
    auto producer = create_producer(*client, "a_topic");

    for (int i = 0; i < 10; ++i) {
        std::string payload = "Hello " + std::to_string(i);
        uint8_t key[] = {};
        producer_send(*producer,
            rust::Slice<const uint8_t>(key, 0),
            rust::Slice<const uint8_t>(reinterpret_cast<const uint8_t*>(payload.data()), payload.size())
        );
    }
    producer_flush(*producer);
    return 0;
}
```

### Custom Producer Configurations
You can configure the producer's internal buffer batching, linger, and memory constraints:

```cpp
auto config = producer_config_new();
producer_config_batch_size(*config, 32768);
producer_config_linger(*config, 100); // 100ms

auto producer = create_producer_with_config(*client, "a_topic", *config);
```

## 3. Consuming Data
Consuming events via streaming from an offset (e.g. `Offset::beginning` indexed via `0`):

```cpp
#include "fluvio-client-cpp-sys/src/lib.rs.h"
#include <iostream>

int main() {
    auto client = fluvio_connect();
    auto consumer = partition_consumer(*client, "a_topic", 0);
    
    // 0 = beginning, -1 = end, >0 = absolute offset
    auto stream = consumer_stream(*consumer, 0); 

    for (int i = 0; i < 2; ++i) {
        auto rec = stream_next(*stream);
        auto val = record_value(*rec);
        std::string payload(val.begin(), val.end());
        std::cout << "Received: " << payload << std::endl;
    }
    return 0;
}
```

### Custom Consumer Configurations
Restrict memory constraints implicitly via `ConsumerConfig` settings:

```cpp
auto config = consumer_config_new();
consumer_config_max_bytes(*config, 1024 * 1024); // 1MB constraint
consumer_config_disable_continuous(*config, true);

auto stream = consumer_with_config(*client, "a_topic", 0, *config);
```

## 4. Connection Details (TLS & Config)

Instead of implicitly connecting via `~/.fluvio/config`, you can explicitly point connections to isolated clusters with custom configurations:

```cpp
auto config = fluvio_config_new("localhost:9003");
fluvio_config_set_client_id(*config, "cpp-app");
auto client = fluvio_connect_with_config(*config);
```

For further reference regarding the core bindings, inspect the generated `lib.rs.h` header which propagates Rust-native doc comments straight to C++!
