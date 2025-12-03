use std::sync::Arc;
use std::collections::HashMap;

use wash_runtime::{
    engine::Engine,
    host::{HostBuilder, HostApi,
        http::{HttpServer, DynamicRouter},
    },
    plugin::{
        wasi_config::WasiConfig,
    },
    types::{
        Workload, WorkloadStartRequest,
    },
};

use wash::plugin::PluginManager;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create a Wasmtime engine
    let engine = Engine::builder().build()?;

    // Create plugin manager to provide wasmcloud:wash interfaces
    let plugin_manager = PluginManager::default();

    // Configure plugins
    let http_router = DynamicRouter::default();
    let http_handler = HttpServer::new(
        http_router,
        "127.0.0.1:8080".parse()?
    );
    let wasi_config_plugin = WasiConfig::default();

    // Build and start the host
    let host = HostBuilder::new()
        .with_engine(engine)
        .with_http_handler(Arc::new(http_handler))
        .with_plugin(Arc::new(wasi_config_plugin))?
        .with_plugin(Arc::new(plugin_manager))?
        .build()?;

    let host = host.start().await?;

    // Start a workload
    let req = WorkloadStartRequest {
        workload_id: uuid::Uuid::new_v4().to_string(),
        workload: Workload {
            namespace: "test".to_string(),
            name: "test-workload".to_string(),
            annotations: HashMap::new(),
            service: None,
            components: vec![],
            host_interfaces: vec![],
            volumes: vec![],
        },
    };

    host.workload_start(req).await?;
    Ok(())
}
