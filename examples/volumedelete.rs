use docker_api::Docker;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let docker = Docker::new("tcp://127.0.0.1:80")?;

    let volume_name = env::args()
        .nth(1)
        .expect("You need to specify an volume name");

    if let Err(e) = docker.volumes().get(&volume_name).delete().await {
        eprintln!("Error: {}", e)
    }

    Ok(())
}
