use std::time::Duration;

use futures::{
    channel::oneshot::{
        self,
        Receiver,
        Sender,
    },
    Future,
    FutureExt,
};
use tokio::net::TcpListener;
use tokio_stream::wrappers::TcpListenerStream;
use tonic::transport::{
    NamedService,
    Server,
};

use crate::{
    execute_rpc,
    graplinc::grapl::api::{
        pipeline_ingress::v1beta1::{
            PublishRawLogRequest,
            PublishRawLogResponse,
        },
        protocol::{
            error::ServeError,
            healthcheck::{
                server::init_health_service,
                HealthcheckError,
                HealthcheckStatus,
            },
            status::Status,
        },
        server::GrpcApi,
    },
    protobufs::graplinc::grapl::api::pipeline_ingress::v1beta1::{
        pipeline_ingress_service_server::{
            PipelineIngressService as PipelineIngressServiceProto,
            PipelineIngressServiceServer as PipelineIngressServiceServerProto,
        },
        PublishRawLogRequest as PublishRawLogRequestProto,
        PublishRawLogResponse as PublishRawLogResponseProto,
    },
};

//
// protocol buffer stuff
//

#[tonic::async_trait]
impl<T> PipelineIngressServiceProto for GrpcApi<T>
where
    T: PipelineIngressApi + Send + Sync + 'static,
{
    async fn publish_raw_log(
        &self,
        request: tonic::Request<PublishRawLogRequestProto>,
    ) -> Result<tonic::Response<PublishRawLogResponseProto>, tonic::Status> {
        execute_rpc!(self, request, publish_raw_log)
    }
}

//
// public API
//

/// Implement this trait to define the pipeline ingress API's business logic
#[tonic::async_trait]
pub trait PipelineIngressApi {
    type Error: Into<Status>;
    async fn publish_raw_log(
        &self,
        request: PublishRawLogRequest,
    ) -> Result<PublishRawLogResponse, Self::Error>;
}

/// The pipeline-ingress server serves the pipeline-ingress API
pub struct PipelineIngressServer<T, H, F>
where
    T: PipelineIngressApi + Send + Sync + 'static,
    H: Fn() -> F + Send + Sync + 'static,
    F: Future<Output = Result<HealthcheckStatus, HealthcheckError>> + Send + Sync + 'static,
{
    api_server: T,
    healthcheck: H,
    healthcheck_polling_interval: Duration,
    tcp_listener: TcpListener,
    shutdown_rx: Receiver<()>,
    service_name: &'static str,
}

impl<T, H, F> PipelineIngressServer<T, H, F>
where
    T: PipelineIngressApi + Send + Sync + 'static,
    H: Fn() -> F + Send + Sync + 'static,
    F: Future<Output = Result<HealthcheckStatus, HealthcheckError>> + Send + Sync + 'static,
{
    /// Construct a new gRPC server which will serve the given API
    /// implementation on the given socket address. Server is constructed in
    /// a non-running state. Call the serve() method to run the server. This
    /// method also returns a channel you can use to trigger server
    /// shutdown.
    pub fn new(
        api_server: T,
        tcp_listener: TcpListener,
        healthcheck: H,
        healthcheck_polling_interval: Duration,
    ) -> (Self, Sender<()>) {
        let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();
        (
            PipelineIngressServer {
                api_server,
                healthcheck,
                healthcheck_polling_interval,
                tcp_listener,
                shutdown_rx,
                service_name: PipelineIngressServiceServerProto::<GrpcApi<T>>::NAME,
            },
            shutdown_tx,
        )
    }

    /// returns the service name associated with this service. You will need
    /// this value to construct a HealthcheckClient with which to query this
    /// service's healthcheck.
    pub fn service_name(&self) -> &'static str {
        self.service_name
    }

    /// Run the gRPC server and serve the API on this server's socket
    /// address. Returns a ServeError if the gRPC server cannot run.
    #[tracing::instrument(skip(self), err)]
    pub async fn serve(self) -> Result<(), ServeError> {
        // TODO: add tower tracing, tls_config, concurrency limits
        let (healthcheck_handle, health_service) =
            init_health_service::<PipelineIngressServiceServerProto<GrpcApi<T>>, _, _>(
                self.healthcheck,
                self.healthcheck_polling_interval,
            )
            .await;
        Ok(Server::builder()
            .trace_fn(|_request| tracing::info_span!("pipeline-ingress"))
            .add_service(health_service)
            .add_service(PipelineIngressServiceServerProto::new(GrpcApi::new(
                self.api_server,
            )))
            .serve_with_incoming_shutdown(
                TcpListenerStream::new(self.tcp_listener),
                self.shutdown_rx.map(|_| ()),
            )
            .then(|result| async move {
                healthcheck_handle.abort();
                result
            })
            .await?)
    }
}
