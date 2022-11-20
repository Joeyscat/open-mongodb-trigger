#[path = "../src/test_utils.rs"]
mod test_utils;

use std::time::Duration;

use abi::{
    trigger::{trigger_service_client::TriggerServiceClient, CreateRequest, DatabaseConfig},
    Config, Trigger,
};

use service::start_server;
use test_utils::{rand_str, TestConfig};
use tonic::transport::Channel;

#[tokio::test]
async fn grpc_server_should_work() {
    let tconfig = TestConfig::with_server_port(40000);
    let mut client = get_test_client(&tconfig).await;

    // first we make a trigger

    let mut tr = Trigger::new_database(
        rand_str(),
        rand_str(),
        rand_str(),
        false,
        DatabaseConfig {
            ..Default::default()
        },
    );

    let ret = client
        .create(CreateRequest {
            trigger: Some(tr.clone()),
        })
        .await
        .unwrap()
        .into_inner()
        .trigger
        .unwrap();

    tr.id = ret.id.clone();
    assert_eq!(ret, tr);
}

async fn get_test_client(tcinfig: &TestConfig) -> TriggerServiceClient<Channel> {
    let config = &tcinfig.config;
    setup_server(config);

    let fut = async move {
        // if error on conn keep retry util timeout
        while TriggerServiceClient::connect(config.server.url(false))
            .await
            .is_err()
        {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
        TriggerServiceClient::connect(config.server.url(false))
            .await
            .unwrap()
    };

    tokio::time::timeout(Duration::from_secs(5), fut)
        .await
        .unwrap()
}

fn setup_server(config: &Config) {
    let config_cloned = config.clone();
    tokio::spawn(async move {
        start_server(&config_cloned).await.unwrap();
    });
}
