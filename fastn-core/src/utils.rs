pub trait ValueOf {
    fn value_of_(&self, name: &str) -> Option<&str>;
    fn values_of_(&self, name: &str) -> Vec<String>;
}

impl ValueOf for clap::ArgMatches {
    fn value_of_(&self, name: &str) -> Option<&str> {
        self.get_one::<String>(name).map(|v| v.as_str())
    }
    fn values_of_(&self, name: &str) -> Vec<String> {
        self.get_many(name)
            .map(|v| v.cloned().collect::<Vec<String>>())
            .unwrap_or_default()
    }
}

// https://stackoverflow.com/questions/71985357/whats-the-best-way-to-write-a-custom-format-macro
#[macro_export]
macro_rules! warning {
    ($($t:tt)*) => {{
        use colored::Colorize;
        let msg = format!($($t)*);
        if fastn_observer::is_traced() {
            tracing::warn!(msg);
        } else {
            eprintln!("WARN: {}", msg.yellow());
        }
        msg
    }};
}

fn id_to_cache_key(id: &str) -> String {
    // TODO: use MAIN_SEPARATOR here
    id.replace(['/', '\\'], "_")
}

pub fn get_ftd_hash(path: &str) -> fastn_core::Result<String> {
    let path = fastn_core::utils::replace_last_n(path, 1, "/", "");
    Ok(fastn_core::utils::generate_hash(
        std::fs::read(format!("{path}.ftd"))
            .or_else(|_| std::fs::read(format!("{path}/index.ftd")))?,
    ))
}

pub fn get_cache_file(id: &str) -> Option<std::path::PathBuf> {
    let cache_dir = dirs::cache_dir()?;
    let base_path = cache_dir.join("fastn.com");

    if !base_path.exists() {
        if let Err(err) = std::fs::create_dir_all(&base_path) {
            eprintln!("Failed to create cache directory: {}", err);
            return None;
        }
    }

    Some(
        base_path
            .join(id_to_cache_key(
                &std::env::current_dir()
                    .expect("cant read current dir")
                    .to_string_lossy(),
            ))
            .join(id_to_cache_key(id)),
    )
}

pub fn get_cached<T>(id: &str) -> Option<T>
where
    T: serde::de::DeserializeOwned,
{
    let cache_file = get_cache_file(id)?;
    serde_json::from_str(
        &std::fs::read_to_string(cache_file)
            .map_err(|e| {
                tracing::debug!("file read error: {}", e.to_string());
                e
            })
            .ok()?,
    )
    .map_err(|e| {
        tracing::debug!("not valid json: {}", e.to_string());
        e
    })
    .ok()
}

pub fn cache_it<T>(id: &str, d: T) -> ftd::interpreter::Result<T>
where
    T: serde::ser::Serialize,
{
    let cache_file = get_cache_file(id)
        .ok_or_else(|| ftd::interpreter::Error::OtherError("cache dir not found".to_string()))?;
    std::fs::create_dir_all(cache_file.parent().unwrap()).map_err(|e| {
        ftd::interpreter::Error::OtherError(format!("failed to create cache dir: {}", e))
    })?;
    std::fs::write(cache_file, serde_json::to_string(&d)?).map_err(|e| {
        ftd::interpreter::Error::OtherError(format!("failed to write cache file: {}", e))
    })?;
    Ok(d)
}

pub fn redirect_page_html(url: &str) -> String {
    include_str!("../redirect.html").replace("__REDIRECT_URL__", url)
}

pub fn print_end(msg: &str, start: std::time::Instant) {
    use colored::Colorize;

    if fastn_core::utils::is_test() {
        println!("done in <omitted>");
    } else {
        println!(
            // TODO: instead of lots of spaces put proper erase current terminal line thing
            "\r{:?} {} in {:?}.                          ",
            std::time::Instant::now(),
            msg.green(),
            start.elapsed()
        );
    }
}

/// replace_last_n("a.b.c.d.e.f", 2, ".", "/") => "a.b.c.d/e/f"
pub fn replace_last_n(s: &str, n: usize, pattern: &str, replacement: &str) -> String {
    use itertools::Itertools;

    s.rsplitn(n + 1, pattern)
        .collect_vec()
        .into_iter()
        .rev()
        .join(replacement)
}

