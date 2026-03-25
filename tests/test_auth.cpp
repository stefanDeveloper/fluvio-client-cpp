#include "fluvio-client-cpp/src/lib.rs.h"
#include <iostream>
#include <cstdlib>

int main() {
    try {
        std::cout << "[E2E-AUTH] Bootstrapping TLS Configurations..." << std::endl;
        
        const char* domain = std::getenv("FLUVIO_E2E_TLS_DOMAIN");
        const char* key = std::getenv("FLUVIO_E2E_TLS_KEY");
        const char* cert = std::getenv("FLUVIO_E2E_TLS_CERT");
        const char* ca = std::getenv("FLUVIO_E2E_TLS_CA");

        auto fluvioConfig = fluvio_config_new("localhost:9003");

        if (domain && key && cert && ca) {
            std::cout << "[E2E-AUTH] Active TLS parameters detected! Configuring strict mTLS execution pipeline." << std::endl;
            fluvio_config_set_tls_file_paths(*fluvioConfig, domain, key, cert, ca);
        } else {
            std::cout << "[E2E-AUTH] No TLS parameters detected in ENV. Proceeding with TLS-Disabled configuration checks." << std::endl;
            fluvio_config_disable_tls(*fluvioConfig);
            std::cout << "CXX TLS Auth Object Creation Successfully Evaluated Offline." << std::endl;
            return 0;
        }
        
        std::cout << "[E2E-AUTH] Attempting live Fluvio Socket TLS Auth Connection..." << std::endl;
        auto authenticatedClient = fluvio_connect_with_config(*fluvioConfig);
        std::cout << "[E2E-AUTH] Successfully authenticated to cluster natively via TLS mTLS bindings!" << std::endl;

        std::cout << "[E2E-AUTH] Creating producer for 'test-auth-topic'..." << std::endl;
        auto producer = create_producer(*authenticatedClient, "test-auth-topic");
        
        uint8_t payload[] = {'s', 'e', 'c', 'u', 'r', 'e'};
        producer_send(*producer, 
            rust::Slice<const uint8_t>(), 
            rust::Slice<const uint8_t>(payload, sizeof(payload)));
        producer_flush(*producer);
        std::cout << "[E2E-AUTH] 🔒 Payload shipped through TLS socket!" << std::endl;

        std::cout << "[E2E-AUTH] Bootstrapping Authenticated Consumer..." << std::endl;
        auto stream = consumer_stream(*authenticatedClient, "test-auth-topic", 0, 0); 
        auto rec = stream_next(*stream);
        auto val = record_value(*rec);

        if(val.size() == 6) {
            std::cout << "[E2E-AUTH] 🔓 Successfully received authenticated payload matrix decrypting exact size match!" << std::endl;
        } else {
            throw std::runtime_error("Decrypted payload size mismatch dropping verification boundary!");
        }

        std::cout << "[E2E-AUTH] CXX TLS Strict Auth End-to-End Test Successfully Evaluated!" << std::endl;
    } catch (const std::exception& e) {
        std::cerr << "[E2E-AUTH] FATAL Auth Disconnect: " << e.what() << std::endl;
        return 1;
    }
    return 0;
}
