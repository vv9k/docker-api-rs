use docker_api::{tty::TtyChunk, Docker, ExecContainerOptions};
use futures::StreamExt;
use std::{env, str};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let docker = Docker::new("tcp://127.0.0.1:80")?;
    let id = env::args()
        .nth(1)
        .expect("You need to specify a container id");

    let options = ExecContainerOptions::builder()
        .cmd(vec![
            "bash",
            "-c",
            "echo -n \"echo VAR=$VAR on stdout\"; echo -n \"echo VAR=$VAR on stderr\" >&2",
        ])
        .env(vec!["VAR=value"])
        .attach_stdout(true)
        .attach_stderr(true)
        .build();

    while let Some(exec_result) = docker.containers().get(&id).exec(&options).next().await {
        match exec_result {
            Ok(chunk) => print_chunk(chunk),
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    Ok(())
}

fn print_chunk(chunk: TtyChunk) {
    match chunk {
        TtyChunk::StdOut(bytes) => {
            println!("Stdout: {}", str::from_utf8(&bytes).unwrap_or_default())
        }
        TtyChunk::StdErr(bytes) => {
            eprintln!("Stdout: {}", str::from_utf8(&bytes).unwrap_or_default())
        }
        TtyChunk::StdIn(_) => unreachable!(),
    }
}