#[cfg(test)]
mod test {
    #[test]
    fn replace_last_n() {
        assert_eq!(
            super::replace_last_n("a.b.c.d.e.f", 2, ".", "/"),
            "a.b.c.d/e/f"
        );
        assert_eq!(
            super::replace_last_n("a.b.c.d.e.", 2, ".", "/"),
            "a.b.c.d/e/"
        );
        assert_eq!(super::replace_last_n("d-e.f", 2, ".", "/"), "d-e/f");
        assert_eq!(
            super::replace_last_n("a.ftd/b.ftd", 1, ".ftd", "/index.html"),
            "a.ftd/b/index.html"
        );
        assert_eq!(
            super::replace_last_n("index.ftd/b/index.ftd", 1, "index.ftd", "index.html"),
            "index.ftd/b/index.html"
        );
    }
}

pub fn print_error(msg: &str, start: std::time::Instant) {
    use colored::Colorize;

    if fastn_core::utils::is_test() {
        println!("done in <omitted>");
    } else {
        eprintln!(
            "\r{:?} {} in {:?}.                          ",
            std::time::Instant::now(),
            msg.red(),
            start.elapsed(),
        );
    }
}

pub fn value_to_colored_string(value: &serde_json::Value, indent_level: u32) -> String {
    use colored::Colorize;

    match value {
        serde_json::Value::Null => "null".bright_black().to_string(),
        serde_json::Value::Bool(v) => v.to_string().bright_green().to_string(),
        serde_json::Value::Number(v) => v.to_string().bright_blue().to_string(),
        serde_json::Value::String(v) => format!(
            "\"{}\"",
            v.replace('\\', "\\\\")
                .replace('\n', "\\n")
                .replace('\"', "\\\"")
        )
        .bright_yellow()
        .to_string(),
        serde_json::Value::Array(v) => {
            let mut s = String::new();
            for (idx, value) in v.iter().enumerate() {
                s.push_str(&format!(
                    "{comma}\n{indent}{value}",
                    indent = "  ".repeat(indent_level as usize),
                    value = value_to_colored_string(value, indent_level + 1),
                    comma = if idx.eq(&0) { "" } else { "," }
                ));
            }
            format!("[{}\n{}]", s, "  ".repeat((indent_level - 1) as usize))
        }
        serde_json::Value::Object(v) => {
            let mut s = String::new();
            for (idx, (key, value)) in v.iter().enumerate() {
                s.push_str(&format!(
                    "{comma}\n{indent}\"{i}\": {value}",
                    indent = "  ".repeat(indent_level as usize),
                    i = key.bright_cyan(),
                    value = value_to_colored_string(value, indent_level + 1),
                    comma = if idx.eq(&0) { "" } else { "," }
                ));
            }
            format!("{{{}\n{}}}", s, "  ".repeat((indent_level - 1) as usize))
        }
    }
}

pub fn value_to_colored_string_without_null(
    value: &serde_json::Value,
    indent_level: u32,
) -> String {
    use colored::Colorize;

    match value {
        serde_json::Value::Null => "".to_string(),
        serde_json::Value::Bool(v) => v.to_string().bright_green().to_string(),
        serde_json::Value::Number(v) => v.to_string().bright_blue().to_string(),
        serde_json::Value::String(v) => format!(
            "\"{}\"",
            v.replace('\\', "\\\\")
                .replace('\n', "\\n")
                .replace('\"', "\\\"")
        )
        .bright_yellow()
        .to_string(),
        serde_json::Value::Array(v) if v.is_empty() => "".to_string(),
        serde_json::Value::Array(v) => {
            let mut s = String::new();
            let mut is_first = true;
            for (_, value) in v.iter().enumerate() {
                let value_string = value_to_colored_string_without_null(value, indent_level + 1);
                if !value_string.is_empty() {
                    s.push_str(&format!(
                        "{comma}\n{indent}{value}",
                        indent = "  ".repeat(indent_level as usize),
                        value = value_string,
                        comma = if is_first { "" } else { "," }
                    ));
                    is_first = false;
                }
            }
            if s.is_empty() {
                "".to_string()
            } else {
                format!("[{}\n{}]", s, "  ".repeat((indent_level - 1) as usize))
            }
        }
        serde_json::Value::Object(v) => {
            let mut s = String::new();
            let mut is_first = true;
            for (key, value) in v {
                let value_string = value_to_colored_string_without_null(value, indent_level + 1);
                if !value_string.is_empty() {
                    s.push_str(&format!(
                        "{comma}\n{indent}\"{i}\": {value}",
                        indent = "  ".repeat(indent_level as usize),
                        i = key.bright_cyan(),
                        value = value_string,
                        comma = if is_first { "" } else { "," }
                    ));
                    is_first = false;
                }
            }
            format!("{{{}\n{}}}", s, "  ".repeat((indent_level - 1) as usize))
        }
    }
}

