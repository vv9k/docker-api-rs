mod common;

use common::init_runtime;

#[tokio::test]
async fn docker_info() {
    let docker = init_runtime();

    let info_result = docker.info().await;
    assert!(info_result.is_ok());
    let info_data = info_result.unwrap();
    assert_eq!(info_data.os_type.unwrap(), "linux".to_string());
}

#[tokio::test]
async fn docker_ping() {
    let docker = init_runtime();

    let ping_result = docker.ping().await;
    assert!(ping_result.is_ok());
    let ping_data = ping_result.unwrap();
    assert!(!ping_data.api_version.is_empty());
}

#[tokio::test]
async fn docker_version() {
    let docker = init_runtime();

    let version_result = docker.version().await;
    assert!(version_result.is_ok());
    let version_data = version_result.unwrap();

    let ping_result = docker.ping().await;
    assert!(ping_result.is_ok());
    let ping_data = ping_result.unwrap();

    assert_eq!(ping_data.api_version, version_data.api_version.unwrap());
}

#[tokio::test]
async fn docker_data_usage() {
    let docker = init_runtime();

    let du_result = docker.data_usage().await;
    assert!(du_result.is_ok());
    let _du_data = du_result.unwrap();
}
