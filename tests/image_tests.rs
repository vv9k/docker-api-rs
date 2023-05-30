mod common;

use common::{
    create_base_image, get_image_full_id, init_runtime, opts, tempdir_with_dockerfile, StreamExt,
    TryStreamExt, DEFAULT_IMAGE,
};

#[tokio::test]
async fn image_create_inspect_delete() {
    let docker = init_runtime();

    let image = create_base_image(&docker, "test-create-image", None).await;
    assert!(image.inspect().await.is_ok());
    let delete_res = image
        .remove(
            &opts::ImageRemoveOpts::builder()
                .force(true)
                .noprune(true)
                .build(),
        )
        .await;
    println!("{delete_res:#?}");
    assert!(delete_res.is_ok());
    assert!(image.inspect().await.is_err());
}

#[tokio::test]
async fn image_inspect() {
    let docker = init_runtime();
    let images = docker.images();

    let image_name = "test-inspect-image";
    create_base_image(&docker, image_name, None).await;

    let image = images.get(image_name);

    let inspect_result = image.inspect().await;
    assert!(inspect_result.is_ok());
    let inspect_data = inspect_result.unwrap();
    assert!(inspect_data
        .repo_tags
        .as_ref()
        .unwrap()
        .contains(&format!("{image_name}:latest")));
    assert!(image.delete().await.is_ok());
}

#[tokio::test]
async fn image_history() {
    let docker = init_runtime();
    let images = docker.images();

    let image_name = "test-history-image";
    create_base_image(&docker, image_name, None).await;

    let image = images.get(image_name);

    let history_result = image.history().await;
    assert!(history_result.is_ok());
    let history_data = history_result.unwrap();
    println!("{history_data:#?}");
    assert!(history_data
        .iter()
        .any(|item| item.tags.iter().any(|t| t == DEFAULT_IMAGE)));
}

#[tokio::test]
async fn image_tag() {
    let docker = init_runtime();
    let images = docker.images();

    let image_name = "test-tag-image";
    create_base_image(&docker, image_name, None).await;

    let image = images.get(image_name);

    let opts = opts::TagOpts::builder()
        .repo(image_name)
        .tag("1.0.0")
        .build();

    assert!(image.tag(&opts).await.is_ok());

    let new_tag = format!("{image_name}:1.0.0");

    assert!(image
        .inspect()
        .await
        .expect("image inspect data")
        .repo_tags
        .expect("repo tags")
        .contains(&new_tag));

    //cleanup
    let _ = image.delete().await;
}

#[tokio::test]
async fn image_export_import() {
    let docker = init_runtime();
    let images = docker.images();

    let image_name = "test-export-image";
    create_base_image(&docker, image_name, None).await;

    let image = images.get(image_name);

    let export_stream = image.export();
    let export_data = export_stream.try_concat().await.expect("image archive");
    assert!(!export_data.is_empty());

    let _ = image
        .remove(
            &opts::ImageRemoveOpts::builder()
                .force(true)
                .noprune(true)
                .build(),
        )
        .await;

    assert!(image.inspect().await.is_err());

    let mut import_stream = images.import(&export_data[..]);
    while let Some(chunk) = import_stream.next().await {
        assert!(chunk.is_ok());
    }
    assert!(image.inspect().await.is_ok());

    let _ = image.delete().await;
    assert!(image.inspect().await.is_err());
}

#[tokio::test]
async fn image_search() {
    let docker = init_runtime();
    let images = docker.images();

    let search_result = images.search("ubuntu").await;
    println!("{search_result:#?}");
    assert!(search_result.is_ok());
    //let search_data = search_result.unwrap();
    //log::error!("{search_data:#?}");
}

#[tokio::test]
async fn image_list() {
    let docker = init_runtime();
    let images = docker.images();

    let name_a = "test-list-image";
    let name_b = "test-list-image2";

    let tmp = tempdir_with_dockerfile(None);

    let label_key = "test-list";
    let value_a = "value_a";
    let value_b = "value_b";
    let opts_a = opts::ImageBuildOpts::builder(tmp.path())
        .labels([(label_key, value_a)])
        .tag(name_a)
        .build();
    let opts_b = opts::ImageBuildOpts::builder(tmp.path())
        .labels([(label_key, value_b)])
        .tag(name_b)
        .build();

    create_base_image(&docker, name_a, Some(opts_a.clone())).await;
    create_base_image(&docker, name_b, Some(opts_b.clone())).await;
    let image_a = images.get(name_a);
    let image_b = images.get(name_b);
    let full_id_a = get_image_full_id(&docker, name_a).await;
    let full_id_b = get_image_full_id(&docker, name_b).await;

    let filter = opts::ImageFilter::LabelKey(label_key.to_string());
    let list_opts = opts::ImageListOpts::builder()
        .filter([filter])
        .all(true)
        .build();
    let list_result = images.list(&list_opts).await;
    assert!(list_result.is_ok());
    let list_data = list_result.unwrap();
    assert_eq!(list_data.len(), 2);
    assert!(list_data.iter().any(|data| data.id == full_id_a));
    assert!(list_data.iter().any(|data| data.id == full_id_b));

    let filter = opts::ImageFilter::Label(label_key.to_string(), value_a.to_string());
    let list_opts = opts::ImageListOpts::builder()
        .filter([filter])
        .all(true)
        .build();
    let list_result = images.list(&list_opts).await;
    // This sometimes breaks when running all tests at the same time
    assert!(list_result.is_ok());
    let list_data = list_result.unwrap();
    assert_eq!(list_data.len(), 1);
    assert!(list_data.iter().any(|report| report.id == full_id_a));
    assert!(!list_data.iter().any(|report| report.id == full_id_b));

    let filter = opts::ImageFilter::Label(label_key.to_string(), value_b.to_string());
    let list_opts = opts::ImageListOpts::builder()
        .filter([filter])
        .all(true)
        .build();
    let list_result = images.list(&list_opts).await;
    assert!(list_result.is_ok());
    let list_data = list_result.unwrap();
    assert_eq!(list_data.len(), 1);
    assert!(!list_data.iter().any(|report| report.id == full_id_a));
    assert!(list_data.iter().any(|report| report.id == full_id_b));

    let filter = opts::ImageFilter::Reference(name_a.to_string(), None);
    let list_opts = opts::ImageListOpts::builder()
        .filter([filter])
        .all(true)
        .build();
    let list_result = images.list(&list_opts).await;
    assert!(list_result.is_ok());
    let list_data = list_result.unwrap();
    assert_eq!(list_data.len(), 1);
    assert_eq!(full_id_a, list_data[0].id);

    let filter = opts::ImageFilter::Reference(name_a.to_string(), Some("latest".to_string()));
    let list_opts = opts::ImageListOpts::builder()
        .filter([filter])
        .all(true)
        .build();
    let list_result = images.list(&list_opts).await;
    assert!(list_result.is_ok());
    let list_data = list_result.unwrap();
    assert_eq!(list_data.len(), 1);
    assert_eq!(full_id_a, list_data[0].id);

    let _ = image_a.delete().await;
    let _ = image_b.delete().await;
}
