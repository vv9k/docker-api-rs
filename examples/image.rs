mod common;
use clap::Parser;
use common::new_docker;
use futures::StreamExt;
use std::path::PathBuf;

#[derive(Parser)]
pub struct Opts {
    #[clap(subcommand)]
    subcmd: Cmd,
}

#[derive(Parser)]
enum Cmd {
    /// Build an image.
    Build {
        /// A path to the directory containing Dockerfile for the image.
        path: PathBuf,
        #[clap(default_value = "latest")]
        tag: String,
    },
    /// Delete an image.
    Delete {
        image: String,
        #[clap(short, long)]
        force: bool,
        #[clap(long)]
        noprune: bool,
    },
    /// Export an image as a tar archive.
    Export {
        image: String,
    },
    /// Inspect an image.
    Inspect {
        image: String,
    },
    Import {
        path: PathBuf,
    },
    /// List existing images.
    List {
        #[clap(long, short)]
        /// Show all images. By default only final layer images are shown.
        all: bool,
    },
    /// Pull an image from image registry.
    Pull {
        /// The name or id of the image to pull.
        image: String,
        /// Username in case authentication is required.
        username: Option<String>,
        /// Password in case authentication is required.
        password: Option<String>,
    },
    /// Search for an image.
    Search {
        image: String,
    },
    Tag {
        /// Repository containing the image to tag.
        repo: String,
        /// The name or id of the image to tag.
        image: String,
        tag: String,
    },
    Prune,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let docker = new_docker()?;
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        Cmd::Build { path, tag } => {
            use docker_api::api::BuildOpts;
            let options = BuildOpts::builder(path).tag(tag).build();

            let mut stream = docker.images().build(&options);
            while let Some(build_result) = stream.next().await {
                match build_result {
                    Ok(output) => println!("{:?}", output),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
        }
        Cmd::Delete {
            image,
            force,
            noprune,
        } => {
            use docker_api::api::RmImageOpts;
            let opts = RmImageOpts::builder().force(force).noprune(noprune).build();
            match docker.images().get(&image).remove(&opts).await {
                Ok(statuses) => {
                    for status in statuses {
                        println!("{:?}", status);
                    }
                }
                Err(e) => eprintln!("Error: {}", e),
            };
        }
        Cmd::Export { image } => {
            use docker_api::Error;
            use std::{fs::OpenOptions, io::Write};
            let mut export_file = OpenOptions::new()
                .write(true)
                .create(true)
                .open(format!("{}.tar", &image))?;

            while let Some(export_result) = docker.images().get(&image).export().next().await {
                match export_result.and_then(|bytes| export_file.write(&bytes).map_err(Error::from))
                {
                    Ok(n) => println!("copied {} bytes", n),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
        }
        Cmd::Inspect { image } => {
            match docker.images().get(&image).inspect().await {
                Ok(image) => println!("{:#?}", image),
                Err(e) => eprintln!("Error: {}", e),
            };
        }
        Cmd::Import { path } => {
            use std::fs::File;
            let f = File::open(path).expect("Unable to open file");

            let reader = Box::from(f);

            let mut stream = docker.images().import(reader);

            while let Some(import_result) = stream.next().await {
                match import_result {
                    Ok(output) => println!("{:?}", output),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
        }
        Cmd::List { all } => {
            use docker_api::api::ImageListOpts;

            let opts = if all {
                ImageListOpts::builder().all(true).build()
            } else {
                Default::default()
            };
            match docker.images().list(&opts).await {
                Ok(images) => {
                    images.into_iter().for_each(|image| {
                        println!(
                            "---------------------------------\nCreated: {}\nId: {}\nRepo tags: {}\nLabels:\n{}",
                            image.created,
                            image.id,
                            image.repo_tags.unwrap_or_default().join(","),
                            image
                                .labels
                                .unwrap_or_default()
                                .into_iter()
                                .map(|(k, v)| format!(" - {}={}", k, v))
                                .collect::<Vec<_>>()
                                .join("\n"),
                        );
                    });
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Cmd::Pull {
            image,
            username,
            password,
        } => {
            use docker_api::api::{PullOpts, RegistryAuth};
            let opts = if let (Some(username), Some(pass)) = (username, password) {
                let auth = RegistryAuth::builder()
                    .username(username)
                    .password(pass)
                    .build();
                PullOpts::builder().image(image).auth(auth).build()
            } else {
                PullOpts::builder().image(image).build()
            };

            let mut stream = docker.images().pull(&opts);

            while let Some(pull_result) = stream.next().await {
                match pull_result {
                    Ok(output) => println!("{:?}", output),
                    Err(e) => eprintln!("{}", e),
                }
            }
        }
        Cmd::Search { image } => {
            match docker.images().search(image).await {
                Ok(results) => {
                    for result in results {
                        println!("{} - {}", result.name, result.description);
                    }
                }
                Err(e) => eprintln!("Error: {}", e),
            };
        }
        Cmd::Tag {
            repo,
            image: name,
            tag,
        } => {
            use docker_api::api::{Image, TagOpts};

            let tag_opts = TagOpts::builder().repo(repo).tag(tag).build();

            let image = Image::new(&docker, name);

            if let Err(e) = image.tag(&tag_opts).await {
                eprintln!("Error: {}", e)
            }
        }
        Cmd::Prune => {
            match docker.images().prune(&Default::default()).await {
                Ok(info) => println!("{:#?}", info),
                Err(e) => eprintln!("Error: {}", e),
            };
        }
    }

    Ok(())
}
