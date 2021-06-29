use docker_api::Docker;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let docker = Docker::new("tcp://127.0.0.1:80")?;
    match docker.containers().list(&Default::default()).await {
        Ok(containers) => {
            for c in containers {
                println!("container -> {:#?}", c)
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}
