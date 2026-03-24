#include "fluvio-client-cpp/src/lib.rs.h"
#include <iostream>
#include <fmt/core.h>
#include <nlohmann/json.hpp>

using json = nlohmann::json;

int main() {
    try {
        fmt::print("Starting Fluvio Consumer Example...\n");

        auto client = fluvio_connect();
        auto consumer = partition_consumer(*client, "example-topic", 0);
        
        auto stream = consumer_stream(*consumer, 0); // Offset::beginning()

        fmt::print("Waiting for messages...\n");
        
        // Fetch one record
        auto rec = stream_next(*stream);
        auto val = record_value(*rec);

        std::string payload(val.begin(), val.end());
        fmt::print("Received Raw Bytes: {}\n", payload);

        try {
            json j = json::parse(payload);
            fmt::print("Parsed JSON successfully: Sensor={} Value={}\n", 
                j["sensor"].get<std::string>(), 
                j["value"].get<double>());
        } catch (const json::parse_error& e) {
            fmt::print(stderr, "Failed to parse JSON: {}\n", e.what());
        }

    } catch (const std::exception& e) {
        fmt::print(stderr, "Fatal error: {}\n", e.what());
        return 1;
    }
    return 0;
}
