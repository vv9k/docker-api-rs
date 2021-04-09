use moby::{Docker, VolumeCreateOptions};
use std::{collections::HashMap, env};

#[tokio::main]
async fn main() {
    let docker = Docker::new();

    let volume_name = env::args()
        .nth(1)
        .expect("You need to specify an volume name");

    let mut labels = HashMap::new();
    labels.insert("com.github", "moby");

    match docker
        .volumes()
        .create(
            &VolumeCreateOptions::builder()
                .name(volume_name.as_ref())
                .labels(&labels)
                .build(),
        )
        .await
    {
        Ok(info) => println!("{:?}", info),
        Err(e) => eprintln!("Error: {}", e),
    }
}