pub fn time(msg: &str) -> Timer {
    Timer {
        start: std::time::Instant::now(),
        msg,
    }
}

pub struct Timer<'a> {
    start: std::time::Instant,
    msg: &'a str,
}

impl<'a> Timer<'a> {
    pub fn it<T>(&self, a: T) -> T {
        use colored::Colorize;

        if !fastn_core::utils::is_test() {
            let duration = format!("{:?}", self.start.elapsed());
            println!("{} in {}", self.msg.green(), duration.red());
        }

        a
    }
}

pub trait HasElements {
    fn has_elements(&self) -> bool;
}

impl<T> HasElements for Vec<T> {
    fn has_elements(&self) -> bool {
        !self.is_empty()
    }
}

pub(crate) fn timestamp_nanosecond() -> u128 {
    match std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_nanos(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

pub(crate) fn language_to_human(language: &str) -> String {
    realm_lang::Language::from_2_letter_code(language)
        .map(|v| v.human())
        .unwrap_or_else(|_| language.to_string())
}

pub(crate) fn nanos_to_rfc3339(nanos: &u128) -> String {
    nanos.to_string() // TODO
}

pub(crate) fn history_path(id: &str, base_path: &str, timestamp: &u128) -> camino::Utf8PathBuf {
    let id_with_timestamp_extension = snapshot_id(id, timestamp);
    let base_path = camino::Utf8PathBuf::from(base_path);
    base_path.join(".history").join(id_with_timestamp_extension)
}

pub(crate) fn snapshot_id(path: &str, timestamp: &u128) -> String {
    if let Some((id, ext)) = path.rsplit_once('.') {
        format!("{}.{}.{}", id, timestamp, ext)
    } else {
        format!("{}.{}", path, timestamp)
    }
}

pub(crate) fn track_path(id: &str, base_path: &str) -> camino::Utf8PathBuf {
    let base_path = camino::Utf8PathBuf::from(base_path);
    base_path.join(".tracks").join(format!("{}.track", id))
}

pub(crate) async fn get_number_of_documents(
    config: &fastn_core::Config,
) -> fastn_core::Result<String> {
    let mut no_of_docs = fastn_core::snapshot::get_latest_snapshots(&config.root)
        .await?
        .len()
        .to_string();
    if let Ok(original_path) = config.original_path() {
        let no_of_original_docs = fastn_core::snapshot::get_latest_snapshots(&original_path)
            .await?
            .len();
        no_of_docs = format!("{} / {}", no_of_docs, no_of_original_docs);
    }
    Ok(no_of_docs)
}

pub(crate) async fn get_current_document_last_modified_on(
    config: &fastn_core::Config,
    document_id: &str,
) -> Option<String> {
    fastn_core::snapshot::get_latest_snapshots(&config.root)
        .await
        .unwrap_or_default()
        .get(document_id)
        .map(nanos_to_rfc3339)
}

pub(crate) async fn get_last_modified_on(path: &camino::Utf8PathBuf) -> Option<String> {
    fastn_core::snapshot::get_latest_snapshots(path)
        .await
        .unwrap_or_default()
        .values()
        .max()
        .map(nanos_to_rfc3339)
}

/*
// todo get_package_title needs to be implemented
    @amitu need to come up with idea
    This data would be used in fastn.title
pub(crate) fn get_package_title(config: &fastn_core::Config) -> String {
    let fastn = if let Ok(fastn) = std::fs::read_to_string(config.root.join("index.ftd")) {
        fastn
    } else {
        return config.package.name.clone();
    };
    let lib = fastn_core::Library {
        config: config.clone(),
        markdown: None,
        document_id: "index.ftd".to_string(),
        translated_data: Default::default(),
        current_package: std::sync::Arc::new(std::sync::Mutex::new(vec![config.package.clone()])),
    };
    let main_ftd_doc = match ftd::p2::Document::from("index.ftd", fastn.as_str(), &lib) {
        Ok(v) => v,
        Err(_) => {
            return config.package.name.clone();
        }
    };
    match &main_ftd_doc.title() {
        Some(x) => x.rendered.clone(),
        _ => config.package.name.clone(),
    }
}*/

#[async_recursion::async_recursion(?Send)]
pub async fn copy_dir_all(
    src: impl AsRef<camino::Utf8Path> + 'static,
    dst: impl AsRef<camino::Utf8Path> + 'static,
) -> std::io::Result<()> {
    tokio::fs::create_dir_all(dst.as_ref()).await?;
    let mut dir = tokio::fs::read_dir(src.as_ref()).await?;
    while let Some(child) = dir.next_entry().await? {
        if child.metadata().await?.is_dir() {
            copy_dir_all(
                camino::Utf8PathBuf::from_path_buf(child.path())
                    .expect("we only work with utf8 paths"),
                dst.as_ref().join(
                    child
                        .file_name()
                        .into_string()
                        .expect("we only work with utf8 paths"),
                ),
            )
            .await?;
        } else {
            tokio::fs::copy(
                child.path(),
                dst.as_ref().join(
                    child
                        .file_name()
                        .into_string()
                        .expect("we only work with utf8 paths"),
                ),
            )
            .await?;
        }
    }
    Ok(())
}

pub(crate) fn seconds_to_human(s: u64) -> String {
    let days = s / 3600 / 24;
    let hours = s / 3600 - days * 24;
    let months = days / 30;
    if s == 0 {
        "Just now".to_string()
    } else if s == 1 {
        "One second ago".to_string()
    } else if s < 60 {
        format!("{} seconds ago", s)
    } else if s < 3600 {
        format!("{} minutes ago", s / 60)
    } else if s < 3600 * 10 {
        let r = s - hours * 60;
        if r == 0 {
            format!("{} hours ago", hours)
        } else if hours == 1 && r == 1 {
            "An hour and a minute ago".to_string()
        } else if hours == 1 {
            format!("An hour and {} minutes ago", r)
        } else {
            format!("{} hours ago", hours)
        }
    } else if days < 1 {
        format!("{} hours ago", hours)
    } else if days == 1 && hours == 0 {
        "A day ago".to_string()
    } else if days == 1 && hours == 1 {
        "A day an hour ago".to_string()
    } else if days == 1 {
        format!("A day ago and {} hours ago", hours)
    } else if days < 7 && hours == 0 {
        format!("{} days ago", days)
    } else if months == 1 {
        "A month ago".to_string()
    } else if months < 24 {
        format!("{} months ago", months)
    } else {
        format!("{} years ago", months / 12)
    }
}

pub(crate) fn validate_base_url(package: &fastn_core::Package) -> fastn_core::Result<()> {
    if package.download_base_url.is_none() {
        warning!("expected base in fastn.package: {:?}", package.name);
    }

    Ok(())
}

pub fn escape_string(s: &str) -> String {
    let mut result = String::new();
    for c in s.chars() {
        match c {
            '\\' => result.push_str("\\\\"),
            '\"' => result.push_str("\\\""),
            '\n' => result.push_str("\\n"),
            '\r' => result.push_str("\\r"),
            '\t' => result.push_str("\\t"),
            '\0' => result.push_str("\\0"),
            _ => result.push(c),
        }
    }
    result
}

#[allow(dead_code)]
pub fn escape_ftd(file: &str) -> String {
    use itertools::Itertools;

    file.split('\n')
        .map(|v| {
            if v.starts_with("-- ") || v.starts_with("--- ") {
                format!("\\{}", v)
            } else {
                v.to_string()
            }
        })
        .join("\n")
}

pub fn id_to_path(id: &str) -> String {
    id.replace("/index.ftd", "/")
        .replace("index.ftd", "/")
        .replace(".ftd", std::path::MAIN_SEPARATOR.to_string().as_str())
        .replace("/index.md", "/")
        .replace("/README.md", "/")
        .replace("index.md", "/")
        .replace("README.md", "/")
        .replace(".md", std::path::MAIN_SEPARATOR.to_string().as_str())
}

/// returns true if an existing file named "file_name"
/// exists in the root package folder
fn is_file_in_root(root: &str, file_name: &str) -> bool {
    camino::Utf8PathBuf::from(root).join(file_name).exists()
}

/// returns favicon html tag as string
/// (if favicon is passed as header in fastn.package or if any favicon.* file is present in the root package folder)
/// otherwise returns None
fn resolve_favicon(
    root_path: &str,
    package_name: &str,
    favicon: &Option<String>,
) -> Option<String> {
    /// returns html tag for using favicon.
    fn favicon_html(favicon_path: &str, content_type: &str) -> String {
        let favicon_html = format!(
            "\n<link rel=\"shortcut icon\" href=\"{}\" type=\"{}\">",
            favicon_path, content_type
        );
        favicon_html
    }

    /// returns relative favicon path from package and its mime content type
    fn get_favicon_path_and_type(package_name: &str, favicon_path: &str) -> (String, String) {
        // relative favicon path wrt package
        let path = camino::Utf8PathBuf::from(package_name).join(favicon_path);
        // mime content type of the favicon
        let content_type = mime_guess::from_path(path.as_str()).first_or_octet_stream();

        (favicon_path.to_string(), content_type.to_string())
    }

    // favicon image path from fastn.package if provided
    let fav_path = favicon;

    let (full_fav_path, fav_mime_content_type): (String, String) = {
        match fav_path {
            Some(ref path) => {
                // In this case, favicon is provided with fastn.package in FASTN.ftd
                get_favicon_path_and_type(package_name, path)
            }
            None => {
                // If favicon not provided so we will look for favicon in the package directory
                // By default if any file favicon.* is present we will use that file instead
                // In case of favicon.* conflict priority will be: .ico > .svg > .png > .jpg.
                // Default searching directory being the root folder of the package

                // Just check if any favicon exists in the root package directory
                // in the above mentioned priority order
                let found_favicon_id = if is_file_in_root(root_path, "favicon.ico") {
                    "favicon.ico"
                } else if is_file_in_root(root_path, "favicon.svg") {
                    "favicon.svg"
                } else if is_file_in_root(root_path, "favicon.png") {
                    "favicon.png"
                } else if is_file_in_root(root_path, "favicon.jpg") {
                    "favicon.jpg"
                } else {
                    // Not using any favicon
                    return None;
                };

                get_favicon_path_and_type(package_name, found_favicon_id)
            }
        }
    };

    // Will use some favicon
    Some(favicon_html(&full_fav_path, &fav_mime_content_type))
}

pub fn get_external_js_html(external_js: &[String]) -> String {
    let mut result = "".to_string();
    for js in external_js {
        result = format!("{}<script src=\"{}\"></script>", result, js);
    }
    result
}

pub fn get_external_css_html(external_js: &[String]) -> String {
    let mut result = "".to_string();
    for js in external_js {
        result = format!("{}<link rel=\"stylesheet\" href=\"{}.css\">", result, js);
    }
    result
}

pub fn get_inline_js_html(inline_js: &[String]) -> String {
    let mut result = "".to_string();
    for path in inline_js {
        if camino::Utf8Path::new(path).exists() {
            if let Ok(content) = std::fs::read_to_string(path) {
                result = format!("{}<script>{}</script>", result, content);
            }
        }
    }
    result
}

pub fn get_inline_css_html(inline_js: &[String]) -> String {
    let mut result = "".to_string();
    for path in inline_js {
        if camino::Utf8Path::new(path).exists() {
            if let Ok(content) = std::fs::read_to_string(path) {
                result = format!("{}<style>{}</style>", result, content);
            }
        }
    }
    result
}

fn get_extra_js(external_js: &[String], inline_js: &[String], js: &str, rive_data: &str) -> String {
    format!(
        "{}{}{}{}",
        get_external_js_html(external_js),
        get_inline_js_html(inline_js),
        js,
        rive_data
    )
}

fn get_extra_css(external_css: &[String], inline_css: &[String], css: &str) -> String {
    format!(
        "{}{}{}",
        get_external_css_html(external_css),
        get_inline_css_html(inline_css),
        css
    )
}

#[allow(clippy::too_many_arguments)]
pub fn replace_markers_2022(
    s: &str,
    html_ui: ftd::html::HtmlUI,
    config: &fastn_core::Config,
    main_id: &str,
    font_style: &str,
    base_url: &str,
) -> String {
    ftd::html::utils::trim_all_lines(
        s.replace(
            "__ftd_meta_data__",
            ftd::html::utils::get_meta_data(&html_ui.html_data).as_str(),
        )
        .replace(
            "__ftd_doc_title__",
            html_ui.html_data.title.unwrap_or_default().as_str(),
        )
        .replace("__ftd_data__", html_ui.variables.as_str())
        .replace(
            "__ftd_canonical_url__",
            config.package.generate_canonical_url(main_id).as_str(),
        )
        .replace(
            "__favicon_html_tag__",
            resolve_favicon(
                config.root.as_str(),
                config.package.name.as_str(),
                &config.package.favicon,
            )
            .unwrap_or_default()
            .as_str(),
        )
        .replace("__ftd_external_children__", "{}")
        .replace("__hashed_default_css__", hashed_default_css_name())
        .replace("__hashed_default_js__", hashed_default_js_name())
        .replace(
            "__ftd__",
            format!("{}{}", html_ui.html.as_str(), font_style).as_str(),
        )
        .replace(
            "__extra_js__",
            get_extra_js(
                config.ftd_external_js.as_slice(),
                config.ftd_inline_js.as_slice(),
                html_ui.js.as_str(),
                html_ui.rive_data.as_str(),
            )
            .as_str(),
        )
        .replace(
            "__extra_css__",
            get_extra_css(
                config.ftd_external_css.as_slice(),
                config.ftd_inline_css.as_slice(),
                html_ui.css.as_str(),
            )
            .as_str(),
        )
        .replace(
            "__ftd_functions__",
            format!(
                "{}\n{}\n{}\n{}\n{}\n{}\n{}",
                html_ui.functions.as_str(),
                html_ui.dependencies.as_str(),
                html_ui.variable_dependencies.as_str(),
                html_ui.dummy_html.as_str(),
                html_ui.raw_html.as_str(),
                html_ui.mutable_variable,
                html_ui.immutable_variable
            )
            .as_str(),
        )
        .replace("__ftd_body_events__", html_ui.outer_events.as_str())
        .replace("__ftd_element_css__", "")
        .replace("__base_url__", base_url)
        .as_str(),
    )
}

pub fn get_fastn_package_data(package: &fastn_core::Package) -> String {
    format!(
        indoc::indoc! {"
        let __fastn_package_name__ = \"{package_name}\";
    "},
        package_name = package.name
    )
}

pub fn replace_markers_2023(
    js_script: &str,
    scripts: &str,
    ssr_body: &str,
    font_style: &str,
    default_css: &str,
    base_url: &str,
    config: &fastn_core::Config,
) -> String {
    format!(
        include_str!("../../ftd/ftd-js.html"),
        fastn_package = get_fastn_package_data(&config.package).as_str(),
        base_url_tag = if !base_url.is_empty() {
            format!("<base href=\"{}\">", base_url)
        } else {
            "".to_string()
        },
        favicon_html_tag = resolve_favicon(
            config.root.as_str(),
            config.package.name.as_str(),
            &config.package.favicon,
        )
        .unwrap_or_default()
        .as_str(),
        js_script = format!("{js_script}{}", fastn_core::utils::available_code_themes()).as_str(),
        script_file = format!(
            r#"
                <script src="{}"></script>
                <script src="{}"></script>
                <script src="{}"></script>
                <link rel="stylesheet" href="{}">
                {}
            "#,
            hashed_markdown_js(),
            hashed_prism_js(),
            hashed_default_ftd_js(config.package.name.as_str()),
            hashed_prism_css(),
            scripts,
        )
        .as_str(),
        extra_js = get_extra_js(
            config.ftd_external_js.as_slice(),
            config.ftd_inline_js.as_slice(),
            "",
            "",
        )
        .as_str(),
        default_css = default_css,
        html_body = format!("{}{}", ssr_body, font_style).as_str(),
    )
}

pub fn is_test() -> bool {
    cfg!(test) || std::env::args().any(|e| e == "--test")
}

pub(crate) async fn write(
    root: &camino::Utf8PathBuf,
    file_path: &str,
    data: &[u8],
) -> fastn_core::Result<()> {
    if root.join(file_path).exists() {
        return Ok(());
    }
    update1(root, file_path, data).await
}

pub(crate) async fn overwrite(
    root: &camino::Utf8PathBuf,
    file_path: &str,
    data: &[u8],
) -> fastn_core::Result<()> {
    update1(root, file_path, data).await
}

// TODO: remove this function use update instead
pub(crate) async fn update1(
    root: &camino::Utf8PathBuf,
    file_path: &str,
    data: &[u8],
) -> fastn_core::Result<()> {
    use tokio::io::AsyncWriteExt;

    let (file_root, file_name) = if let Some((file_root, file_name)) = file_path.rsplit_once('/') {
        (file_root.to_string(), file_name.to_string())
    } else {
        ("".to_string(), file_path.to_string())
    };

    if !root.join(&file_root).exists() {
        tokio::fs::create_dir_all(root.join(&file_root)).await?;
    }

    Ok(
        tokio::fs::File::create(root.join(file_root).join(file_name))
            .await?
            .write_all(data)
            .await?,
    )
}

pub(crate) async fn copy(
    from: impl AsRef<camino::Utf8Path>,
    to: impl AsRef<camino::Utf8Path>,
) -> fastn_core::Result<()> {
    let content = fastn_core::tokio_fs::read(from.as_ref()).await?;
    fastn_core::utils::update(to, content.as_slice()).await
}

pub(crate) async fn update(
    root: impl AsRef<camino::Utf8Path>,
    data: &[u8],
) -> fastn_core::Result<()> {
    use tokio::io::AsyncWriteExt;

    let (file_root, file_name) = if let Some(file_root) = root.as_ref().parent() {
        (
            file_root,
            root.as_ref()
                .file_name()
                .ok_or_else(|| fastn_core::Error::UsageError {
                    message: format!(
                        "Invalid File Path: Can't find file name `{:?}`",
                        root.as_ref()
                    ),
                })?,
        )
    } else {
        return Err(fastn_core::Error::UsageError {
            message: format!(
                "Invalid File Path: file path doesn't have parent: {:?}",
                root.as_ref()
            ),
        });
    };

    if !file_root.exists() {
        tokio::fs::create_dir_all(file_root).await?;
    }

    Ok(tokio::fs::File::create(file_root.join(file_name))
        .await?
        .write_all(data)
        .await?)
}

pub(crate) fn ids_matches(id1: &str, id2: &str) -> bool {
    return strip_id(id1).eq(&strip_id(id2));

    fn strip_id(id: &str) -> String {
        let id = id
            .trim()
            .replace("/index.html", "/")
            .replace("index.html", "/");
        if id.eq("/") {
            return id;
        }
        id.trim_matches('/').to_string()
    }
}

/// Parse argument from CLI
/// If CLI command: fastn serve --identities a@foo.com,foo
/// key: --identities -> output: a@foo.com,foo
pub fn parse_from_cli(key: &str) -> Option<String> {
    use itertools::Itertools;
    let args = std::env::args().collect_vec();
    let mut index = None;
    for (idx, arg) in args.iter().enumerate() {
        if arg.eq(key) {
            index = Some(idx);
        }
    }
    index
        .and_then(|idx| args.get(idx + 1))
        .map(String::to_string)
}

/// Remove path: It can be directory or file
pub async fn remove(path: &std::path::Path) -> std::io::Result<()> {
    if path.is_file() {
        tokio::fs::remove_file(path).await?;
    } else if path.is_dir() {
        tokio::fs::remove_dir_all(path).await?
    } else if path.is_symlink() {
        // TODO:
        // It can be a directory or a file
    }
    Ok(())
}

/// Remove from provided `root` except given list
pub async fn remove_except(root: &camino::Utf8Path, except: &[&str]) -> fastn_core::Result<()> {
    use itertools::Itertools;
    let except = except
        .iter()
        .map(|x| root.join(x))
        .map(|x| x.into_std_path_buf())
        .collect_vec();
    let mut all = tokio::fs::read_dir(root).await?;
    while let Some(file) = all.next_entry().await? {
        if except.contains(&file.path()) {
            continue;
        }
        if file.metadata().await?.is_dir() {
            tokio::fs::remove_dir_all(file.path()).await?;
        } else if file.metadata().await?.is_file() {
            tokio::fs::remove_file(file.path()).await?;
        }
    }
    Ok(())
}

/// /api/?a=1&b=2&c=3 => vec[(a, 1), (b, 2), (c, 3)]
pub fn query(uri: &str) -> fastn_core::Result<Vec<(String, String)>> {
    use itertools::Itertools;
    Ok(
        url::Url::parse(format!("https://fifthtry.com/{}", uri).as_str())?
            .query_pairs()
            .into_owned()
            .collect_vec(),
    )
}
pub fn generate_hash(content: impl AsRef<[u8]>) -> String {
    use sha2::digest::FixedOutput;
    use sha2::Digest;
    let mut hasher = sha2::Sha256::new();
    hasher.update(content);
    format!("{:X}", hasher.finalize_fixed())
}

static CSS_HASH: once_cell::sync::Lazy<String> =
    once_cell::sync::Lazy::new(|| format!("default-{}.css", generate_hash(ftd::css())));

pub fn hashed_default_css_name() -> &'static str {
    &CSS_HASH
}

static JS_HASH: once_cell::sync::Lazy<String> = once_cell::sync::Lazy::new(|| {
    format!(
        "default-{}.js",
        generate_hash(format!("{}\n\n{}", ftd::build_js(), fastn_core::fastn_2022_js()).as_str())
    )
});

pub fn hashed_default_js_name() -> &'static str {
    &JS_HASH
}

static FTD_JS_HASH: once_cell::sync::OnceCell<String> = once_cell::sync::OnceCell::new();

pub fn hashed_default_ftd_js(package_name: &str) -> &'static str {
    FTD_JS_HASH.get_or_init(|| {
        format!(
            "default-{}.js",
            generate_hash(ftd::js::all_js_without_test(package_name).as_str())
        )
    })
}

