pub fn process(
    value: ftd::ast::VariableValue,
    kind: ftd::interpreter::Kind,
    doc: &ftd::interpreter::TDoc,
    req_config: &fastn_core::RequestConfig,
) -> ftd::interpreter::Result<ftd::interpreter::Value> {
    use itertools::Itertools;
    #[derive(Debug, serde::Serialize)]
    struct UiApp {
        name: String,
        package: String,
        #[serde(rename = "url")]
        url: String,
        icon: Option<ftd::ImageSrc>,
    }

    let apps = req_config
        .config
        .package
        .apps
        .iter()
        .map(|a| UiApp {
            name: a.name.clone(),
            package: a.package.name.clone(),
            url: a.mount_point.to_string(),
            icon: a.package.icon.clone(),
        })
        .collect_vec();

    let installed_apps = fastn_core::ds::LengthList::from_owned(apps);
    doc.from_json(&installed_apps, &kind, &value)
}
