// Discussion: https://github.com/ftd-lang/fastn/discussions/475
// Docs: https://fastn.io/sitemap/clear-cache/

#[derive(Default, Debug)]
pub struct QueryParams {
    file: Vec<String>,
    package: Vec<String>,
    all_dependencies: bool,
}

fn query(uri: &str) -> fastn_core::Result<QueryParams> {
    use itertools::Itertools;
    let query = fastn_core::utils::query(uri)?;
    Ok(QueryParams {
        file: query
            .iter()
            .filter_map(|(key, value)| {
                if key.eq("file") {
                    Some(value.to_string())
                } else {
                    None
                }
            })
            .collect_vec(),
        package: query
            .iter()
            .filter_map(|(key, value)| {
                if key.eq("package") {
                    Some(value.to_string())
                } else {
                    None
                }
            })
            .collect_vec(),
        all_dependencies: query
            .iter()
            .any(|(key, value)| key.eq("all-dependencies") && (value.eq("true") || value.eq("t"))),
    })
}

pub async fn clear(
    config: &fastn_core::Config,
    req: &fastn_core::http::Request,
) -> fastn_core::http::Response {
    let query = match query(req.uri()) {
        Ok(q) => q,
        Err(err) => {
            return fastn_core::server_error!(
                "fastn-Error: /-/clear-cache/, uri: {:?}, error: {:?}",
                req.uri(),
                err
            );
        }
    };

    if let Err(err) = clear_(config, &query, req).await {
        return fastn_core::server_error!(
            "fastn-Error: /-/clear-cache/, query: {:?}, error: {:?}",
            query,
            err
        );
    }
    dbg!("cache-cleared");
    fastn_core::http::ok("Done".into())
}

pub async fn clear_(
    config: &fastn_core::Config,
    query: &QueryParams,
    _req: &fastn_core::http::Request,
) -> fastn_core::Result<()> {
    if config.package.download_base_url.is_none() {
        return Err(fastn_core::Error::APIResponseError(
            "cannot remove anything, package does not have `download_base_url`".to_string(),
        ));
    }

    // file: file path can be from main package or .packages folder
    for file in query.file.iter() {
        let main_file_path = config.root.join(file.as_str());
        let package_file_path = config.packages_root.join(file.as_str());
        if main_file_path.exists() {
            let path = tokio::fs::canonicalize(main_file_path).await?;
            if path.starts_with(&config.root) {
                fastn_core::utils::remove(path.as_path()).await?;
            }
        } else if package_file_path.exists() {
            let path = tokio::fs::canonicalize(package_file_path).await?;
            if path.starts_with(&config.root) {
                fastn_core::utils::remove(path.as_path()).await?;
            }
        } else {
            println!("Not able to remove file from cache: {}", file);
        }
    }

    // package value: main, <package-name>
    for package in query.package.iter() {
        if package.eq("main") {
            // TODO: List directories and files other than main
            fastn_core::utils::remove_except(&config.root, &[".packages", ".build"]).await?;
        } else {
            let path = tokio::fs::canonicalize(config.packages_root.join(package)).await?;
            if path.starts_with(&config.packages_root) {
                fastn_core::utils::remove(path.as_path()).await?;
            }
        }
    }

    if query.all_dependencies {
        tokio::fs::remove_dir_all(&config.packages_root).await?;
    }

    // Download FASTN.ftd again after removing all the content
    if !config.root.join("FASTN.ftd").exists() {
        fastn_core::commands::serve::download_init_package(&config.package.download_base_url)
            .await?;
    }

    Ok(())
}
