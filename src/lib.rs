pub mod admin;
pub mod client;
pub mod config;
pub mod consumer;
pub mod produce_output;
pub mod producer;
pub mod c_api;

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        type FluvioClient;
        type FluvioProducer;
        type FluvioStream;
        type FluvioRecord;

        type FluvioConfigWrapper;
        type ConsumerConfigWrapper;
        type ProducerConfigWrapper;
        type FluvioProduceOutput;
        type FluvioRecordMetadata;
        type FluvioAdminClient;

        /// Connects to a Fluvio cluster
        fn fluvio_connect() -> Result<Box<FluvioClient>>;
        /// Connects to a Fluvio cluster with explicit config
        fn fluvio_connect_with_config(config: &FluvioConfigWrapper) -> Result<Box<FluvioClient>>;

        /// Creates a new topic producer configuration builder
        fn producer_config_new() -> Box<ProducerConfigWrapper>;
        /// Sets the maximum batch size in bytes for the producer
        fn producer_config_batch_size(c: &mut ProducerConfigWrapper, size: usize);
        /// Sets the linger time in milliseconds for the producer
        fn producer_config_linger(c: &mut ProducerConfigWrapper, linger: u64);

        /// Creates a new consumer configuration builder
        fn consumer_config_new() -> Box<ConsumerConfigWrapper>;
        /// Sets the maximum bytes to fetch per request
        fn consumer_config_max_bytes(c: &mut ConsumerConfigWrapper, max: i32);
        /// Disables continuous fetching
        fn consumer_config_disable_continuous(c: &mut ConsumerConfigWrapper, val: bool);

        /// Creates a new Fluvio cluster configuration with the specified endpoint
        fn fluvio_config_new(addr: &str) -> Box<FluvioConfigWrapper>;
        /// Loads the Fluvio configuration from the default profile path
        fn fluvio_config_load() -> Result<Box<FluvioConfigWrapper>>;
        /// Sets the endpoint for the cluster configuration
        fn fluvio_config_set_endpoint(c: &mut FluvioConfigWrapper, endpoint: &str);
        /// Sets the client identifier for the cluster configuration
        fn fluvio_config_set_client_id(c: &mut FluvioConfigWrapper, client_id: &str);
        fn fluvio_config_disable_tls(c: &mut FluvioConfigWrapper);
        fn fluvio_config_set_anonymous_tls(c: &mut FluvioConfigWrapper);
        fn fluvio_config_set_inline_tls(c: &mut FluvioConfigWrapper, domain: &str, key: &str, cert: &str, ca_cert: &str);
        fn fluvio_config_set_tls_file_paths(c: &mut FluvioConfigWrapper, domain: &str, key_path: &str, cert_path: &str, ca_cert_path: &str);

        /// Creates a producer for the specified topic
        fn create_producer(client: &FluvioClient, topic: &str) -> Result<Box<FluvioProducer>>;
        /// Creates a producer for the specified topic with custom configuration
        fn create_producer_with_config(client: &FluvioClient, topic: &str, config: &ProducerConfigWrapper) -> Result<Box<FluvioProducer>>;
        /// Sends a key-value record to the topic asynchronously
        fn producer_send(producer: &FluvioProducer, key: &[u8], value: &[u8]) -> Result<Box<FluvioProduceOutput>>;
        /// Flushes the producer batches
        fn producer_flush(producer: &FluvioProducer) -> Result<()>;
        /// Blocks and waits for the producer record confirmation
        fn produce_output_wait(output: &mut FluvioProduceOutput) -> Result<Box<FluvioRecordMetadata>>;

        /// Creates a continuous stream for the consumer starting from the given offset index (0=Beginning, -1=End)
        fn consumer_stream(client: &FluvioClient, topic: &str, partition: u32, offset_index: i64) -> Result<Box<FluvioStream>>;
        /// Retrieves the next record from the stream blocks until available
        fn stream_next(stream: &mut FluvioStream) -> Result<Box<FluvioRecord>>;
        /// Retrieves the payload value byte array from a fetched record
        fn record_value(record: &FluvioRecord) -> Vec<u8>;
        /// Retrieves the key byte array from a fetched record
        fn record_key(record: &FluvioRecord) -> Vec<u8>;
        /// Retrieves the literal offset index of the fetched record
        fn record_offset(record: &FluvioRecord) -> i64;

        /// Connects to the Fluvio Administrative controller
        fn fluvio_admin_connect() -> Result<Box<FluvioAdminClient>>;
        /// Dispatches a command to create a new topic
        fn admin_create_topic(admin: &FluvioAdminClient, topic: &str, partitions: i32, replicas: i32) -> Result<()>;
        /// Dispatches a command to violently delete a topic
        fn admin_delete_topic(admin: &FluvioAdminClient, topic: &str) -> Result<()>;
    }
}

// Re-export all functions so cxx::bridge can find them in the root crate module
pub use admin::*;
pub use client::*;
pub use config::*;
pub use consumer::*;
pub use produce_output::*;
pub use producer::*;
