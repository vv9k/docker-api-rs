use docker_api::{exec::ExecResizeOpts, Docker, Exec, ExecContainerOpts};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let docker = Docker::new("tcp://127.0.0.1:80")?;
    let mut args = env::args().skip(1);

    // First argument is container id
    let id = args.next().expect("You need to specify a container id");
    // Second is width
    let width: u64 = args.next().map_or(Ok(0), |s| s.parse::<u64>())?;
    // Third is height
    let height: u64 = args.next().map_or(Ok(0), |s| s.parse::<u64>())?;

    // Create an exec instance
    let exec_opts = ExecContainerOpts::builder()
        .cmd(vec!["echo", "123"])
        .attach_stdout(true)
        .attach_stderr(true)
        .build();
    let exec = Exec::create(&docker, &id, &exec_opts).await?;

    // Resize its window with given parameters
    let resize_opts = ExecResizeOpts::builder()
        .width(width)
        .height(height)
        .build();
    exec.resize(&resize_opts).await?;

    Ok(())
}
