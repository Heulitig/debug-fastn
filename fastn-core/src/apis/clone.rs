#[derive(serde::Serialize, serde::Deserialize, std::fmt::Debug)]
pub struct CloneResponse {
    pub package_name: String,
    pub files: std::collections::BTreeMap<String, Vec<u8>>,
    pub reserved_crs: Vec<i32>,
}

pub async fn clone(config: &fastn_core::Config) -> fastn_core::Result<fastn_core::http::Response> {
    // TODO: implement authentication
    match clone_worker(config).await {
        Ok(data) => fastn_core::http::api_ok(data),
        Err(err) => fastn_core::http::api_error(err.to_string(), None),
    }
}

async fn clone_worker(config: &fastn_core::Config) -> fastn_core::Result<CloneResponse> {
    use itertools::Itertools;

    let all_files = config
        .deprecated_get_all_file_path(&config.package, Default::default())?
        .into_iter()
        .filter(|v| !config.remote_cr().eq(v))
        .collect_vec();
    let root = config.get_root_for_package(&config.package);
    let files = futures::future::join_all(
        all_files
            .into_iter()
            .map(|x| {
                let root = root.clone();
                tokio::spawn(async move {
                    fastn_core::tokio_fs::read(&x).await.map(|v| {
                        (
                            x.strip_prefix(root)
                                .unwrap_or_else(|_| x.as_path())
                                .to_string(),
                            v,
                        )
                    })
                })
            })
            .collect::<Vec<tokio::task::JoinHandle<fastn_core::Result<(String, Vec<u8>)>>>>(),
    )
    .await
    .into_iter()
    .flatten()
    .flatten()
    .collect::<std::collections::BTreeMap<String, Vec<u8>>>();
    Ok(CloneResponse {
        package_name: config.package.name.to_string(),
        files,
        reserved_crs: config.get_reserved_crs(None).await?,
    })
}
