#include "fluvio-client-cpp/src/lib.rs.h"
#include "rust/cxx.h"
#include <iostream>

int main() {
    try {
        std::cout << "Test: Connecting to Fluvio..." << std::endl;
        auto client = fluvio_connect();

        std::cout << "Test: Creating stream for 'test-topic' partition 0..." << std::endl;
        auto consumer = partition_consumer(*client, "test-topic", 0);
        
        auto stream = consumer_stream(*consumer, 0); // Offset::beginning()
        
        std::cout << "Test: Fetching one record..." << std::endl;
        auto rec = stream_next(*stream);

        auto val = record_value(*rec);
        std::cout << "Fetched record value of size: " << val.size() << std::endl;

        std::cout << "Consumer Test Passed!" << std::endl;
    } catch (const std::exception& e) {
        std::cerr << "Test Failed: " << e.what() << std::endl;
        return 1;
    }
    return 0;
}
