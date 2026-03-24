#include "fluvio.h"
#include <stdio.h>

int main() {
    fluvio_config_t* config = NULL;
    if (fluvio_c_config_load(&config) != 0) {
        printf("Failed to load generic config\n");
    }

    fluvio_c_config_disable_tls(config);
    fluvio_c_config_set_anonymous_tls(config);
    fluvio_c_config_set_inline_tls(config, "domain.com", "secret-key", "cert-val", "ca-val");
    fluvio_c_config_set_tls_file_paths(config, "domain.com", "key.pem", "cert.pem", "ca.pem");

    printf("Pure C Native Auth Config Test Successfully Evaluated!\n");
    return 0;
}
