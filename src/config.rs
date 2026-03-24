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

pub fn fluvio_config_disable_tls(c: &mut FluvioConfigWrapper) {
    c.inner.tls = fluvio::config::TlsPolicy::Disabled;
}

pub fn fluvio_config_set_anonymous_tls(c: &mut FluvioConfigWrapper) {
    c.inner.tls = fluvio::config::TlsPolicy::Anonymous;
}

pub fn fluvio_config_set_inline_tls(c: &mut FluvioConfigWrapper, domain: &str, key: &str, cert: &str, ca_cert: &str) {
    c.inner.tls = fluvio::config::TlsPolicy::Verified(fluvio::config::TlsConfig::Inline(fluvio::config::TlsCerts {
        domain: domain.to_string(),
        key: key.to_string(),
        cert: cert.to_string(),
        ca_cert: ca_cert.to_string(),
    }));
}

pub fn fluvio_config_set_tls_file_paths(c: &mut FluvioConfigWrapper, domain: &str, key_path: &str, cert_path: &str, ca_cert_path: &str) {
    c.inner.tls = fluvio::config::TlsPolicy::Verified(fluvio::config::TlsConfig::Files(fluvio::config::TlsPaths {
        domain: domain.to_string(),
        key: std::path::PathBuf::from(key_path),
        cert: std::path::PathBuf::from(cert_path),
        ca_cert: std::path::PathBuf::from(ca_cert_path),
    }));
}
