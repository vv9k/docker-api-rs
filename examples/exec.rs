mod common;
use clap::Clap;
use common::new_docker;
use docker_api::Exec;

#[derive(Clap)]
pub struct Opts {
    #[clap(subcommand)]
    subcmd: Cmd,
}

#[derive(Clap)]
enum Cmd {
    /// Run a command in container and inspect it
    Inspect {
        /// The container to run the command in.
        container: String,
        /// Command to run.
        cmd: Vec<String>,
    },
    /// Resize the TTY session used by an exec instance.
    Resize {
        exec: String,
        width: u64,
        height: u64,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let opts: Opts = Opts::parse();
    let docker = new_docker()?;

    match opts.subcmd {
        Cmd::Inspect { container, cmd } => {
            use docker_api::ExecContainerOpts;
            use futures::StreamExt;

            // Create Opts with specified command
            let opts = ExecContainerOpts::builder()
                .cmd(cmd)
                .attach_stdout(true)
                .attach_stderr(true)
                .build();

            let exec = Exec::create(&docker, &container, &opts).await?;

            println!("{:#?}", exec.inspect().await?);

            let mut stream = exec.start();

            stream.next().await;

            println!("{:#?}", exec.inspect().await?);
        }
        Cmd::Resize {
            exec,
            width,
            height,
        } => {
            use docker_api::api::ExecResizeOpts;
            let exec = Exec::get(&docker, &exec);

            // Resize its window with given parameters
            let resize_opts = ExecResizeOpts::builder()
                .width(width)
                .height(height)
                .build();
            exec.resize(&resize_opts).await?;
        }
    }

    Ok(())
}
