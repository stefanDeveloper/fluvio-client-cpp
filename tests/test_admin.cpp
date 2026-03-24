#include "fluvio-client-cpp/src/lib.rs.h"
#include <iostream>

int main() {
    try {
        std::cout << "Test: Connecting to admin..." << std::endl;
        auto admin = fluvio_admin_connect();
        
        std::cout << "Test: Creating topic 'admin-test-topic'..." << std::endl;
        try {
            admin_create_topic(*admin, "admin-test-topic", 1, 1);
        } catch (...) {}
        
        std::cout << "Test: Deleting topic 'admin-test-topic'..." << std::endl;
        admin_delete_topic(*admin, "admin-test-topic");

        std::cout << "Admin Test Passed!" << std::endl;
    } catch (const std::exception& e) {
        std::cerr << "Test Failed: " << e.what() << std::endl;
        return 1;
    }
    return 0;
}
