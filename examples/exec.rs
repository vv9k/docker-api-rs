mod common;
use clap::Parser;
use common::new_docker;
use docker_api::{conn::TtyChunk, Exec};

#[derive(Parser)]
pub struct Opts {
    #[command(subcommand)]
    subcmd: Cmd,
}

#[derive(Parser)]
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
            use docker_api::opts::ExecCreateOpts;
            use futures::StreamExt;

            // Create Opts with specified command
            let opts = ExecCreateOpts::builder()
                .command(cmd)
                .attach_stdout(true)
                .attach_stderr(true)
                .build();

            let exec = Exec::create(docker, &container, &opts).await?;

            println!("{:#?}", exec.inspect().await?);

            let mut stream = exec.start(&Default::default()).await?;

            while let Some(Ok(chunk)) = stream.next().await {
                println!("{chunk:?}");
                match chunk {
                    TtyChunk::StdOut(buf) => {
                        println!("STDOUT: {}", String::from_utf8_lossy(&buf));
                    }
                    TtyChunk::StdErr(buf) => {
                        println!("STDERR: {}", String::from_utf8_lossy(&buf));
                    }
                    TtyChunk::StdIn(buf) => {
                        println!("STDIN: {}", String::from_utf8_lossy(&buf));
                    }
                }
            }

            println!("{:#?}", exec.inspect().await?);
        }
        Cmd::Resize {
            exec,
            width,
            height,
        } => {
            use docker_api::opts::ExecResizeOpts;
            let exec = Exec::get(docker, &exec);

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
