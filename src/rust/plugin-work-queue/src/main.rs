use structopt::StructOpt;
use plugin_work_queue::PluginWorkQueueServiceConfig;
use plugin_work_queue::server::exec_service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (_env, _guard) = grapl_config::init_grapl_env!();
    let config = PluginWorkQueueServiceConfig::from_args();
    exec_service(config).await?;
    Ok(())
}
