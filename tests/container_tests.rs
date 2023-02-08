mod common;

use common::{
    cleanup_container,
    conn::TtyChunk,
    create_base_container, get_container_full_id, init_runtime,
    opts::{ContainerCreateOpts, ExecCreateOpts},
    StreamExt, TryStreamExt, DEFAULT_CMD, DEFAULT_CMD_ARRAY, DEFAULT_IMAGE,
};

#[tokio::test]
async fn container_create_inspect_remove() {
    let docker = init_runtime();
    let container_name = "test-create-exist-container";

    let container = create_base_container(&docker, container_name, None).await;
    let inspect_result = container.inspect().await;
    assert!(inspect_result.is_ok());

    let remove_result = container.delete().await;
    assert!(remove_result.is_ok());

    let inspect_result = container.inspect().await;
    assert!(inspect_result.is_err());
}

#[tokio::test]
async fn container_inspect() {
    let docker = init_runtime();

    let container_name = "test-inspect-container";
    let container = create_base_container(&docker, container_name, None).await;
    let inspect_result = container.inspect().await;
    assert!(inspect_result.is_ok());
    let inspect_data = inspect_result.unwrap();
    assert!(inspect_data
        .config
        .as_ref()
        .unwrap()
        .image
        .as_ref()
        .unwrap()
        .contains("ubuntu:latest"));
    assert!(inspect_data.name.as_ref().unwrap().contains(container_name));
    assert_eq!(inspect_data.config.unwrap().cmd.unwrap(), DEFAULT_CMD_ARRAY);

    cleanup_container(&docker, container_name).await;
    // check that the container got correctly removed
    let inspect_result = container.inspect().await;
    assert!(inspect_result.is_err());
}

#[tokio::test]
async fn container_rename() {
    let docker = init_runtime();
    cleanup_container(&docker, "new-container-name").await;

    let container_name = "test-rename-container";
    let new_container_name = "test-rename-container-new";
    let container = create_base_container(&docker, container_name, None).await;

    let inspect_result = docker.containers().get(new_container_name).inspect().await;
    assert!(inspect_result.is_err());

    let rename_result = container.rename(new_container_name).await;
    assert!(rename_result.is_ok());

    let inspect_result = docker.containers().get(new_container_name).inspect().await;
    assert!(inspect_result.is_ok());

    cleanup_container(&docker, container_name).await;
    cleanup_container(&docker, new_container_name).await;
}

#[tokio::test]
async fn container_start() {
    let docker = init_runtime();

    let container_name = "test-start-container";
    let container = create_base_container(&docker, container_name, None).await;

    let start_result = container.start().await;
    assert!(start_result.is_ok());

    let inspect_data = container
        .inspect()
        .await
        .expect("started container inspect data");
    assert!(inspect_data.state.unwrap().running.unwrap());

    cleanup_container(&docker, container_name).await;
}

#[tokio::test]
async fn container_restart() {
    let docker = init_runtime();

    let container_name = "test-restart-container";
    let container = create_base_container(&docker, container_name, None).await;
    let _ = container.start().await;

    let timestamp = container
        .inspect()
        .await
        .expect("restarted container inspect data before")
        .state
        .unwrap()
        .started_at
        .unwrap();

    let restart_result = container.restart(None).await;
    assert!(restart_result.is_ok());

    let timestamp_after = container
        .inspect()
        .await
        .expect("restarted container inspect data")
        .state
        .unwrap()
        .started_at
        .unwrap();
    assert_ne!(timestamp, timestamp_after);

    cleanup_container(&docker, container_name).await;
}

#[tokio::test]
async fn container_pause_unpause() {
    let docker = init_runtime();

    let container_name = "test-pause-container";
    let container = create_base_container(&docker, container_name, None).await;

    let _ = container.start().await;

    let pause_result = container.pause().await;
    assert!(pause_result.is_ok());

    let inspect_data = container
        .inspect()
        .await
        .expect("paused container inspect data");
    assert!(inspect_data.state.unwrap().paused.unwrap());

    let unpause_result = container.unpause().await;
    assert!(unpause_result.is_ok());

    let inspect_data = container
        .inspect()
        .await
        .expect("paused container inspect data");
    assert!(!inspect_data.state.unwrap().paused.unwrap());

    cleanup_container(&docker, container_name).await;
}

