use docker_api::Docker;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let docker = Docker::new("tcp://127.0.0.1:80")?;
    match docker.volumes().list().await {
        Ok(volumes) => {
            for v in volumes {
                println!("volume -> {:#?}", v)
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}
