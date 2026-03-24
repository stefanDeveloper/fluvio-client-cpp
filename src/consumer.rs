use fluvio::{PartitionConsumer, Offset, consumer::Record as NativeRecord};
use fluvio_future::task::run_block_on;
use futures_util::stream::StreamExt;
use futures_util::stream::Stream;
use std::pin::Pin;
use fluvio::dataplane::link::ErrorCode;
use crate::config::ConsumerConfigWrapper;
use crate::client::FluvioClient;

pub struct FluvioConsumer { pub inner: PartitionConsumer }
pub struct FluvioRecord { pub inner: NativeRecord }

type PartitionConsumerStreamInner = Pin<Box<dyn Stream<Item = Result<NativeRecord, ErrorCode>> + Send>>;
pub struct FluvioStream { pub inner: PartitionConsumerStreamInner }

pub fn partition_consumer(client: &FluvioClient, topic: &str, partition: u32) -> Result<Box<FluvioConsumer>, String> {
    run_block_on(client.inner.partition_consumer(topic, partition)).map(|c| Box::new(FluvioConsumer { inner: c })).map_err(|e| e.to_string())
}

#[allow(deprecated)]
pub fn consumer_with_config(client: &FluvioClient, topic: &str, partition: u32, config: &ConsumerConfigWrapper) -> Result<Box<FluvioStream>, String> {
    let consumer = run_block_on(client.inner.partition_consumer(topic, partition)).map_err(|e| e.to_string())?;
    let built_config = config.inner.build().map_err(|e| e.to_string())?;
    run_block_on(consumer.stream_with_config(Offset::beginning(), built_config))
        .map(|s| Box::new(FluvioStream { inner: Box::pin(s) }))
        .map_err(|e| e.to_string())
}

#[allow(deprecated)]
pub fn consumer_stream(consumer: &FluvioConsumer, offset_index: i64) -> Result<Box<FluvioStream>, String> {
    let offset = if offset_index == -1 { Offset::end() } else if offset_index == 0 { Offset::beginning() } else { Offset::absolute(offset_index).unwrap() };
    run_block_on(consumer.inner.stream(offset)).map(|s| Box::new(FluvioStream { inner: Box::pin(s) })).map_err(|e| e.to_string())
}

pub fn stream_next(stream: &mut FluvioStream) -> Result<Box<FluvioRecord>, String> {
    match run_block_on(stream.inner.next()) {
        Some(Ok(rec)) => Ok(Box::new(FluvioRecord { inner: rec })),
        Some(Err(e)) => Err(e.to_string()),
        None => Err("EOF".to_string()),
    }
}

pub fn record_value(record: &FluvioRecord) -> Vec<u8> {
    record.inner.value().iter().cloned().collect()
}

pub fn record_key(record: &FluvioRecord) -> Vec<u8> {
    record.inner.key().map(|k| k.iter().cloned().collect()).unwrap_or_default()
}

pub fn record_offset(record: &FluvioRecord) -> i64 {
    record.inner.offset()
}
