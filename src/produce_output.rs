use fluvio::{ProduceOutput as NativeProduceOutput, RecordMetadata as NativeRecordMetadata};
use fluvio_future::task::run_block_on;

pub struct FluvioProduceOutput { pub inner: Option<NativeProduceOutput> }
pub struct FluvioRecordMetadata { pub inner: NativeRecordMetadata }

pub fn produce_output_wait(output: &mut FluvioProduceOutput) -> Result<Box<FluvioRecordMetadata>, String> {
    let inner = output.inner.take();
    match inner {
        Some(produce_output) => {
            run_block_on(produce_output.wait())
                .map(|metadata| Box::new(FluvioRecordMetadata { inner: metadata }))
                .map_err(|e| e.to_string())
        },
        None => Err("ProduceOutput already consumed".to_string())
    }
}
