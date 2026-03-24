#include "fluvio.h"
#include <stdio.h>
#include <string.h>

int main() {
    printf("Starting C Test...\n");

    fluvio_client_t* client = NULL;
    if (fluvio_c_connect(&client) != 0) {
        printf("Failed to connect\n");
        return 1;
    }

    fluvio_topic_producer_t* producer = NULL;
    if (fluvio_c_create_producer(client, "test-topic", &producer) != 0) {
        printf("Failed to create producer\n");
        return 1;
    }

    const char* val = "C_PAYLOAD";
    fluvio_produce_output_t* out = NULL;
    if (fluvio_c_producer_send(producer, NULL, 0, (const uint8_t*)val, strlen(val), &out) != 0) {
        printf("Failed to send\n");
        return 1;
    }

    fluvio_c_produce_output_wait(out);
    fluvio_c_produce_output_free(out);
    fluvio_c_producer_flush(producer);

    fluvio_partition_consumer_t* consumer = NULL;
    if (fluvio_c_partition_consumer(client, "test-topic", 0, &consumer) != 0) {
        printf("Failed to create consumer\n");
        return 1;
    }

    fluvio_stream_t* stream = NULL;
    if (fluvio_c_consumer_stream(consumer, 0, &stream) != 0) {
        printf("Failed to create stream\n");
        return 1;
    }

    fluvio_record_t* record = NULL;
    if (fluvio_c_stream_next(stream, &record) == 0) {
        const uint8_t* buf = NULL;
        size_t len = 0;
        fluvio_c_record_value(record, &buf, &len);
        printf("Received payload length: %zu\n", len);
        fluvio_c_record_free(record);
    }

    fluvio_c_stream_free(stream);
    fluvio_c_consumer_free(consumer);
    fluvio_c_producer_free(producer);
    fluvio_c_client_free(client);

    printf("C Test Passed!\n");
    return 0;
}
