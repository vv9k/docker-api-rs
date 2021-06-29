use docker_api::Docker;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let docker = Docker::new("tcp://127.0.0.1:80")?;
    match docker.version().await {
        Ok(ver) => println!("version -> {:#?}", ver),
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}
