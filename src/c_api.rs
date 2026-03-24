use crate::client::{FluvioClient, fluvio_connect};
use crate::producer::{FluvioProducer, create_producer, producer_send, producer_flush};
use crate::consumer::{FluvioConsumer, FluvioStream, FluvioRecord, partition_consumer, consumer_stream, stream_next};
use crate::produce_output::{FluvioProduceOutput, produce_output_wait};
use std::os::raw::c_char;
use std::ffi::CStr;

#[unsafe(no_mangle)]
pub extern "C" fn fluvio_c_connect(out_client: *mut *mut FluvioClient) -> i32 {
    match fluvio_connect() {
        Ok(client) => {
            unsafe { *out_client = Box::into_raw(client); }
            0
        }
        Err(_) => -1,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn fluvio_c_client_free(client: *mut FluvioClient) {
    if !client.is_null() { unsafe { let _ = Box::from_raw(client); } }
}

#[unsafe(no_mangle)]
pub extern "C" fn fluvio_c_create_producer(client: *mut FluvioClient, topic: *const c_char, out_producer: *mut *mut FluvioProducer) -> i32 {
    if client.is_null() || topic.is_null() || out_producer.is_null() { return -1; }
    let topic_str = unsafe { CStr::from_ptr(topic).to_str() }.unwrap_or("");
    match create_producer(unsafe { &*client }, topic_str) {
        Ok(producer) => { unsafe { *out_producer = Box::into_raw(producer); } 0 }
        Err(_) => -1,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn fluvio_c_producer_send(producer: *mut FluvioProducer, key: *const u8, key_len: usize, val: *const u8, val_len: usize, out: *mut *mut FluvioProduceOutput) -> i32 {
    if producer.is_null() || (key.is_null() && key_len > 0) || (val.is_null() && val_len > 0) { return -1; }
    let key_slice = if key_len > 0 { unsafe { std::slice::from_raw_parts(key, key_len) } } else { &[] };
    let val_slice = if val_len > 0 { unsafe { std::slice::from_raw_parts(val, val_len) } } else { &[] };
    match producer_send(unsafe { &*producer }, key_slice, val_slice) {
        Ok(o) => { if !out.is_null() { unsafe { *out = Box::into_raw(o); } } 0 }
        Err(_) => -1,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn fluvio_c_produce_output_wait(out: *mut FluvioProduceOutput) -> i32 {
    if out.is_null() { return -1; }
    match produce_output_wait(unsafe { &mut *out }) { Ok(_) => 0, Err(_) => -1 }
}

#[unsafe(no_mangle)]
pub extern "C" fn fluvio_c_producer_flush(producer: *mut FluvioProducer) -> i32 {
    if producer.is_null() { return -1; }
    match producer_flush(unsafe { &*producer }) { Ok(_) => 0, Err(_) => -1 }
}

#[unsafe(no_mangle)]
pub extern "C" fn fluvio_c_producer_free(producer: *mut FluvioProducer) {
    if !producer.is_null() { unsafe { let _ = Box::from_raw(producer); } }
}

#[unsafe(no_mangle)]
pub extern "C" fn fluvio_c_produce_output_free(out: *mut FluvioProduceOutput) {
    if !out.is_null() { unsafe { let _ = Box::from_raw(out); } }
}

#[unsafe(no_mangle)]
pub extern "C" fn fluvio_c_partition_consumer(client: *mut FluvioClient, topic: *const c_char, partition: u32, out_consumer: *mut *mut FluvioConsumer) -> i32 {
    if client.is_null() || topic.is_null() || out_consumer.is_null() { return -1; }
    let topic_str = unsafe { CStr::from_ptr(topic).to_str() }.unwrap_or("");
    match partition_consumer(unsafe { &*client }, topic_str, partition) {
        Ok(consumer) => { unsafe { *out_consumer = Box::into_raw(consumer); } 0 }
        Err(_) => -1,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn fluvio_c_consumer_stream(consumer: *mut FluvioConsumer, offset_index: i64, out_stream: *mut *mut FluvioStream) -> i32 {
    if consumer.is_null() || out_stream.is_null() { return -1; }
    match consumer_stream(unsafe { &*consumer }, offset_index) {
        Ok(stream) => { unsafe { *out_stream = Box::into_raw(stream); } 0 }
        Err(_) => -1,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn fluvio_c_stream_next(stream: *mut FluvioStream, out_record: *mut *mut FluvioRecord) -> i32 {
    if stream.is_null() || out_record.is_null() { return -1; }
    match stream_next(unsafe { &mut *stream }) {
        Ok(record) => { unsafe { *out_record = Box::into_raw(record); } 0 }
        Err(_) => -1,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn fluvio_c_record_value(record: *mut FluvioRecord, out_buf: *mut *const u8, out_len: *mut usize) -> i32 {
    if record.is_null() || out_buf.is_null() || out_len.is_null() { return -1; }
    let val = unsafe { &*record }.inner.value();
    unsafe { *out_buf = val.as_ptr(); *out_len = val.len(); }
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn fluvio_c_record_free(rec: *mut FluvioRecord) {
    if !rec.is_null() { unsafe { let _ = Box::from_raw(rec); } }
}

#[unsafe(no_mangle)]
pub extern "C" fn fluvio_c_stream_free(stream: *mut FluvioStream) {
    if !stream.is_null() { unsafe { let _ = Box::from_raw(stream); } }
}

#[unsafe(no_mangle)]
pub extern "C" fn fluvio_c_consumer_free(consumer: *mut FluvioConsumer) {
    if !consumer.is_null() { unsafe { let _ = Box::from_raw(consumer); } }
}
