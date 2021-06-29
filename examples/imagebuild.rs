use docker_api::{image::BuildOpts, Docker};
use futures::StreamExt;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let docker = Docker::new("tcp://127.0.0.1:80")?;
    let path = env::args().nth(1).expect("You need to specify a path");

    let options = BuildOpts::builder(path).tag("docker_api_test").build();

    let mut stream = docker.images().build(&options);
    while let Some(build_result) = stream.next().await {
        match build_result {
            Ok(output) => println!("{:?}", output),
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    Ok(())
}