#[tokio::test]
async fn container_kill() {
    let docker = init_runtime();

    let container_name = "test-kill-container";
    let container = create_base_container(&docker, container_name, None).await;

    let _ = container.start().await;

    let inspect_data = container
        .inspect()
        .await
        .expect("killed container inspect data");
    let state = inspect_data.state.unwrap();
    assert!(state.running.unwrap());

    let kill_result = container.kill(None).await;
    assert!(kill_result.is_ok());

    let inspect_data = container
        .inspect()
        .await
        .expect("killed container inspect data");
    let state = inspect_data.state.unwrap();
    assert!(!state.running.unwrap());

    cleanup_container(&docker, container_name).await;
}

#[tokio::test]
async fn container_stop() {
    let docker = init_runtime();

    let container_name = "test-stop-container";
    let container = create_base_container(&docker, container_name, None).await;

    let _ = container.start().await;

    let inspect_data = container
        .inspect()
        .await
        .expect("stopped container inspect data");
    assert!(inspect_data.state.unwrap().running.unwrap());

    let stop_result = container.stop(None).await;
    assert!(stop_result.is_ok());

    let inspect_data = container
        .inspect()
        .await
        .expect("stopped container inspect data");
    assert!(!inspect_data.state.unwrap().running.unwrap());

    cleanup_container(&docker, container_name).await;
}

#[tokio::test]
async fn container_commit() {
    use docker_api::opts::ContainerCommitOpts;
    let docker = init_runtime();

    let container_name = "test-commit-container";
    let container = create_base_container(&docker, container_name, None).await;

    let author = "docker api rs";
    let commit_result = container
        .commit(
            &ContainerCommitOpts::builder()
                .author(author)
                .repo("test-commit-image")
                .tag("test")
                .build(),
        )
        .await;
    assert!(commit_result.is_ok());
    let image_name = "test-commit-image:test";

    let inspect_data = docker
        .images()
        .get(image_name)
        .inspect()
        .await
        .expect("commited image inspect data");
    assert_eq!(inspect_data.author.unwrap(), author);

    cleanup_container(&docker, container_name).await;
}

#[tokio::test]
async fn container_exec() {
    let docker = init_runtime();

    let container_name = "test-exec-container";
    let container = create_base_container(&docker, container_name, None).await;

    let _ = container.start().await;

    let mut exec_stream = container.exec(
        &ExecCreateOpts::builder()
            .attach_stderr(true)
            .attach_stdout(true)
            .command([
                "bash",
                "-c",
                "mkdir /tmp/test123 && echo 1234 >> /tmp/test123/testfile",
            ])
            .build(),
    );
    while exec_stream.next().await.is_some() {}

    let mut exec_stream = container.exec(
        &ExecCreateOpts::builder()
            .attach_stderr(true)
            .attach_stdout(true)
            .command(["cat", "test123/testfile"])
            .working_dir("/tmp")
            .build(),
    );
    let chunk = exec_stream.next().await;
    assert!(chunk.is_some());
    match chunk.unwrap() {
        Ok(TtyChunk::StdOut(chunk)) => {
            let testfile_content = String::from_utf8_lossy(&chunk);
            assert_eq!(testfile_content, "1234\n");
        }
        Ok(chunk) => {
            let fd = match chunk {
                TtyChunk::StdIn(_) => "stdin",
                TtyChunk::StdOut(_) => "stdOut",
                TtyChunk::StdErr(_) => "stderr",
            };
            let chunk = String::from_utf8_lossy(&chunk);
            eprintln!("invalid chunk, fd: {fd}, content: `{chunk:?}`");
            std::process::exit(1);
        }
        chunk => {
            eprintln!("invalid chunk {chunk:?}");
            std::process::exit(1);
        }
    }

    cleanup_container(&docker, container_name).await;
}

