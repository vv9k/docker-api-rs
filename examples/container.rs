#![allow(clippy::suspicious_else_formatting)]
mod common;
use clap::Parser;
use common::{new_docker, print_chunk};
use futures::StreamExt;
use std::path::PathBuf;

#[derive(Parser)]
struct Opts {
    #[clap(subcommand)]
    subcmd: Cmd,
}

#[derive(Parser)]
enum Cmd {
    /// Attach to a running containers TTY.
    Attach { id: String },
    /// Copy files from a container.
    CopyFrom {
        id: String,
        remote_path: PathBuf,
        local_path: PathBuf,
    },
    /// Copy files into a container.
    CopyInto {
        local_path: PathBuf,
        id: String,
        remote_path: PathBuf,
    },
    /// Create a new container.
    Create {
        image: String,
        #[clap(short, long = "name")] // for some reason naming field `name` makes clap error. Possibly a bug?
        /// The name of the container to create.
        nam: Option<String>,
    },
    /// Delete an existing container.
    Delete {
        id: String,
        #[clap(short, long)]
        force: bool,
    },
    /// Execute a command in a running container.
    Exec { id: String, cmd: Vec<String> },
    /// Inspect a container.
    Inspect { id: String },
    /// List active containers.
    List {
        #[clap(long, short)]
        /// List stopped and running containers.
        all: bool,
    },
    /// Print logs of a container.
    Logs {
        id: String,
        #[clap(long)]
        stdout: bool,
        #[clap(long)]
        stderr: bool,
    },
    /// Delete stopped containers.
    Prune {
        #[clap(long)]
        /// Prune containers before this timestamp. Can be a unix timestamp or duration
        /// string like `1h30m`
        until: Option<String>,
    },
    /// Get information about a file in container.
    StatFile { id: String, path: PathBuf },
    /// Returns usage statistics of the container.
    Stats { id: String },
    /// Returns information about running processes in the container.
    Top {
        id: String,
        /// Arguments passed to `ps` in the container.
        psargs: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let opts: Opts = Opts::parse();
    let docker = new_docker()?;

    match opts.subcmd {
        Cmd::Attach { id } => {
            let container = docker.containers().get(&id);
            let tty_multiplexer = container.attach().await?;

            let (mut reader, _writer) = tty_multiplexer.split();

            while let Some(tty_result) = reader.next().await {
                match tty_result {
                    Ok(chunk) => print_chunk(chunk),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
        }
        Cmd::CopyFrom {
            id,
            remote_path,
            local_path,
        } => {
            use futures::TryStreamExt;
            use tar::Archive;
            let bytes = docker
                .containers()
                .get(&id)
                .copy_from(&remote_path)
                .try_concat()
                .await?;

            let mut archive = Archive::new(&bytes[..]);
            archive.unpack(&local_path)?;
        }
        Cmd::CopyInto {
            local_path,
            id,
            remote_path,
        } => {
            use std::{fs::File, io::Read};

            let mut file = File::open(&local_path)?;
            let mut bytes = Vec::new();
            file.read_to_end(&mut bytes)
                .expect("Cannot read file on the localhost.");

            if let Err(e) = docker
                .containers()
                .get(&id)
                .copy_file_into(remote_path, &bytes)
                .await
            {
                eprintln!("Error: {}", e)
            }
        }
        Cmd::Create { image, nam } => {
            use docker_api::api::ContainerCreateOpts;
            let opts = if let Some(name) = nam {
                ContainerCreateOpts::builder(image).name(name).build()
            } else {
                ContainerCreateOpts::builder(image).build()
            };
            match docker.containers().create(&opts).await {
                Ok(info) => println!("{:?}", info),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Cmd::Delete { id, force } => {
            use docker_api::api::RmContainerOpts;

            let opts = if force {
                RmContainerOpts::builder().force(true).build()
            } else {
                Default::default()
            };
            if let Err(e) = docker.containers().get(&id).remove(&opts).await {
                eprintln!("Error: {}", e)
            }
        }
        Cmd::Exec { id, cmd } => {
            use docker_api::api::ExecContainerOpts;
            let options = ExecContainerOpts::builder()
                .cmd(cmd)
                .attach_stdout(true)
                .attach_stderr(true)
                .build();

            while let Some(exec_result) = docker.containers().get(&id).exec(&options).next().await {
                match exec_result {
                    Ok(chunk) => print_chunk(chunk),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
        }
        Cmd::Inspect { id } => {
            match docker.containers().get(&id).inspect().await {
                Ok(container) => println!("{:#?}", container),
                Err(e) => eprintln!("Error: {}", e),
            };
        }
        Cmd::List { all } => {
            use docker_api::api::ContainerListOpts;

            let opts = if all {
                ContainerListOpts::builder().all(true).build()
            } else {
                Default::default()
            };
            match docker.containers().list(&opts).await {
                Ok(containers) => {
                    containers.into_iter().for_each(|container| {
                        println!(
                            "{}\t{}\t{:?}\t{}\t{}",
                            &container.id[..12],
                            container.image,
                            container.state,
                            container.status,
                            container.names[0]
                        );
                    });
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Cmd::Logs { id, stdout, stderr } => {
            use docker_api::api::LogsOpts;
            let container = docker.containers().get(&id);
            let logs_stream =
                container.logs(&LogsOpts::builder().stdout(stdout).stderr(stderr).build());

            let logs: Vec<_> = logs_stream
                .map(|chunk| match chunk {
                    Ok(chunk) => chunk.to_vec(),
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        vec![]
                    }
                })
                .collect::<Vec<_>>()
                .await
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();
            print!("{}", String::from_utf8_lossy(&logs));
        }
        Cmd::Prune { until } => {
            use docker_api::api::{ContainerPruneFilter, ContainerPruneOpts};

            let opts = if let Some(until) = until {
                ContainerPruneOpts::builder()
                    .filter(vec![ContainerPruneFilter::Until(until)])
                    .build()
            } else {
                Default::default()
            };

            if let Err(e) = docker.containers().prune(&opts).await {
                eprintln!("Error: {}", e)
            }
        }
        Cmd::StatFile { id, path } => {
            let stats = docker.containers().get(&id).stat_file(path).await?;
            println!("{}", stats);
        }
        Cmd::Stats { id } => {
            while let Some(result) = docker.containers().get(&id).stats().next().await {
                match result {
                    Ok(stat) => println!("{:?}", stat),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
        }
        Cmd::Top { id, psargs } => {
            match docker.containers().get(&id).top(psargs.as_deref()).await {
                Ok(top) => println!("{:#?}", top),
                Err(e) => eprintln!("Error: {}", e),
            };
        }
    }

    Ok(())
}
