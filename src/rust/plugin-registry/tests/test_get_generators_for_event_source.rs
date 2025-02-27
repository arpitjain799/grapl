#![cfg(feature = "integration_tests")]

use std::time::Duration;

use bytes::Bytes;
use figment::{
    providers::Env,
    Figment,
};
use grapl_utils::future_ext::GraplFutureExt;
use rust_proto::graplinc::grapl::api::{
    client::{
        ClientError,
        Connect,
    },
    plugin_registry::v1beta1::{
        GetGeneratorsForEventSourceRequest,
        PluginMetadata,
        PluginRegistryClient,
        PluginType,
    },
    protocol::status::Code,
};

#[test_log::test(tokio::test)]
async fn test_get_generators_for_event_source() -> eyre::Result<()> {
    tracing::debug!(
        env=?std::env::args(),
    );

    let client_config = Figment::new()
        .merge(Env::prefixed("PLUGIN_REGISTRY_CLIENT_"))
        .extract()?;
    let mut client = PluginRegistryClient::connect(client_config).await?;

    let tenant_id = uuid::Uuid::new_v4();
    let generator1_display_name = "my first generator".to_string();
    let generator2_display_name = "my second generator".to_string();
    let analyzer_display_name = "my analyzer".to_string();
    let event_source_id = uuid::Uuid::new_v4();

    let generator1_metadata = PluginMetadata::new(
        tenant_id,
        generator1_display_name.clone(),
        PluginType::Generator,
        Some(event_source_id),
    );

    let generator2_metadata = PluginMetadata::new(
        tenant_id,
        generator2_display_name.clone(),
        PluginType::Generator,
        Some(event_source_id),
    );

    let analyzer_metadata = PluginMetadata::new(
        tenant_id,
        analyzer_display_name.clone(),
        PluginType::Analyzer,
        None,
    );

    let chunk = Bytes::from("chonk");

    let create_generator1_chunk = chunk.clone();
    let create_generator1_response = client
        .create_plugin(
            Duration::from_secs(60),
            generator1_metadata,
            futures::stream::once(async move { create_generator1_chunk }),
        )
        .timeout(Duration::from_secs(5))
        .await??;
    let generator1_plugin_id = create_generator1_response.plugin_id();

    let create_generator2_chunk = chunk.clone();
    let create_generator2_response = client
        .create_plugin(
            Duration::from_secs(60),
            generator2_metadata,
            futures::stream::once(async move { create_generator2_chunk }),
        )
        .timeout(Duration::from_secs(5))
        .await??;
    let generator2_plugin_id = create_generator2_response.plugin_id();

    let create_analyzer_chunk = chunk.clone();
    let create_analyzer_response = client
        .create_plugin(
            Duration::from_secs(60),
            analyzer_metadata,
            futures::stream::once(async move { create_analyzer_chunk }),
        )
        .timeout(Duration::from_secs(5))
        .await??;
    let analyzer_plugin_id = create_analyzer_response.plugin_id();

    let generators_for_event_source_response = client
        .get_generators_for_event_source(GetGeneratorsForEventSourceRequest::new(event_source_id))
        .timeout(Duration::from_secs(5))
        .await??;

    assert_eq!(generators_for_event_source_response.plugin_ids().len(), 2);
    assert!(generators_for_event_source_response
        .plugin_ids()
        .contains(&generator1_plugin_id));
    assert!(generators_for_event_source_response
        .plugin_ids()
        .contains(&generator2_plugin_id));
    assert!(!generators_for_event_source_response
        .plugin_ids()
        .contains(&analyzer_plugin_id));

    Ok(())
}

#[test_log::test(tokio::test)]
async fn test_get_generators_for_event_source_not_found() -> eyre::Result<()> {
    tracing::debug!(
        env=?std::env::args(),
    );

    let client_config = Figment::new()
        .merge(Env::prefixed("PLUGIN_REGISTRY_CLIENT_"))
        .extract()?;
    let mut client = PluginRegistryClient::connect(client_config).await?;

    let tenant_id = uuid::Uuid::new_v4();

    if let Err(e) = client
        .get_generators_for_event_source(GetGeneratorsForEventSourceRequest::new(tenant_id))
        .timeout(Duration::from_secs(5))
        .await?
    {
        match e {
            ClientError::Status(s) => {
                if let Code::NotFound = s.code() {
                    Ok(()) // 👍 great success 👍
                } else {
                    panic!("unexpected status code {}", s.code())
                }
            }
            e => panic!("unexpected error {}", e),
        }
    } else {
        panic!("expected error")
    }
}
