use docker_api::Docker;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let docker = Docker::new("tcp://127.0.0.1:80")?;

    match docker.info().await {
        Ok(info) => println!("info {:?}", info),
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}
