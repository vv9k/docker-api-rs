use docker_api::Docker;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let docker = Docker::new("tcp://127.0.0.1:80")?;
    println!("docker images in stock");

    let result = docker.images().list(&Default::default()).await;

    match result {
        Ok(images) => {
            for i in images {
                println!(
                    "{} {} {:?}",
                    i.id,
                    i.created,
                    i.repo_tags.unwrap_or_else(|| vec!["none".into()])
                );
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}
