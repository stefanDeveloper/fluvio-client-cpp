#include "fluvio-client-cpp-sys/src/lib.rs.h"
#include <iostream>

int main() {
    try {
        std::cout << "Test: Creating configurations..." << std::endl;
        auto cConfig = consumer_config_new();
        consumer_config_max_bytes(*cConfig, 1024);

        auto pConfig = producer_config_new();
        
        std::cout << "Config Test Passed!" << std::endl;
    } catch (const std::exception& e) {
        std::cerr << "Test Failed: " << e.what() << std::endl;
        return 1;
    }
    return 0;
}
