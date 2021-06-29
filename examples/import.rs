use docker_api::Docker;
use futures::StreamExt;
use std::{env, fs::File};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let docker = Docker::new("tcp://127.0.0.1:80")?;
    let path = env::args()
        .nth(1)
        .expect("You need to specify an image path");
    let f = File::open(path).expect("Unable to open file");

    let reader = Box::from(f);

    let mut stream = docker.images().import(reader);

    while let Some(import_result) = stream.next().await {
        match import_result {
            Ok(output) => println!("{:?}", output),
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    Ok(())
}
