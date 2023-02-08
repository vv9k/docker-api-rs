#![cfg(unix)] //temporary
mod common;

use common::{create_base_container, create_base_network, init_runtime, opts};

#[tokio::test]
async fn network_create_inspect_delete() {
    let docker = init_runtime();

    let network = create_base_network(&docker, "test-create-network", None).await;

    assert!(network.inspect().await.is_ok());
    assert!(network.delete().await.is_ok());
    assert!(network.inspect().await.is_err());
    let network = create_base_network(&docker, "test-create-network", None).await;
    assert!(network.inspect().await.is_ok());
    assert!(network.delete().await.is_ok());
    assert!(network.inspect().await.is_err());
}

#[tokio::test]
async fn network_inspect() {
    let docker = init_runtime();
    let networks = docker.networks();

    let network_name = "test-inspect-network";
    create_base_network(&docker, network_name, None).await;

    let network = networks.get(network_name);

    let inspect_result = network.inspect().await;
    assert!(inspect_result.is_ok());
    let inspect_data = inspect_result.unwrap();
    assert!(inspect_data.name.as_ref().unwrap().contains(network_name));
    assert!(network.delete().await.is_ok());
}

#[tokio::test]
async fn network_prune() {
    let docker = init_runtime();
    let networks = docker.networks();

    let name_a = "test-prune-network";
    let name_b = "test-prune-network2";

    let label_key = "test-prune";
    let value_a = "value_a";
    let value_b = "value_b";
    let opts_a = opts::NetworkCreateOpts::builder(name_a)
        .labels([(label_key, value_a)])
        .build();
    let opts_b = opts::NetworkCreateOpts::builder(name_b)
        .labels([(label_key, value_b)])
        .build();

    create_base_network(&docker, name_a, Some(opts_a.clone())).await;
    create_base_network(&docker, name_b, Some(opts_b.clone())).await;
    let network_a = networks.get(name_a);
    let network_b = networks.get(name_b);
    assert!(network_a.inspect().await.is_ok());
    assert!(network_b.inspect().await.is_ok());

    let filter = opts::NetworkPruneFilter::LabelKey(label_key.to_string());
    let prune_opts = opts::NetworkPruneOpts::builder().filter([filter]).build();
    let prune_result = networks.prune(&prune_opts).await;
    assert!(prune_result.is_ok());
    let prune_data = prune_result.unwrap().networks_deleted.unwrap_or_default();
    assert!(prune_data.iter().any(|name| name == name_a));
    assert!(prune_data.iter().any(|name| name == name_b));
    assert!(network_a.inspect().await.is_err());
    assert!(network_b.inspect().await.is_err());

    create_base_network(&docker, name_a, Some(opts_a.clone())).await;
    create_base_network(&docker, name_b, Some(opts_b.clone())).await;
    let network_a = networks.get(name_a);
    let network_b = networks.get(name_b);
    assert!(network_a.inspect().await.is_ok());
    assert!(network_b.inspect().await.is_ok());

    let filter = opts::NetworkPruneFilter::Label(label_key.to_string(), value_a.to_string());
    let prune_opts = opts::NetworkPruneOpts::builder().filter([filter]).build();
    let prune_result = networks.prune(&prune_opts).await;
    assert!(prune_result.is_ok());
    let prune_data = prune_result.unwrap().networks_deleted.unwrap_or_default();
    assert!(prune_data.iter().any(|name| name == name_a));
    assert!(!prune_data.iter().any(|name| name == name_b));
    assert!(network_a.inspect().await.is_err());
    assert!(network_b.inspect().await.is_ok());

    let filter = opts::NetworkPruneFilter::Label(label_key.to_string(), value_b.to_string());
    let prune_opts = opts::NetworkPruneOpts::builder().filter([filter]).build();
    let prune_result = networks.prune(&prune_opts).await;
    assert!(prune_result.is_ok());
    let prune_data = prune_result.unwrap().networks_deleted.unwrap_or_default();
    assert!(prune_data.iter().any(|name| name == name_b));

    assert!(network_a.inspect().await.is_err());
    assert!(network_b.inspect().await.is_err());
}

#[tokio::test]
async fn network_list() {
    let docker = init_runtime();
    let networks = docker.networks();

    let name_a = "test-list-network";
    let name_b = "test-list-network2";

    let label_key = "test-list";
    let value_a = "value_a";
    let value_b = "value_b";
    let opts_a = opts::NetworkCreateOpts::builder(name_a)
        .labels([(label_key, value_a)])
        .build();
    let opts_b = opts::NetworkCreateOpts::builder(name_b)
        .labels([(label_key, value_b)])
        .build();

    create_base_network(&docker, name_a, Some(opts_a.clone())).await;
    create_base_network(&docker, name_b, Some(opts_b.clone())).await;
    let network_a = networks.get(name_a);
    let network_b = networks.get(name_b);

    let filter = opts::NetworkFilter::LabelKey(label_key.to_string());
    let list_opts = opts::NetworkListOpts::builder().filter([filter]).build();
    let list_result = networks.list(&list_opts).await;
    assert!(list_result.is_ok());
    let list_data = list_result.unwrap();
    assert_eq!(list_data.len(), 2);
    assert!(list_data
        .iter()
        .any(|data| data.name.as_ref().unwrap() == name_a));
    assert!(list_data
        .iter()
        .any(|data| data.name.as_ref().unwrap() == name_b));

    let filter = opts::NetworkFilter::LabelKeyVal(label_key.to_string(), value_a.to_string());
    let list_opts = opts::NetworkListOpts::builder().filter([filter]).build();
    let list_result = networks.list(&list_opts).await;
    // This sometimes breaks when running all tests at the same time
    assert!(list_result.is_ok());
    let list_data = list_result.unwrap();
    assert_eq!(list_data.len(), 1);
    assert!(list_data
        .iter()
        .any(|data| data.name.as_ref().unwrap() == name_a));
    assert!(!list_data
        .iter()
        .any(|data| data.name.as_ref().unwrap() == name_b));

    let filter = opts::NetworkFilter::LabelKeyVal(label_key.to_string(), value_b.to_string());
    let list_opts = opts::NetworkListOpts::builder().filter([filter]).build();
    let list_result = networks.list(&list_opts).await;
    assert!(list_result.is_ok());
    let list_data = list_result.unwrap();
    assert_eq!(list_data.len(), 1);
    assert!(!list_data
        .iter()
        .any(|data| data.name.as_ref().unwrap() == name_a));
    assert!(list_data
        .iter()
        .any(|data| data.name.as_ref().unwrap() == name_b));

    let _ = network_a.delete().await;
    let _ = network_b.delete().await;
}

#[tokio::test]
async fn network_connect_disconnect() {
    let docker = init_runtime();

    let network_name = "test-connect-network";
    let container_name = "test-connect-network-container";
    let network = create_base_network(&docker, network_name, None).await;

    let container = create_base_container(&docker, container_name, None).await;

    let opts = opts::ContainerConnectionOpts::builder(container_name).build();

    let connect_result = network.connect(&opts).await;
    assert!(connect_result.is_ok());
    connect_result.unwrap();

    let container_data = container.inspect().await.unwrap();
    assert!(container_data
        .network_settings
        .unwrap()
        .networks
        .unwrap()
        .get(network_name)
        .is_some());

    let _ = network.delete().await;
    let _ = container.delete().await;
}
