use docker_api::{
    api::{ContainerPruneFilter, ContainerPruneOpts},
    Docker,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let docker = Docker::new("tcp://127.0.0.1:80")?;

    if let Err(e) = docker
        .containers()
        .prune(
            &ContainerPruneOpts::builder()
                .filter(vec![
                    ContainerPruneFilter::LabelKeyVal("app".into(), "web".into()),
                    ContainerPruneFilter::Until("1h30m".into()),
                ])
                .build(),
        )
        .await
    {
        eprintln!("Error: {}", e)
    }

    Ok(())
}