static MARKDOWN_HASH: once_cell::sync::Lazy<String> =
    once_cell::sync::Lazy::new(|| format!("markdown-{}.js", generate_hash(ftd::markdown_js()),));

pub fn hashed_markdown_js() -> &'static str {
    &MARKDOWN_HASH
}

static PRISM_JS_HASH: once_cell::sync::Lazy<String> =
    once_cell::sync::Lazy::new(|| format!("prism-{}.js", generate_hash(ftd::prism_js().as_str()),));

pub fn hashed_prism_js() -> &'static str {
    &PRISM_JS_HASH
}

static PRISM_CSS_HASH: once_cell::sync::Lazy<String> = once_cell::sync::Lazy::new(|| {
    format!("prism-{}.css", generate_hash(ftd::prism_css().as_str()),)
});

pub fn hashed_prism_css() -> &'static str {
    &PRISM_CSS_HASH
}

static CODE_THEME_HASH: once_cell::sync::Lazy<ftd::Map<String>> =
    once_cell::sync::Lazy::new(|| {
        ftd::theme_css()
            .into_iter()
            .map(|(k, v)| (k, format!("code-theme-{}.css", generate_hash(v.as_str()))))
            .collect()
    });

pub fn hashed_code_theme_css() -> &'static ftd::Map<String> {
    &CODE_THEME_HASH
}

