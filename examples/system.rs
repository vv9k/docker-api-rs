mod common;
use clap::Parser;
use common::new_docker;

#[derive(Parser)]
struct Opts {
    #[clap(subcommand)]
    subcmd: Cmd,
}

#[derive(Parser)]
enum Cmd {
    Info,
    Ping,
    Version,
    DataUsage,
    Events,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let docker = new_docker()?;
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        Cmd::Info => {
            match docker.info().await {
                Ok(info) => println!("{:#?}", info),
                Err(e) => eprintln!("Error: {}", e),
            };
        }
        Cmd::Ping => {
            match docker.ping().await {
                Ok(ping) => println!("{:#?}", ping),
                Err(e) => eprintln!("Error: {}", e),
            };
        }
        Cmd::Version => {
            match docker.version().await {
                Ok(ver) => println!("{:#?}", ver),
                Err(e) => eprintln!("Error: {}", e),
            };
        }
        Cmd::DataUsage => {
            match docker.data_usage().await {
                Ok(info) => println!("{:#?}", info),
                Err(e) => eprintln!("Error: {}", e),
            };
        }
        Cmd::Events => {
            use futures::StreamExt;
            while let Some(event_result) = docker.events(&Default::default()).next().await {
                match event_result {
                    Ok(event) => println!("{:?}", event),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
        }
    }

    Ok(())
}