#[tokio::test]
async fn container_copy_from() {
    let docker = init_runtime();

    let container_name = "test-copy-from-container";
    let container = create_base_container(&docker, container_name, None).await;

    let _ = container.start().await;

    let mut exec_stream = container.exec(
        &ExecCreateOpts::builder()
            .attach_stderr(true)
            .attach_stdout(true)
            .command([
                "bash",
                "-c",
                "mkdir /tmp/test123 && echo 1234 >> /tmp/test123/test-copy-from",
            ])
            .build(),
    );
    while exec_stream.next().await.is_some() {}

    let tar_stream = container.copy_from("/tmp/test123");
    let bytes = tar_stream.try_concat().await.expect("joined tarball bytes");
    let mut archive = tar::Archive::new(&bytes[..]);
    let tmp = tempfile::TempDir::new().expect("temporary dir");
    archive.unpack(tmp.path()).unwrap();

    let local_path = tmp.path().join("test123").join("test-copy-from");
    assert!(local_path.exists());
    assert_eq!(std::fs::read_to_string(&local_path).unwrap(), "1234\n");

    cleanup_container(&docker, container_name).await;
}

#[tokio::test]
async fn container_copy_file_into() {
    let docker = init_runtime();

    let container_name = "test-copy-file-into-container";
    let container = create_base_container(&docker, container_name, None).await;

    let _ = container.start().await;

    let data = b"12345";
    let copy_result = container.copy_file_into("/tmp/test-file", data).await;
    assert!(copy_result.is_ok());

    let mut exec_stream = container.exec(
        &ExecCreateOpts::builder()
            .attach_stderr(true)
            .attach_stdout(true)
            .command(["cat", "/tmp/test-file"])
            .build(),
    );
    let chunk = exec_stream.next().await;
    assert!(chunk.is_some());
    match chunk.unwrap() {
        Ok(TtyChunk::StdOut(chunk)) => {
            assert_eq!(chunk, data);
        }
        chunk => {
            eprintln!("invalid chunk {chunk:?}");
            std::process::exit(1);
        }
    }

    cleanup_container(&docker, container_name).await;
}

#[tokio::test]
async fn container_changes() {
    let docker = init_runtime();

    let container_name = "test-changes-container";
    let container = create_base_container(&docker, container_name, None).await;

    let _ = container.start().await;

    let mut exec_stream = container.exec(
        &ExecCreateOpts::builder()
            .attach_stderr(true)
            .attach_stdout(true)
            .command([
                "bash",
                "-c",
                "rm /etc/xattr.conf && echo 12345 >> /tmp/test-changes",
            ])
            .build(),
    );
    while exec_stream.next().await.is_some() {}

    use docker_api::models::ContainerChangeResponseItem;

    let changes = container
        .changes()
        .await
        .expect("container changes")
        .unwrap_or_default();
    assert!(changes.contains(&ContainerChangeResponseItem {
        kind: 0,
        path: "/tmp".into()
    }));
    assert!(changes.contains(&ContainerChangeResponseItem {
        kind: 1,
        path: "/tmp/test-changes".into()
    }));
    assert!(changes.contains(&ContainerChangeResponseItem {
        kind: 2,
        path: "/etc/xattr.conf".into()
    }));

    cleanup_container(&docker, container_name).await;
}

#[tokio::test]
async fn container_logs() {
    let docker = init_runtime();

    let container_name = "test-logs-container";
    let container = create_base_container(
        &docker,
        container_name,
        Some(
            ContainerCreateOpts::builder()
                .image(DEFAULT_IMAGE)
                .name(container_name)
                .command(["bash", "-c", "echo 123456 && sleep inf"])
                .build(),
        ),
    )
    .await;

    let _ = container.start().await;

    use docker_api::opts::LogsOpts;

    let mut logs_stream = container.logs(&LogsOpts::builder().stdout(true).stderr(true).build());
    let chunk = logs_stream.next().await;
    match chunk {
        Some(Ok(TtyChunk::StdOut(chunk))) => {
            let logs = String::from_utf8_lossy(&chunk);
            assert_eq!(logs, "123456\n");
        }
        chunk => {
            eprintln!("invalid chunk {chunk:?}");
            std::process::exit(1);
        }
    }

    cleanup_container(&docker, container_name).await;
}

