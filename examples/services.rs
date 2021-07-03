#[cfg(feature = "swarm")]
use docker_api::{service::ListOpts, Docker};

#[cfg(feature = "swarm")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let docker = Docker::new("tcp://127.0.0.1:80")?;
    match docker
        .services()
        .list(&ListOpts::builder().enable_status().build())
        .await
    {
        Ok(services) => {
            for s in services {
                println!("service -> {:#?}", s)
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}

#[cfg(not(feature = "swarm"))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
