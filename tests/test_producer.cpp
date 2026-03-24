#include "fluvio-client-cpp-sys/src/lib.rs.h"
#include "rust/cxx.h"
#include <iostream>

int main() {
    try {
        std::cout << "Test: Connecting to admin..." << std::endl;
        auto admin = fluvio_admin_connect();
        
        std::cout << "Test: Creating topic 'test-topic' (ignoring if it exists)..." << std::endl;
        try {
            admin_create_topic(*admin, "test-topic", 1, 1);
        } catch (const std::exception& e) {
            std::cout << "Topic might already exist: " << e.what() << std::endl;
        }

        std::cout << "Test: Connecting to Fluvio..." << std::endl;
        auto client = fluvio_connect();
        std::cout << "Test: Connecting successful." << std::endl;

        std::cout << "Test: Creating producer for 'test-topic'..." << std::endl;
        auto producer = create_producer(*client, "test-topic");

        std::cout << "Test: Sending record..." << std::endl;
        uint8_t key[] = {'t', 'e', 's', 't'};
        uint8_t val[] = {'1', '2', '3'};
        auto out = producer_send(*producer, 
            rust::Slice<const uint8_t>(key, sizeof(key)), 
            rust::Slice<const uint8_t>(val, sizeof(val)));

        std::cout << "Test: Waiting for record confirmation..." << std::endl;
        auto meta = produce_output_wait(*out);
        
        producer_flush(*producer);
        std::cout << "Producer Test Passed!" << std::endl;
    } catch (const std::exception& e) {
        std::cerr << "Test Failed: " << e.what() << std::endl;
        return 1;
    }
    return 0;
}
