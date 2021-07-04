mod common;
use clap::Clap;
use common::new_docker;

#[derive(Clap)]
pub struct Opts {
    #[clap(subcommand)]
    subcmd: Cmd,
}

#[derive(Clap)]
enum Cmd {
    /// Connect a container to a network.
    Connect {
        container: String,
        network: String,
    },
    /// Create a new network.
    Create {
        network: String,
        #[clap(default_value = "bridge")]
        driver: String,
    },
    /// Delete a network.
    Delete {
        network: String,
    },
    /// Disconnect a container from a network.
    Disconnect {
        container: String,
        network: String,
    },
    Inspect {
        network: String,
    },
    List,
    Prune,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let docker = new_docker()?;
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        Cmd::Connect { container, network } => {
            use docker_api::api::ContainerConnectionOpts;
            if let Err(e) = docker
                .networks()
                .get(&network)
                .connect(&ContainerConnectionOpts::builder(&container).build())
                .await
            {
                eprintln!("Error: {}", e)
            }
        }
        Cmd::Create { network, driver } => {
            use docker_api::api::NetworkCreateOpts;
            match docker
                .networks()
                .create(&NetworkCreateOpts::builder(network).driver(driver).build())
                .await
            {
                Ok(info) => println!("{:#?}", info),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Cmd::Delete { network } => {
            if let Err(e) = docker.networks().get(&network).delete().await {
                eprintln!("Error: {}", e)
            }
        }
        Cmd::Disconnect { container, network } => {
            use docker_api::api::ContainerConnectionOpts;
            if let Err(e) = docker
                .networks()
                .get(network)
                .disconnect(&ContainerConnectionOpts::builder(container).build())
                .await
            {
                eprintln!("Error: {}", e)
            }
        }
        Cmd::Inspect { network } => {
            match docker.networks().get(&network).inspect().await {
                Ok(network_info) => println!("{:#?}", network_info),
                Err(e) => eprintln!("Error: {}", e),
            };
        }
        Cmd::List => match docker.networks().list(&Default::default()).await {
            Ok(networks) => networks.into_iter().for_each(|net| {
                println!(
                    "----------------------\nId: {}\nName: {}\nDriver: {}\nLabels:\n{}",
                    net.id,
                    net.name.unwrap_or_default(),
                    net.driver.unwrap_or_default(),
                    net.labels
                        .iter()
                        .map(|(k, v)| format!("{}={}", k, v))
                        .collect::<Vec<_>>()
                        .join(",")
                )
            }),
            Err(e) => eprintln!("Error: {}", e),
        },
        Cmd::Prune => {
            match docker.networks().prune(&Default::default()).await {
                Ok(info) => println!("{:#?}", info),
                Err(e) => eprintln!("Error: {}", e),
            };
        }
    }

    Ok(())
}
