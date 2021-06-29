use docker_api::Docker;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let docker = Docker::new("tcp://127.0.0.1:80")?;

    match docker.networks().list(&Default::default()).await {
        Ok(networks) => {
            for network in networks {
                println!("network -> {:#?}", network)
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}
