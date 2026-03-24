use fluvio::Fluvio;
use fluvio_future::task::run_block_on;
use crate::config::FluvioConfigWrapper;

pub struct FluvioClient { pub inner: Fluvio }

pub fn fluvio_connect() -> Result<Box<FluvioClient>, String> {
    run_block_on(Fluvio::connect())
        .map(|fluvio| Box::new(FluvioClient { inner: fluvio }))
        .map_err(|e| e.to_string())
}

pub fn fluvio_connect_with_config(config: &FluvioConfigWrapper) -> Result<Box<FluvioClient>, String> {
    run_block_on(Fluvio::connect_with_config(&config.inner))
        .map(|fluvio| Box::new(FluvioClient { inner: fluvio }))
        .map_err(|e| e.to_string())
}
