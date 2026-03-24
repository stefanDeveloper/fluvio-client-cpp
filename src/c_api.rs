use crate::client::{FluvioClient, fluvio_connect};
use crate::producer::{FluvioProducer, create_producer, producer_send, producer_flush};
use crate::consumer::{FluvioConsumer, FluvioStream, FluvioRecord, partition_consumer, consumer_stream, stream_next};
use crate::produce_output::{FluvioProduceOutput, produce_output_wait};
use crate::config::{FluvioConfigWrapper, fluvio_config_load};
use std::os::raw::c_char;
use std::ffi::CStr;

#[repr(C)]
pub struct fluvio_config_t {
    _private: [u8; 0],
}

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
pub unsafe extern "C" fn fluvio_c_connect_with_config(config: *mut fluvio_config_t, out_client: *mut *mut FluvioClient) -> i32 {
    if config.is_null() || out_client.is_null() { return -1; }
    let config_wrapper = &mut *(config as *mut FluvioConfigWrapper);
    match crate::client::fluvio_connect_with_config(config_wrapper) {
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
pub extern "C" fn fluvio_c_config_load(out_config: *mut *mut fluvio_config_t) -> i32 {
    if out_config.is_null() { return -1; }
    match fluvio_config_load() {
        Ok(config) => { unsafe { *out_config = Box::into_raw(config) as *mut fluvio_config_t; } 0 }
        Err(_) => -1,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fluvio_c_config_set_endpoint(config: *mut fluvio_config_t, endpoint: *const std::ffi::c_char) {
    if config.is_null() || endpoint.is_null() { return; }
    let config_wrapper = &mut *(config as *mut FluvioConfigWrapper);
    let ep_str = std::ffi::CStr::from_ptr(endpoint).to_string_lossy();
    crate::config::fluvio_config_set_endpoint(config_wrapper, &ep_str);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fluvio_c_config_set_client_id(config: *mut fluvio_config_t, client_id: *const std::ffi::c_char) {
    if config.is_null() || client_id.is_null() { return; }
    let config_wrapper = &mut *(config as *mut FluvioConfigWrapper);
    let client_id_str = std::ffi::CStr::from_ptr(client_id).to_string_lossy();
    crate::config::fluvio_config_set_client_id(config_wrapper, &client_id_str);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fluvio_c_config_disable_tls(config: *mut fluvio_config_t) {
    if config.is_null() { return; }
    let config_wrapper = &mut *(config as *mut FluvioConfigWrapper);
    crate::config::fluvio_config_disable_tls(config_wrapper);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fluvio_c_config_set_anonymous_tls(config: *mut fluvio_config_t) {
    if config.is_null() { return; }
    let config_wrapper = &mut *(config as *mut FluvioConfigWrapper);
    crate::config::fluvio_config_set_anonymous_tls(config_wrapper);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fluvio_c_config_set_inline_tls(config: *mut fluvio_config_t, domain: *const std::ffi::c_char, key: *const std::ffi::c_char, cert: *const std::ffi::c_char, ca_cert: *const std::ffi::c_char) {
    if config.is_null() || domain.is_null() || key.is_null() || cert.is_null() || ca_cert.is_null() { return; }
    let config_wrapper = &mut *(config as *mut FluvioConfigWrapper);
    crate::config::fluvio_config_set_inline_tls(config_wrapper, 
        &std::ffi::CStr::from_ptr(domain).to_string_lossy(),
        &std::ffi::CStr::from_ptr(key).to_string_lossy(),
        &std::ffi::CStr::from_ptr(cert).to_string_lossy(),
        &std::ffi::CStr::from_ptr(ca_cert).to_string_lossy(),
    );
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fluvio_c_config_set_tls_file_paths(config: *mut fluvio_config_t, domain: *const std::ffi::c_char, key_path: *const std::ffi::c_char, cert_path: *const std::ffi::c_char, ca_cert_path: *const std::ffi::c_char) {
    if config.is_null() || domain.is_null() || key_path.is_null() || cert_path.is_null() || ca_cert_path.is_null() { return; }
    let config_wrapper = &mut *(config as *mut FluvioConfigWrapper);
    crate::config::fluvio_config_set_tls_file_paths(config_wrapper, 
        &std::ffi::CStr::from_ptr(domain).to_string_lossy(),
        &std::ffi::CStr::from_ptr(key_path).to_string_lossy(),
        &std::ffi::CStr::from_ptr(cert_path).to_string_lossy(),
        &std::ffi::CStr::from_ptr(ca_cert_path).to_string_lossy(),
    );
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
