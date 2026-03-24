use fluvio::{Offset, consumer::Record as NativeRecord, consumer::ConsumerConfigExtBuilder};
use fluvio_future::task::run_block_on;
use futures_util::stream::StreamExt;
use futures_util::stream::Stream;
use std::pin::Pin;
use fluvio::dataplane::link::ErrorCode;
use crate::client::FluvioClient;

pub struct FluvioRecord { pub inner: NativeRecord }

type ConsumerStreamInner = Pin<Box<dyn Stream<Item = Result<NativeRecord, ErrorCode>> + Send>>;
pub struct FluvioStream { pub inner: ConsumerStreamInner }

pub fn consumer_stream(client: &FluvioClient, topic: &str, partition: u32, offset_index: i64) -> Result<Box<FluvioStream>, String> {
    let offset = if offset_index == -1 { Offset::end() } else if offset_index == 0 { Offset::beginning() } else { Offset::absolute(offset_index).unwrap() };
    let config = ConsumerConfigExtBuilder::default()
        .topic(topic.to_string())
        .partition(partition)
        .offset_start(offset)
        .build()
        .map_err(|e| e.to_string())?;

    let consumer_stream = run_block_on(client.inner.consumer_with_config(config)).map_err(|e| e.to_string())?;
    Ok(Box::new(FluvioStream { inner: Box::pin(consumer_stream) }))
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
