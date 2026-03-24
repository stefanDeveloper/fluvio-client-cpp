use fluvio::TopicProducerPool;
use fluvio_future::task::run_block_on;
use crate::config::ProducerConfigWrapper;
use crate::client::FluvioClient;
use crate::produce_output::FluvioProduceOutput;

pub struct FluvioProducer { pub inner: TopicProducerPool }

pub fn create_producer(client: &FluvioClient, topic: &str) -> Result<Box<FluvioProducer>, String> {
    run_block_on(client.inner.topic_producer(topic))
        .map(|producer| Box::new(FluvioProducer { inner: producer }))
        .map_err(|e| e.to_string())
}

pub fn create_producer_with_config(client: &FluvioClient, topic: &str, config: &ProducerConfigWrapper) -> Result<Box<FluvioProducer>, String> {
    let built_config = config.inner.build().map_err(|e| e.to_string())?;
    run_block_on(client.inner.topic_producer_with_config(topic, built_config))
        .map(|producer| Box::new(FluvioProducer { inner: producer }))
        .map_err(|e| e.to_string())
}

pub fn producer_send(producer: &FluvioProducer, key: &[u8], value: &[u8]) -> Result<Box<FluvioProduceOutput>, String> {
    run_block_on(producer.inner.send(key, value))
        .map(|out| Box::new(FluvioProduceOutput { inner: Some(out) }))
        .map_err(|e| e.to_string())
}

pub fn producer_flush(producer: &FluvioProducer) -> Result<(), String> {
    run_block_on(producer.inner.flush())
        .map(|_| ())
        .map_err(|e| e.to_string())
}
