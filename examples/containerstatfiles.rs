use docker_api::Docker;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let docker = Docker::unix("/run/docker.sock");
    let id = env::args()
        .nth(1)
        .expect("You need to specify a container id");

    let path = env::args()
        .nth(2)
        .expect("You need to specify a path in container to stat");

    let stats = docker.containers().get(&id).stat_file(path).await?;

    println!("{}", stats);

    Ok(())
}
