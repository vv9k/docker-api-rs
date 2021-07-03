#[cfg(feature = "swarm")]
use {docker_api::Docker, std::env};

#[cfg(feature = "swarm")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let docker = Docker::new("tcp://127.0.0.1:80")?;
    let id = env::args()
        .nth(1)
        .expect("You need to specify an service name");

    if let Err(e) = docker.services().get(&id).delete().await {
        eprintln!("Error: {}", e)
    }

    Ok(())
}

#[cfg(not(feature = "swarm"))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
