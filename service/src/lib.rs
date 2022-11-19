mod function_service;
mod trigger_service;

use std::pin::Pin;

use abi::{
    trigger::trigger_service_server::TriggerServiceServer, trigger::Trigger, Config, Function,
    FunctionServiceServer,
};
use function::DefaultFunctionManager;
use futures::Stream;
use tokio::sync::mpsc;
use tonic::{transport::Server, Status};
use tracing::info;
use trigger::DefaultTriggerManager;

pub struct TrigrService {
    manager: DefaultTriggerManager,
    function_manager: DefaultFunctionManager,
}

pub struct FuncService {
    manager: DefaultFunctionManager,
}

pub struct TonicReceiverStream<T> {
    inner: mpsc::Receiver<Result<T, abi::Error>>,
}

impl<T> TonicReceiverStream<T> {
    pub fn new(inner: mpsc::Receiver<Result<T, abi::Error>>) -> Self {
        Self { inner }
    }
}

pub type TriggerStream = Pin<Box<dyn Stream<Item = Result<Trigger, Status>> + Send>>;

pub type FunctionStream = Pin<Box<dyn Stream<Item = Result<Function, Status>> + Send>>;

pub async fn start_server(config: &Config) -> Result<(), anyhow::Error> {
    let addr = format!("{}:{}", config.server.host, config.server.port).parse()?;

    let trigr_svc = TriggerServiceServer::new(TrigrService::from_config(config).await?);
    let func_svc = FunctionServiceServer::new(FuncService::from_config(config).await?);

    info!("Listening on {}", addr);
    // api services
    Server::builder()
        .add_service(trigr_svc)
        .add_service(func_svc)
        .serve(addr)
        .await?;

    Ok(())
}
