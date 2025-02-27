use std::time::Duration;

use figment::{
    providers::Env,
    Figment,
};
use rust_proto::graplinc::grapl::api::{
    client::{
        ClientConfiguration,
        Connect,
    },
    plugin_work_queue::v1beta1::PluginWorkQueueClient,
};
use serde::{
    Deserialize,
    Serialize,
};
use uuid::Uuid;

use crate::work::{
    PluginWorkProcessor,
    Workload,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SidecarConfig {
    plugin_id: Uuid,
}

impl SidecarConfig {
    pub fn plugin_id(&self) -> Uuid {
        self.plugin_id
    }
}

pub struct PluginExecutor<P: PluginWorkProcessor> {
    plugin_work_processor: P,
    plugin_work_queue_client: PluginWorkQueueClient,
    plugin_id: Uuid,
}

impl<P> PluginExecutor<P>
where
    P: PluginWorkProcessor,
{
    pub async fn new(
        plugin_id: Uuid,
        plugin_work_processor: P,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let client_config: ClientConfiguration = Figment::new()
            .merge(Env::prefixed("PLUGIN_WORK_QUEUE_CLIENT_"))
            .extract()?;

        let plugin_work_queue_client = PluginWorkQueueClient::connect(client_config).await?;

        Ok(Self {
            plugin_work_processor,
            plugin_work_queue_client,
            plugin_id,
        })
    }

    #[tracing::instrument(skip(self), err)]
    pub async fn main_loop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Continually scan for new work for this Plugin.
        while let Ok(work) = self
            .plugin_work_processor
            .get_work(self.plugin_id, &mut self.plugin_work_queue_client)
            .await
        {
            let request_id = work.request_id();
            if let Some(job) = work.maybe_job() {
                let tenant_id = job.tenant_id();
                let trace_id = job.trace_id();
                let event_source_id = job.event_source_id();
                let plugin_id = self.plugin_id;

                tracing::debug!(
                    message = "retrieved execution job",
                    tenant_id =% tenant_id,
                    trace_id =% trace_id,
                    event_source_id =% event_source_id,
                    plugin_id =% plugin_id,
                    request_id =? request_id,
                );

                // Process the job
                let process_result = self
                    .plugin_work_processor
                    .process_job(self.plugin_id, job)
                    .await;

                if let Err(e) = process_result.as_ref() {
                    tracing::error!(
                        message = "error processing execution job",
                        request_id = ?request_id,
                        error = ?e,
                        tenant_id = %tenant_id,
                        trace_id = %trace_id,
                        event_source_id = %event_source_id,
                        plugin_id =% plugin_id,
                    );
                } else {
                    tracing::debug!(
                        message = "processed execution job successfully",
                        tenant_id =% tenant_id,
                        trace_id =% trace_id,
                        event_source_id =% event_source_id,
                        plugin_id =% plugin_id,
                        request_id =? request_id,
                    );
                }

                let should_ack = match process_result.as_ref() {
                    // If it's retriable, just don't ack - PWQ will make the message
                    // available again in 10 seconds.
                    Err(e) if e.is_retriable() => false,
                    // Otherwise, it's a perma-fail error or a success, so inform PWQ
                    Err(_) => true,
                    Ok(_) => true,
                };

                if should_ack {
                    self.plugin_work_processor
                        .ack_work(
                            self.plugin_id,
                            &mut self.plugin_work_queue_client,
                            process_result,
                            request_id,
                            tenant_id,
                            trace_id,
                            event_source_id,
                        )
                        .await?;
                }
            } else {
                let delay = Duration::from_secs(1);
                tracing::warn!(
                    message = "found no execution job",
                    request_id =% request_id,
                    delay =? delay,
                );
                tokio::time::sleep(delay).await; // FIXME: backoff?
                continue;
            }
        }
        Err("Unable to get new work".into())
    }
}
