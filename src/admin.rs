use fluvio::FluvioAdmin;
use fluvio_sc_schema::topic::TopicSpec;
use fluvio_future::task::run_block_on;

pub struct FluvioAdminClient { pub inner: FluvioAdmin }

pub fn fluvio_admin_connect() -> Result<Box<FluvioAdminClient>, String> {
    run_block_on(FluvioAdmin::connect()).map(|a| Box::new(FluvioAdminClient { inner: a })).map_err(|e| e.to_string())
}

pub fn admin_create_topic(admin: &FluvioAdminClient, topic: &str, partitions: i32, replicas: i32) -> Result<(), String> {
    run_block_on(admin.inner.create(topic.to_string(), false, TopicSpec::new_computed(partitions as u32, replicas as u32, None)))
        .map_err(|e| e.to_string())
}

pub fn admin_delete_topic(admin: &FluvioAdminClient, topic: &str) -> Result<(), String> {
    run_block_on(admin.inner.delete::<TopicSpec>(topic.to_string()))
        .map_err(|e| e.to_string())
}
