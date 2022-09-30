#![allow(dead_code)]

use std::env;
use std::path::PathBuf;

pub use docker_api::{api, conn, models, models::ImageBuildChunk, opts, Docker};
pub use futures_util::{StreamExt, TryStreamExt};
pub use tempfile::TempDir;

pub const DEFAULT_IMAGE: &str = "ubuntu:latest";
pub const DEFAULT_CMD: &str = "sleep inf";
pub const DEFAULT_CMD_ARRAY: &[&str] = &["sleep", "inf"];
pub const TEST_IMAGE_PATH: &str = "/var/test123";

const URI_ENV_VAR: &str = "DOCKER_API_URI";

pub fn init_runtime() -> Docker {
    let _ = env_logger::try_init();
    if let Ok(uri) = env::var(URI_ENV_VAR) {
        Docker::new(uri).unwrap()
    } else {
        #[cfg(unix)]
        {
            let uid = nix::unistd::Uid::effective();
            let docker_dir = PathBuf::from(format!("/run/user/{uid}/docker"));
            let docker_root_dir = PathBuf::from("/var/run");
            if docker_dir.exists() {
                Docker::unix(docker_dir.join("docker.sock"))
            } else if docker_root_dir.exists() {
                Docker::unix(docker_root_dir.join("docker.sock"))
            } else {
                panic!(
                    "Docker socket not found. Tried {URI_ENV_VAR} env variable, {} and {}",
                    docker_dir.display(),
                    docker_root_dir.display()
                );
            }
        }
        #[cfg(not(unix))]
        {
            panic!("Docker socket not found. Try setting the {URI_ENV_VAR} env variable",);
        }
    }
}

pub async fn create_base_container(
    docker: &Docker,
    name: &str,
    opts: Option<opts::ContainerCreateOpts>,
) -> api::Container {
    cleanup_container(docker, name).await;

    let opts = opts.unwrap_or_else(|| {
        opts::ContainerCreateOpts::builder()
            .image(DEFAULT_IMAGE)
            .name(name)
            .command(DEFAULT_CMD_ARRAY)
            .build()
    });
    docker
        .containers()
        .create(&opts)
        .await
        .expect("created base container");
    docker.containers().get(name)
}

pub async fn cleanup_container(docker: &Docker, name: &str) {
    let _ = docker
        .containers()
        .get(name)
        .remove(&opts::ContainerRemoveOpts::builder().force(true).build())
        .await;
}

pub async fn get_container_full_id(docker: &Docker, name: &str) -> String {
    docker
        .containers()
        .get(name)
        .inspect()
        .await
        .map(|data| data.id)
        .expect("container inspect data")
        .expect("container full id")
}

pub fn tempdir_with_dockerfile(content: Option<&str>) -> TempDir {
    let tmp = TempDir::new().expect("temp dir for image");
    let default_dockerfile = format!(
        "FROM {DEFAULT_IMAGE}\nRUN echo 1234 > {TEST_IMAGE_PATH}\nRUN echo 321\nCMD sleep inf",
    );

    std::fs::write(
        tmp.path().join("Dockerfile"),
        content.unwrap_or(default_dockerfile.as_str()),
    )
    .expect("saved Dockerfile");
    tmp
}

pub async fn create_base_image(
    docker: &Docker,
    tag: &str,
    opts: Option<opts::ImageBuildOpts>,
) -> api::Image {
    let images = docker.images();
    let _ = images
        .get(tag)
        .remove(
            &opts::ImageRemoveOpts::builder()
                .force(true)
                .noprune(true)
                .build(),
        )
        .await;

    let tmp = tempdir_with_dockerfile(None);

    println!("Tmp: {}", tmp.path().display());
    println!("Exists: {}", tmp.path().exists());
    let opts = opts.unwrap_or_else(|| opts::ImageBuildOpts::builder(tmp.path()).tag(tag).build());

    let mut image_stream = images.build(&opts);
    let mut digest = None;
    while let Some(chunk) = image_stream.next().await {
        println!("{chunk:?}");
        assert!(chunk.is_ok());
        if matches!(chunk, Ok(models::ImageBuildChunk::Digest { .. })) {
            digest = Some(chunk);
        }
    }

    match digest.unwrap().unwrap() {
        ImageBuildChunk::Digest { aux } => docker.images().get(aux.id),
        chunk => panic!("invalid chunk {chunk:?}"),
    }
}

pub async fn get_image_full_id(docker: &Docker, name: &str) -> String {
    docker
        .images()
        .get(name)
        .inspect()
        .await
        .map(|data| data.id)
        .expect("image inspect data")
        .expect("image full id")
}

pub async fn create_base_volume(
    docker: &Docker,
    name: &str,
    opts: Option<opts::VolumeCreateOpts>,
) -> api::Volume {
    cleanup_volume(docker, name).await;

    let opts = opts.unwrap_or_else(|| opts::VolumeCreateOpts::builder().name(name).build());
    docker
        .volumes()
        .create(&opts)
        .await
        .expect("created base volume");
    docker.volumes().get(name)
}

pub async fn cleanup_volume(docker: &Docker, name: &str) {
    let _ = docker.volumes().get(name).delete().await;
}

pub async fn create_base_network(
    docker: &Docker,
    name: &str,
    opts: Option<opts::NetworkCreateOpts>,
) -> api::Network {
    cleanup_network(docker, name).await;

    let opts = opts.unwrap_or_else(|| opts::NetworkCreateOpts::builder(name).build());
    docker
        .networks()
        .create(&opts)
        .await
        .expect("created base network");
    docker.networks().get(name)
}

pub async fn cleanup_network(docker: &Docker, name: &str) {
    let _ = docker.networks().get(name).delete().await;
}
