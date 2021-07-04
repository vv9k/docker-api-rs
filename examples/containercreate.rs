use docker_api::{container::ContainerCreateOpts, Docker};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let docker = Docker::new("tcp://127.0.0.1:80")?;
    let image = env::args()
        .nth(1)
        .expect("You need to specify an image name");

    match docker
        .containers()
        .create(&ContainerCreateOpts::builder(image).build())
        .await
    {
        Ok(info) => println!("{:?}", info),
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}