pub fn available_code_themes() -> String {
    let themes = hashed_code_theme_css();
    let mut result = vec![];
    for (theme, url) in themes {
        result.push(format!(
            "fastn_dom.codeData.availableThemes[\"{theme}\"] = \"{url}\";"
        ))
    }
    result.join("\n")
}

#[cfg(test)]
mod tests {
    #[test]
    fn query() {
        assert_eq!(
            super::query("/api/?a=1&b=2&c=3").unwrap(),
            vec![
                ("a".to_string(), "1".to_string()),
                ("b".to_string(), "2".to_string()),
                ("c".to_string(), "3".to_string())
            ]
        )
    }
}

pub fn ignore_headers() -> Vec<&'static str> {
    vec!["host", "x-forwarded-ssl"]
}

pub(crate) fn is_ftd_path(path: &str) -> bool {
    path.trim_matches('/').ends_with(".ftd")
}

#[derive(
    Clone,
    Debug,
    diesel::deserialize::FromSqlRow,
    diesel::expression::AsExpression,
    PartialOrd,
    PartialEq,
)]
#[diesel(sql_type = fastn_core::schema::sql_types::Citext)]
pub struct CiString(pub String);

pub fn citext(s: &str) -> CiString {
    CiString(s.into())
}

impl diesel::serialize::ToSql<fastn_core::schema::sql_types::Citext, diesel::pg::Pg> for CiString {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, diesel::pg::Pg>,
    ) -> diesel::serialize::Result {
        diesel::serialize::ToSql::<diesel::sql_types::Text, diesel::pg::Pg>::to_sql(&self.0, out)
    }
}

impl diesel::deserialize::FromSql<fastn_core::schema::sql_types::Citext, diesel::pg::Pg>
    for CiString
{
    fn from_sql(
        bytes: <diesel::pg::Pg as diesel::backend::Backend>::RawValue<'_>,
    ) -> diesel::deserialize::Result<Self> {
        Ok(CiString(diesel::deserialize::FromSql::<
            diesel::sql_types::Text,
            diesel::pg::Pg,
        >::from_sql(bytes)?))
    }
}
