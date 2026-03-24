#include "fluvio-client-cpp/src/lib.rs.h"
#include <iostream>
#include <fmt/core.h>
#include <nlohmann/json.hpp>

using json = nlohmann::json;

int main() {
    try {
        fmt::print("Starting Fluvio Producer Example...\n");

        auto admin = fluvio_admin_connect();
        try {
            admin_create_topic(*admin, "example-topic", 1, 1);
            fmt::print("Created 'example-topic'.\n");
        } catch (...) {
            fmt::print("'example-topic' already exists or creation failed.\n");
        }

        auto client = fluvio_connect();
        auto producer = create_producer(*client, "example-topic");

        // Create a JSON payload
        json j = {
            {"sensor", "temp-01"},
            {"value", 24.5},
            {"status", "active"}
        };
        std::string payload = j.dump();

        fmt::print("Sending JSON: {}\n", payload);

        uint8_t key[] = {'j', 's', 'o', 'n'};
        auto out = producer_send(*producer,
            rust::Slice<const uint8_t>(key, sizeof(key)),
            rust::Slice<const uint8_t>(reinterpret_cast<const uint8_t*>(payload.data()), payload.size())
        );

        auto meta = produce_output_wait(*out);
        producer_flush(*producer);

        fmt::print("Record successfully sent to Fluvio!\n");

    } catch (const std::exception& e) {
        fmt::print(stderr, "Fatal error: {}\n", e.what());
        return 1;
    }
    return 0;
}