#[tokio::test]
async fn container_stats() {
    let docker = init_runtime();

    let container_name = "test-stats-container";
    let container = create_base_container(&docker, container_name, None).await;

    let _ = container.start().await;

    const WANT: i32 = 5;
    let mut got = 0;
    let mut stats_stream = container.stats();

    for _ in 0..WANT {
        if let Some(chunk) = stats_stream.next().await {
            assert!(chunk.is_ok());
            got += 1;
        }
    }

    assert_eq!(got, WANT);

    cleanup_container(&docker, container_name).await;
}

#[tokio::test]
async fn container_top() {
    let docker = init_runtime();

    let container_name = "test-top-container";
    let container = create_base_container(&docker, container_name, None).await;

    let _ = container.start().await;

    let top_result = container.top(None).await;
    assert!(top_result.is_ok());
    assert!(top_result.unwrap().processes.unwrap_or_default()[0].contains(&DEFAULT_CMD.to_string()));

    cleanup_container(&docker, container_name).await;
}

#[tokio::test]
async fn containers_list() {
    use docker_api::opts::{ContainerFilter, ContainerListOpts};
    let docker = init_runtime();

    let container_name = "test-list-container";
    let opts = ContainerCreateOpts::builder()
        .command(DEFAULT_CMD_ARRAY)
        .image(DEFAULT_IMAGE)
        .labels([("test-docker-list", "value")])
        .name(container_name)
        .build();

    let second_name = "test-list-second-container";
    let second_opts = ContainerCreateOpts::builder()
        .command(DEFAULT_CMD_ARRAY)
        .image(DEFAULT_IMAGE)
        .labels([("test-docker-list", "value2")])
        .name(second_name)
        .build();
    create_base_container(&docker, container_name, Some(opts)).await;
    create_base_container(&docker, second_name, Some(second_opts)).await;

    let filter = ContainerFilter::Name(container_name.to_string());
    let opts = ContainerListOpts::builder()
        .all(true)
        .filter([filter])
        .build();
    let list_result = docker.containers().list(&opts).await;
    assert!(list_result.is_ok());
    let list_data = list_result.unwrap();
    assert_eq!(list_data.len(), 1);
    assert!(list_data[0]
        .names
        .as_ref()
        .unwrap()
        .contains(&format!("/{container_name}")));

    let filter = ContainerFilter::LabelKey("test-docker-list".into());
    let opts = ContainerListOpts::builder()
        .all(true)
        .filter([filter])
        .build();
    let list_result = docker.containers().list(&opts).await;
    assert!(list_result.is_ok());
    let list_data = list_result.unwrap();
    assert_eq!(list_data.len(), 2);

    let filter = ContainerFilter::Label("test-docker-list".into(), "value".into());
    let opts = ContainerListOpts::builder()
        .all(true)
        .filter([filter])
        .build();
    let list_result = docker.containers().list(&opts).await;
    assert!(list_result.is_ok());
    let list_data = list_result.unwrap();
    assert_eq!(list_data.len(), 1);
    assert!(list_data[0]
        .names
        .as_ref()
        .unwrap()
        .contains(&format!("/{container_name}")));

    let filter = ContainerFilter::Label("test-docker-list".into(), "value2".into());
    let opts = ContainerListOpts::builder()
        .all(true)
        .filter([filter])
        .build();
    let list_result = docker.containers().list(&opts).await;
    assert!(list_result.is_ok());
    let list_data = list_result.unwrap();
    assert_eq!(list_data.len(), 1);
    assert!(list_data[0]
        .names
        .as_ref()
        .unwrap()
        .contains(&format!("/{second_name}")));

    cleanup_container(&docker, container_name).await;
    cleanup_container(&docker, second_name).await;
}

