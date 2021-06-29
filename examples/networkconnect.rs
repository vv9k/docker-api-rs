use docker_api::{network::ContainerConnectionOpts, Docker};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let docker = Docker::new("tcp://127.0.0.1:80")?;
    let networks = docker.networks();

    match (env::args().nth(1), env::args().nth(2)) {
        (Some(container_id), Some(network_id)) => {
            if let Err(e) = networks
                .get(&network_id)
                .connect(&ContainerConnectionOpts::builder(&container_id).build())
                .await
            {
                eprintln!("Error: {}", e)
            }
        }
        _ => eprintln!("please provide a container_id and network_id"),
    }

    Ok(())
}
