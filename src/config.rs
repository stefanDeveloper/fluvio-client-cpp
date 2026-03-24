use fluvio::FluvioConfig;
use fluvio::consumer::ConsumerConfigBuilder;
use fluvio::TopicProducerConfigBuilder;

pub struct FluvioConfigWrapper { pub inner: FluvioConfig }
pub struct ConsumerConfigWrapper { pub inner: ConsumerConfigBuilder }
pub struct ProducerConfigWrapper { pub inner: TopicProducerConfigBuilder }

pub fn producer_config_new() -> Box<ProducerConfigWrapper> {
    Box::new(ProducerConfigWrapper { inner: TopicProducerConfigBuilder::default() })
}

pub fn consumer_config_new() -> Box<ConsumerConfigWrapper> {
    Box::new(ConsumerConfigWrapper { inner: ConsumerConfigBuilder::default() })
}

pub fn consumer_config_max_bytes(c: &mut ConsumerConfigWrapper, max: i32) {
    c.inner.max_bytes(max);
}

pub fn consumer_config_disable_continuous(c: &mut ConsumerConfigWrapper, val: bool) {
    c.inner.disable_continuous(val);
}

pub fn producer_config_batch_size(c: &mut ProducerConfigWrapper, size: usize) {
    c.inner.batch_size(size);
}

pub fn producer_config_linger(c: &mut ProducerConfigWrapper, linger: u64) {
    c.inner.linger(std::time::Duration::from_millis(linger));
}

pub fn fluvio_config_new(addr: &str) -> Box<FluvioConfigWrapper> {
    Box::new(FluvioConfigWrapper { inner: FluvioConfig::new(addr) })
}

pub fn fluvio_config_load() -> Result<Box<FluvioConfigWrapper>, String> {
    FluvioConfig::load().map(|c| Box::new(FluvioConfigWrapper { inner: c })).map_err(|e| e.to_string())
}

pub fn fluvio_config_set_endpoint(c: &mut FluvioConfigWrapper, endpoint: &str) {
    c.inner.endpoint = endpoint.to_string();
}

pub fn fluvio_config_set_client_id(c: &mut FluvioConfigWrapper, client_id: &str) {
    c.inner.client_id = Some(client_id.to_string());
}