#[tokio::test]
async fn containers_prune() {
    use docker_api::opts::{ContainerPruneFilter, ContainerPruneOpts};
    let docker = init_runtime();

    let container_name = "test-prune-container";
    let opts = ContainerCreateOpts::builder()
        .command(DEFAULT_CMD_ARRAY)
        .image(DEFAULT_IMAGE)
        .labels([("test-docker-prune", "value")])
        .name(container_name)
        .build();

    let second_name = "test-prune-second-container";
    let second_opts = ContainerCreateOpts::builder()
        .command(DEFAULT_CMD_ARRAY)
        .image(DEFAULT_IMAGE)
        .labels([("test-docker-prune", "value2")])
        .name(second_name)
        .build();

    create_base_container(&docker, container_name, Some(opts.clone())).await;
    create_base_container(&docker, second_name, Some(second_opts.clone())).await;
    let full_id = get_container_full_id(&docker, container_name).await;
    let second_full_id = get_container_full_id(&docker, second_name).await;

    let filter = ContainerPruneFilter::LabelKey("test-docker-prune".into());
    let prune_opts = ContainerPruneOpts::builder().filter([filter]).build();
    let prune_result = docker.containers().prune(&prune_opts).await;
    assert!(prune_result.is_ok());
    let prune_data = prune_result.unwrap().containers_deleted.unwrap_or_default();
    assert_eq!(prune_data.len(), 2);
    assert!(prune_data.iter().any(|id| id == &full_id));
    assert!(prune_data.iter().any(|id| id == &second_full_id));

    create_base_container(&docker, container_name, Some(opts.clone())).await;
    create_base_container(&docker, second_name, Some(second_opts.clone())).await;
    let full_id = get_container_full_id(&docker, container_name).await;
    let second_full_id = get_container_full_id(&docker, second_name).await;

    let filter = ContainerPruneFilter::Label("test-docker-prune".into(), "value".into());
    let prune_opts = ContainerPruneOpts::builder().filter([filter]).build();
    let prune_result = docker.containers().prune(&prune_opts).await;
    assert!(prune_result.is_ok());
    let prune_data = prune_result.unwrap().containers_deleted.unwrap_or_default();
    assert_eq!(prune_data.len(), 1);
    assert!(prune_data.iter().any(|id| id == &full_id));

    let filter = ContainerPruneFilter::Label("test-docker-prune".into(), "value2".into());
    let prune_opts = ContainerPruneOpts::builder().filter([filter]).build();
    let prune_result = docker.containers().prune(&prune_opts).await;
    assert!(prune_result.is_ok());
    let prune_data = prune_result.unwrap().containers_deleted.unwrap_or_default();
    assert_eq!(prune_data.len(), 1);
    assert!(prune_data.iter().any(|id| id == &second_full_id));
}

#[tokio::test]
async fn container_attach() {
    let docker = init_runtime();

    let container_name = "test-attach-tty-container";
    let container = create_base_container(
        &docker,
        container_name,
        Some(
            ContainerCreateOpts::builder()
                .attach_stderr(true)
                .attach_stdout(true)
                .attach_stdin(true)
                .tty(true)
                .image(DEFAULT_IMAGE)
                .name(container_name)
                .command(["bash", "-c", "while true; do echo 123456 && sleep 2; done"])
                .build(),
        ),
    )
    .await;

    let _ = container.start().await;

    let mut multiplexer = container.attach().await.unwrap();
    while let Some(chunk) = multiplexer.next().await {
        match chunk {
            Ok(TtyChunk::StdOut(chunk)) => {
                let logs = String::from_utf8_lossy(&chunk);
                assert_eq!(logs, "123456\r\n");
                break;
            }
            chunk => {
                eprintln!("invalid chunk {chunk:?}");
                std::process::exit(1);
            }
        }
    }

    cleanup_container(&docker, container_name).await;

    let container_name = "test-attach-non-tty-container";
    let container = create_base_container(
        &docker,
        container_name,
        Some(
            ContainerCreateOpts::builder()
                .attach_stderr(true)
                .attach_stdout(true)
                .attach_stdin(true)
                .image(DEFAULT_IMAGE)
                .name(container_name)
                .command(["bash", "-c", "while true; do echo 123456 && sleep 2; done"])
                .build(),
        ),
    )
    .await;

    let _ = container.start().await;

    let mut multiplexer = container.attach().await.unwrap();
    while let Some(chunk) = multiplexer.next().await {
        match chunk {
            Ok(TtyChunk::StdOut(chunk)) => {
                let logs = String::from_utf8_lossy(&chunk);
                assert_eq!(logs, "123456\n");
                break;
            }
            chunk => {
                eprintln!("invalid chunk {chunk:?}");
                std::process::exit(1);
            }
        }
    }

    cleanup_container(&docker, container_name).await;
}
