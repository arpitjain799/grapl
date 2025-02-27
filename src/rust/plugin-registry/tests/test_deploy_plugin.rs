#![cfg(feature = "integration_tests")]

use std::time::Duration;

use figment::{
    providers::Env,
    Figment,
};
use grapl_utils::future_ext::GraplFutureExt;
use integration_test_utils::{
    plugin_health::assert_eventual_health,
    test_fixtures::{
        get_example_generator,
        get_suspicious_svchost_analyzer,
        get_sysmon_generator,
    },
};
use rust_proto::graplinc::grapl::api::{
    client::{
        ClientError,
        Connect,
    },
    plugin_registry::v1beta1::{
        DeployPluginRequest,
        GetPluginDeploymentRequest,
        GetPluginHealthRequest,
        GetPluginHealthResponse,
        PluginDeploymentStatus,
        PluginHealthStatus,
        PluginMetadata,
        PluginRegistryClient,
        PluginType,
        TearDownPluginRequest,
    },
    protocol::status::Code,
};

#[test_log::test(tokio::test)]
async fn test_deploy_example_generator() -> eyre::Result<()> {
    let client_config = Figment::new()
        .merge(Env::prefixed("PLUGIN_REGISTRY_CLIENT_"))
        .extract()?;
    let mut client = PluginRegistryClient::connect(client_config).await?;

    let tenant_id = uuid::Uuid::new_v4();
    let event_source_id = uuid::Uuid::new_v4();

    let create_response = {
        let display_name = uuid::Uuid::new_v4().to_string();
        let artifact = get_example_generator()?;
        let metadata = PluginMetadata::new(
            tenant_id,
            display_name.clone(),
            PluginType::Generator,
            Some(event_source_id.clone()),
        );

        client
            .create_plugin(
                Duration::from_secs(60),
                metadata,
                futures::stream::once(async move { artifact.clone() }),
            )
            .timeout(Duration::from_secs(5))
            .await??
    };

    let plugin_id = create_response.plugin_id();

    let request = DeployPluginRequest::new(plugin_id);

    let _response = client
        .deploy_plugin(request)
        .timeout(Duration::from_secs(5))
        .await??;

    let plugin_deployment = client
        .get_plugin_deployment(GetPluginDeploymentRequest::new(plugin_id))
        .await?
        .plugin_deployment();

    assert_eq!(plugin_deployment.plugin_id(), plugin_id);
    assert!(plugin_deployment.deployed());
    assert_eq!(plugin_deployment.status(), PluginDeploymentStatus::Success);

    Ok(())
}

#[test_log::test(tokio::test)]
async fn test_deploy_sysmon_generator() -> eyre::Result<()> {
    let client_config = Figment::new()
        .merge(Env::prefixed("PLUGIN_REGISTRY_CLIENT_"))
        .extract()?;
    let mut client = PluginRegistryClient::connect(client_config).await?;

    let tenant_id = uuid::Uuid::new_v4();
    let event_source_id = uuid::Uuid::new_v4();

    let create_response = {
        let display_name = "sysmon-generator";
        let artifact = get_sysmon_generator()?;
        let metadata = PluginMetadata::new(
            tenant_id,
            display_name.to_owned(),
            PluginType::Generator,
            Some(event_source_id.clone()),
        );

        client
            .create_plugin(
                Duration::from_secs(60),
                metadata,
                futures::stream::once(async move { artifact.clone() }),
            )
            .timeout(Duration::from_secs(5))
            .await??
    };

    let plugin_id = create_response.plugin_id();

    // Ensure that an un-deployed plugin is NotDeployed
    assert_health(&mut client, plugin_id, PluginHealthStatus::NotDeployed).await?;

    tracing::info!(
        message = "test_deploy_sysmon_generator IDs",
        tenant_id =? tenant_id,
        plugin_id =? plugin_id,
    );

    let _deploy_response = client
        .deploy_plugin(DeployPluginRequest::new(plugin_id))
        .timeout(Duration::from_secs(5))
        .await??;

    // Ensure that a now-deployed plugin is now Running
    // If it's Pending, it's possible the agent is out of mem or disk
    // and was unable to allocate it.
    assert_eventual_health(
        &client,
        plugin_id,
        PluginHealthStatus::Running,
        Duration::from_secs(60),
    )
    .await?;

    Ok(())
}

