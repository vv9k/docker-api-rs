use docker_api::{ContainerConnectionOptions, Docker};
use std::env;

async fn network_disconnect(
    container_id: &str,
    network_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let docker = Docker::new("tcp://127.0.0.1:80")?;
    if let Err(e) = docker
        .networks()
        .get(network_id)
        .disconnect(&ContainerConnectionOptions::builder(container_id).build())
        .await
    {
        eprintln!("Error: {}", e)
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    match (env::args().nth(1), env::args().nth(2)) {
        (Some(container_id), Some(network_id)) => {
            network_disconnect(&container_id, &network_id).await?;
        }
        _ => eprintln!("please provide a container_id and network_id"),
    }

    Ok(())
}
