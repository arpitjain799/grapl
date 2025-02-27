use tonic::transport::Endpoint;

use crate::{
    graplinc::grapl::api::{
        client::{
            Client,
            ClientError,
            Connectable,
            WithClient,
        },
        graph_query_proxy::v1beta1::messages as native,
    },
    protobufs::graplinc::grapl::api::graph_query_proxy::v1beta1::graph_query_proxy_service_client::GraphQueryProxyServiceClient,
};

#[async_trait::async_trait]
impl Connectable for GraphQueryProxyServiceClient<tonic::transport::Channel> {
    async fn connect(endpoint: Endpoint) -> Result<Self, ClientError> {
        Ok(Self::connect(endpoint).await?)
    }
}

#[derive(Clone)]
pub struct GraphQueryProxyClient {
    client: Client<GraphQueryProxyServiceClient<tonic::transport::Channel>>,
}

impl WithClient<GraphQueryProxyServiceClient<tonic::transport::Channel>> for GraphQueryProxyClient {
    fn with_client(
        client: Client<GraphQueryProxyServiceClient<tonic::transport::Channel>>,
    ) -> Self {
        Self { client }
    }
}

impl GraphQueryProxyClient {
    pub async fn query_graph_with_uid(
        &mut self,
        request: native::QueryGraphWithUidRequest,
    ) -> Result<native::QueryGraphWithUidResponse, ClientError> {
        self.client
            .execute(
                request,
                None,
                |status| status.code() == tonic::Code::Unavailable,
                10,
                |mut client, request| async move { client.query_graph_with_uid(request).await },
            )
            .await
    }

    pub async fn query_graph_from_uid(
        &mut self,
        request: native::QueryGraphFromUidRequest,
    ) -> Result<native::QueryGraphFromUidResponse, ClientError> {
        self.client
            .execute(
                request,
                None,
                |status| status.code() == tonic::Code::Unavailable,
                10,
                |mut client, request| async move { client.query_graph_from_uid(request).await },
            )
            .await
    }
}
