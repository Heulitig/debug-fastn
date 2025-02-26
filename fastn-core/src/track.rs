#[derive(serde::Serialize, serde::Deserialize, Debug, Default)]
pub struct TrackingInfo {
    pub filename: String,
    pub version: i32,
    #[serde(rename = "self-version")]
    pub self_version: Option<i32>,
}

impl TrackingInfo {
    pub(crate) fn new(filename: &str, version: i32, self_version: Option<i32>) -> TrackingInfo {
        TrackingInfo {
            filename: filename.to_string(),
            version,
            self_version,
        }
    }
}

#[allow(dead_code)]
pub(crate) async fn get_tracking_info(
    config: &fastn_core::Config,
    path: &camino::Utf8PathBuf,
) -> fastn_core::Result<Vec<fastn_core::track::TrackingInfo>> {
    let track_path = config.track_path(path);
    get_tracking_info_(&track_path).await
}

pub(crate) async fn get_tracking_info_(
    track_path: &camino::Utf8PathBuf,
) -> fastn_core::Result<Vec<fastn_core::track::TrackingInfo>> {
    if !track_path.exists() {
        return fastn_core::usage_error(format!("No tracking found for {}", track_path));
    }

    let doc = fastn_core::tokio_fs::read_to_string(&track_path).await?;
    resolve_tracking_info(&doc, track_path).await
}

pub(crate) async fn resolve_tracking_info(
    content: &str,
    path: &camino::Utf8PathBuf,
) -> fastn_core::Result<Vec<fastn_core::track::TrackingInfo>> {
    if content.trim().is_empty() {
        return Err(fastn_core::Error::UsageError {
            message: format!("Content is empty in track file for {}", path),
        });
    }
    let lib = fastn_core::FastnLibrary::default();
    let b = match fastn_core::doc::parse_ftd(path.to_string().as_str(), content, &lib) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("failed to parse track for {}: {:?}", path, &e);
            todo!();
        }
    };

    Ok(b.get("fastn#tracks")?)
}

pub(crate) async fn create_tracking_info(
    config: &fastn_core::Config,
    tracking_infos: &[fastn_core::track::TrackingInfo],
    path: &camino::Utf8PathBuf,
) -> fastn_core::Result<()> {
    let tracking_info_content = generate_tracking_info_content(tracking_infos);
    fastn_core::utils::update(&config.track_path(path), tracking_info_content.as_bytes()).await?;
    Ok(())
}

pub(crate) fn generate_tracking_info_content(
    tracking_infos: &[fastn_core::track::TrackingInfo],
) -> String {
    let mut tracking_info_vector = vec!["-- import: fastn".to_string()];

    for tracking_info in tracking_infos {
        let mut content = format!(
            "-- fastn.tracks: {}\nversion: {}",
            tracking_info.filename, tracking_info.version
        );
        if let Some(ref self_version) = tracking_info.self_version {
            content = format!("{}\nself-version: {}", content, self_version);
        }
        tracking_info_vector.push(content);
    }
    let content = tracking_info_vector.join("\n\n");
    format!("{content}\n")
}
