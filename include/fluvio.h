#ifndef FLUVIO_C_H
#define FLUVIO_C_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

// Opaque types
typedef struct fluvio_client_opaque fluvio_client_t;
typedef struct fluvio_producer_opaque fluvio_topic_producer_t;
typedef struct fluvio_consumer_opaque fluvio_partition_consumer_t;
typedef struct fluvio_produce_output_opaque fluvio_produce_output_t;
typedef struct fluvio_stream_opaque fluvio_stream_t;
typedef struct fluvio_record_opaque fluvio_record_t;
typedef struct fluvio_config_opaque fluvio_config_t;

// Client
int fluvio_c_config_load(fluvio_config_t** out_config);
void fluvio_c_config_set_endpoint(fluvio_config_t* config, const char* endpoint);
void fluvio_c_config_set_client_id(fluvio_config_t* config, const char* client_id);
void fluvio_c_config_disable_tls(fluvio_config_t* config);
void fluvio_c_config_set_anonymous_tls(fluvio_config_t* config);
void fluvio_c_config_set_inline_tls(fluvio_config_t* config, const char* domain, const char* key, const char* cert, const char* ca_cert);
void fluvio_c_config_set_tls_file_paths(fluvio_config_t* config, const char* domain, const char* key_path, const char* cert_path, const char* ca_cert_path);
int fluvio_c_connect(fluvio_client_t** out_client);
int fluvio_c_connect_with_config(fluvio_config_t* config, fluvio_client_t** out_client);
void fluvio_c_client_free(fluvio_client_t* client);

// Producer
int fluvio_c_create_producer(fluvio_client_t* client, const char* topic, fluvio_topic_producer_t** out_producer);
int fluvio_c_producer_send(fluvio_topic_producer_t* producer, const uint8_t* key, size_t key_len, const uint8_t* val, size_t val_len, fluvio_produce_output_t** out);
int fluvio_c_produce_output_wait(fluvio_produce_output_t* out);
int fluvio_c_producer_flush(fluvio_topic_producer_t* producer);
void fluvio_c_producer_free(fluvio_topic_producer_t* producer);
void fluvio_c_produce_output_free(fluvio_produce_output_t* out);

// Consumer
int fluvio_c_partition_consumer(fluvio_client_t* client, const char* topic, uint32_t partition, fluvio_partition_consumer_t** out_consumer);
int fluvio_c_consumer_stream(fluvio_partition_consumer_t* consumer, int64_t offset_index, fluvio_stream_t** out_stream);
int fluvio_c_stream_next(fluvio_stream_t* stream, fluvio_record_t** out_record);
int fluvio_c_record_value(fluvio_record_t* record, const uint8_t** out_buf, size_t* out_len);
void fluvio_c_record_free(fluvio_record_t* record);
void fluvio_c_stream_free(fluvio_stream_t* stream);
void fluvio_c_consumer_free(fluvio_partition_consumer_t* consumer);

#ifdef __cplusplus
}
#endif

#endif // FLUVIO_C_H