#[test_log::test(tokio::test)]
async fn test_deploy_suspicious_svchost_analyzer() -> eyre::Result<()> {
    let client_config = Figment::new()
        .merge(Env::prefixed("PLUGIN_REGISTRY_CLIENT_"))
        .extract()?;
    let mut client = PluginRegistryClient::connect(client_config).await?;

    let tenant_id = uuid::Uuid::new_v4();

    let create_response = {
        let display_name = "suspicious-svchost";
        let artifact = get_suspicious_svchost_analyzer()?;
        let metadata = PluginMetadata::new(
            tenant_id,
            display_name.to_owned(),
            PluginType::Analyzer,
            None,
        );

        client
            .create_plugin(
                Duration::from_secs(60),
                metadata,
                futures::stream::once(async move { artifact.clone() }),
            )
            .timeout(Duration::from_secs(5))
            .await??
    };

    let plugin_id = create_response.plugin_id();

    // Ensure that an un-deployed plugin is NotDeployed
    assert_health(&mut client, plugin_id, PluginHealthStatus::NotDeployed).await?;

    let _deploy_response = client
        .deploy_plugin(DeployPluginRequest::new(plugin_id))
        .timeout(Duration::from_secs(5))
        .await??;

    // Ensure that a now-deployed plugin is now Running
    // If it's Pending, it's possible the agent is out of mem or disk
    // and was unable to allocate it.
    assert_eventual_health(
        &client,
        plugin_id,
        PluginHealthStatus::Running,
        Duration::from_secs(60),
    )
    .await?;

    Ok(())
}

fn assert_contains(input: &str, expected_substr: &str) {
    assert!(
        input.contains(expected_substr),
        "Expected input '{input}' to contain '{expected_substr}'"
    )
}

async fn assert_health(
    client: &mut PluginRegistryClient,
    plugin_id: uuid::Uuid,
    expected: PluginHealthStatus,
) -> eyre::Result<()> {
    let get_health_response: GetPluginHealthResponse = client
        .get_plugin_health(GetPluginHealthRequest::new(plugin_id))
        .timeout(Duration::from_secs(5))
        .await??;

    let actual = get_health_response.health_status();
    if expected == actual {
        Ok(())
    } else {
        Err(eyre::eyre!("Expected one of {expected:?}, got {actual:?}"))
    }
}

#[test_log::test(tokio::test)]
/// So we *expect* this call to fail since it's an arbitrary PluginID that
/// hasn't been created yet
async fn test_deploy_plugin_but_plugin_id_doesnt_exist() -> eyre::Result<()> {
    let client_config = Figment::new()
        .merge(Env::prefixed("PLUGIN_REGISTRY_CLIENT_"))
        .extract()?;
    let mut client = PluginRegistryClient::connect(client_config).await?;

    let randomly_selected_plugin_id = uuid::Uuid::new_v4();

    let request = DeployPluginRequest::new(randomly_selected_plugin_id);

    let response = client
        .deploy_plugin(request)
        .timeout(Duration::from_secs(5))
        .await?;

    match response {
        Err(ClientError::Status(s)) => {
            assert_eq!(s.code(), Code::NotFound);
            assert_contains(s.message(), &sqlx::Error::RowNotFound.to_string());
        }
        _ => panic!("Expected an error"),
    };
    Ok(())
}

#[test_log::test(tokio::test)]
async fn test_teardown_plugin() {
    let client_config = Figment::new()
        .merge(Env::prefixed("PLUGIN_REGISTRY_CLIENT_"))
        .extract()
        .expect("failed to configure plugin-registry client");
    let mut client = PluginRegistryClient::connect(client_config)
        .await
        .expect("failed to connect to plugin-registry");

    let tenant_id = uuid::Uuid::new_v4();
    let event_source_id = uuid::Uuid::new_v4();

    let create_response = {
        let display_name = "sysmon-generator";
        let artifact = get_sysmon_generator().expect("failed to get sysmon generator");
        let metadata = PluginMetadata::new(
            tenant_id,
            display_name.to_owned(),
            PluginType::Generator,
            Some(event_source_id.clone()),
        );

        client
            .create_plugin(
                Duration::from_secs(60),
                metadata,
                futures::stream::once(async move { artifact.clone() }),
            )
            .timeout(Duration::from_secs(5))
            .await
            .expect("timeout elapsed")
            .expect("failed to create plugin")
    };

    let plugin_id = create_response.plugin_id();

    // Ensure that an un-deployed plugin is NotDeployed
    assert_health(&mut client, plugin_id, PluginHealthStatus::NotDeployed)
        .await
        .expect("failed to assert health");

    client
        .deploy_plugin(DeployPluginRequest::new(plugin_id))
        .timeout(Duration::from_secs(5))
        .await
        .expect("timeout elapsed")
        .expect("failed to deploy plugin");

    client
        .tear_down_plugin(TearDownPluginRequest::new(plugin_id))
        .timeout(Duration::from_secs(5))
        .await
        .expect("timeout elapsed")
        .expect("failed to tear down plugin");

    let plugin_deployment = client
        .get_plugin_deployment(GetPluginDeploymentRequest::new(plugin_id))
        .await
        .expect("failed to get plugin deployment")
        .plugin_deployment();

    assert_eq!(plugin_deployment.plugin_id(), plugin_id);
    assert!(!plugin_deployment.deployed());
    assert_eq!(plugin_deployment.status(), PluginDeploymentStatus::Success);
}
