use docker_api::Docker;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let docker = Docker::new("tcp://127.0.0.1:80")?;
    let id = env::args()
        .nth(1)
        .expect("Usage: cargo run --example serviceinspect -- <service>");

    match docker.services().get(&id).inspect().await {
        Ok(service) => println!("{:#?}", service),
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}
