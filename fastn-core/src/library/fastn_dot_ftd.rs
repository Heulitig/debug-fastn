use crate::utils::HasElements;

async fn i18n_data(lib: &fastn_core::Library) -> String {
    let lang = match lib.config.config.package.selected_language {
        Some(ref lang) => {
            realm_lang::Language::from_2_letter_code(lang).unwrap_or(realm_lang::Language::English)
        }
        None => realm_lang::Language::English,
    };

    let primary_lang = match lib.config.config.package.translation_of.as_ref() {
        Some(ref package) => match package.selected_language {
            Some(ref lang) => realm_lang::Language::from_2_letter_code(lang)
                .unwrap_or(realm_lang::Language::English),
            None => lang,
        },
        None => lang,
    };

    let current_document_last_modified_on =
        fastn_core::utils::get_current_document_last_modified_on(
            &lib.config.config,
            lib.document_id.as_str(),
        )
        .await;

    format!(
        indoc::indoc! {"
            -- i18n-data i18n:
            current-language: {current_language}
            document: {document}
            language-detail-page-body: {language_detail_page_body}
            language-detail-page: {language_detail_page}
            language: {language}
            last-modified-on: {last_modified_on}
            missing: {missing}
            never-marked: {never_marked}
            never-synced: {never_synced}
            other-available-languages: {other_available_languages}
            out-dated-body: {out_dated_body}
            out-dated-heading: {out_dated_heading}
            out-dated: {out_dated}
            show-latest-version: {show_latest_version}
            show-outdated-version: {show_outdated_version}
            show-translation-status: {show_translation_status}
            show-unapproved-version: {show_unapproved_version}
            status: {status}
            total-number-of-documents: {total_number_of_documents}
            translation-not-available: {translation_not_available}
            unapproved-heading: {unapproved_heading}
            upto-date: {upto_date}
            welcome-fastn-page-subtitle: {welcome_fastn_page_subtitle}
            welcome-fastn-page: {welcome_fastn_page}
        "},
        current_language = fastn_core::i18n::translation::search(
            &lang,
            &primary_lang,
            "current-language",
            &current_document_last_modified_on
        ),
        document = fastn_core::i18n::translation::search(
            &lang,
            &primary_lang,
            "document",
            &current_document_last_modified_on
        ),
        language = fastn_core::i18n::translation::search(
            &lang,
            &primary_lang,
            "language",
            &current_document_last_modified_on
        ),
        language_detail_page = fastn_core::i18n::translation::search(
            &lang,
            &primary_lang,
            "language-detail-page",
            &current_document_last_modified_on
        ),
        language_detail_page_body = fastn_core::i18n::translation::search(
            &lang,
            &primary_lang,
            "language-detail-page-body",
            &current_document_last_modified_on
        ),
        last_modified_on = fastn_core::i18n::translation::search(
            &lang,
            &primary_lang,
            "last-modified-on",
            &current_document_last_modified_on
        ),
        never_synced = fastn_core::i18n::translation::search(
            &lang,
            &primary_lang,
            "never-synced",
            &current_document_last_modified_on
        ),
        missing = fastn_core::i18n::translation::search(
            &lang,
            &primary_lang,
            "missing",
            &current_document_last_modified_on
        ),
        never_marked = fastn_core::i18n::translation::search(
            &lang,
            &primary_lang,
            "never-marked",
            &current_document_last_modified_on
        ),
        other_available_languages = fastn_core::i18n::translation::search(
            &lang,
            &primary_lang,
            "other-available-languages",
            &current_document_last_modified_on
        ),
        out_dated = fastn_core::i18n::translation::search(
            &lang,
            &primary_lang,
            "out-dated",
            &current_document_last_modified_on
        ),
        out_dated_body = fastn_core::i18n::translation::search(
            &lang,
            &primary_lang,
            "out-dated-body",
            &current_document_last_modified_on
        ),
        out_dated_heading = fastn_core::i18n::translation::search(
            &lang,
            &primary_lang,
            "out-dated-heading",
            &current_document_last_modified_on
        ),
        show_latest_version = fastn_core::i18n::translation::search(
            &lang,
            &primary_lang,
            "show-latest-version",
            &current_document_last_modified_on
        ),
        show_outdated_version = fastn_core::i18n::translation::search(
            &lang,
            &primary_lang,
            "show-outdated-version",
            &current_document_last_modified_on
        ),
        show_translation_status = fastn_core::i18n::translation::search(
            &lang,
            &primary_lang,
            "show-translation-status",
            &current_document_last_modified_on
        ),
        show_unapproved_version = fastn_core::i18n::translation::search(
            &lang,
            &primary_lang,
            "show-unapproved-version",
            &current_document_last_modified_on
        ),
        status = fastn_core::i18n::translation::search(
            &lang,
            &primary_lang,
            "status",
            &current_document_last_modified_on
        ),
        total_number_of_documents = fastn_core::i18n::translation::search(
            &lang,
            &primary_lang,
            "total-number-of-documents",
            &current_document_last_modified_on
        ),
        translation_not_available = fastn_core::i18n::translation::search(
            &lang,
            &primary_lang,
            "translation-not-available",
            &current_document_last_modified_on
        ),
        unapproved_heading = fastn_core::i18n::translation::search(
            &lang,
            &primary_lang,
            "unapproved-heading",
            &current_document_last_modified_on
        ),
        upto_date = fastn_core::i18n::translation::search(
            &lang,
            &primary_lang,
            "upto-date",
            &current_document_last_modified_on
        ),
        welcome_fastn_page = fastn_core::i18n::translation::search(
            &lang,
            &primary_lang,
            "welcome-fastn-page",
            &current_document_last_modified_on
        ),
        welcome_fastn_page_subtitle = fastn_core::i18n::translation::search(
            &lang,
            &primary_lang,
            "welcome-fastn-page-subtitle",
            &current_document_last_modified_on
        ),
    )
}

fn construct_fastn_cli_variables(_lib: &fastn_core::Library) -> String {
    format!(
        indoc::indoc! {"
        -- fastn.build-info info:
        cli-version: {cli_version}
        cli-git-commit-hash: {cli_git_commit_hash}
        cli-created-on: {cli_created_on}
        build-created-on: {build_created_on}
        ftd-version: {ftd_version}
    "},
        cli_version = if fastn_core::utils::is_test() {
            "FASTN_CLI_VERSION"
        } else {
            env!("CARGO_PKG_VERSION")
        },
        cli_git_commit_hash = if fastn_core::utils::is_test() {
            "FASTN_CLI_GIT_HASH"
        } else {
            option_env!("GITHUB_SHA").unwrap_or("unknown-sha")
        },
        cli_created_on = if fastn_core::utils::is_test() {
            "FASTN_CLI_BUILD_TIMESTAMP"
        } else {
            // TODO: calculate this in github action and pass it, vergen is too heave a dependency
            option_env!("FASTN_CLI_BUILD_TIMESTAMP").unwrap_or("0")
        },
        ftd_version = if fastn_core::utils::is_test() {
            "FTD_VERSION"
        } else {
            ""
            // TODO
        },
        build_created_on = if fastn_core::utils::is_test() {
            String::from("BUILD_CREATE_TIMESTAMP")
        } else {
            std::time::SystemTime::now()
                .duration_since(std::time::SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
                .to_string()
        }
    )
}

pub(crate) async fn get2022_(lib: &fastn_core::Library) -> String {
    #[allow(clippy::format_in_format_args)]
    let mut fastn_base = format!(
        indoc::indoc! {"
            {fastn_base}
            {capital_fastn}

            {build_info}

            -- string document-name: {document_id}
            -- string package-title: {title}
            -- string package-name: {package_name}
            -- string home-url: {home_url}
        "},
        fastn_base = fastn_package::old_fastn::fastn_ftd_2021(),
        capital_fastn = capital_fastn(lib),
        build_info = construct_fastn_cli_variables(lib),
        document_id = lib.document_id,
        title = lib.config.config.package.name,
        package_name = lib.config.config.package.name,
        home_url = format!("https://{}", lib.config.config.package.name),
    );

    if let Ok(number_of_documents) = futures::executor::block_on(
        fastn_core::utils::get_number_of_documents(&lib.config.config),
    ) {
        fastn_base = format!(
            indoc::indoc! {"
                {fastn_base}
                
                -- number-of-documents: {number_of_documents}    
            "},
            fastn_base = fastn_base,
            number_of_documents = number_of_documents,
        );
    }

    if let Some((ref filename, ref content)) = lib.markdown {
        fastn_base = format!(
            indoc::indoc! {"
                {fastn_base}
                
                -- string markdown-filename: {filename}                        
                -- string markdown-content:
    
                {content}
            "},
            fastn_base = fastn_base,
            filename = filename,
            content = content,
        );
    }

    fastn_base
}

pub(crate) async fn get(lib: &fastn_core::Library) -> String {
    #[allow(clippy::format_in_format_args)]
    let mut fastn_base = format!(
        indoc::indoc! {"
            {fastn_base}
            {design_ftd}
            {capital_fastn}

            {i18n_data}

            {build_info}

            -- string document-id: {document_id}
            -- string translation-status-url: {home_url}
            -- string package-title: {title}
            -- string package-name: {package_name}
            -- string home-url: {home_url}
        "},
        fastn_base = fastn_package::old_fastn::fastn_ftd_2021(),
        design_ftd = fastn_core::design_ftd(),
        capital_fastn = capital_fastn(lib),
        i18n_data = i18n_data(lib).await,
        build_info = construct_fastn_cli_variables(lib),
        document_id = lib.document_id,
        title = lib.config.config.package.name,
        package_name = lib.config.config.package.name,
        home_url = format!("https://{}", lib.config.config.package.name),
    );

    if lib.config.config.package.translation_of.is_some() {
        fastn_base = format!(
            indoc::indoc! {"
                {fastn_base}
                
                -- is-translation-package: true
            "},
            fastn_base = fastn_base,
        );
    }

    if lib.config.config.package.translations.has_elements() {
        fastn_base = format!(
            indoc::indoc! {"
                {fastn_base}
                
                -- has-translations: true
            "},
            fastn_base = fastn_base,
        );
    }

    if let Some(ref zip) = lib.config.config.package.zip {
        fastn_base = format!(
            indoc::indoc! {"
                {fastn_base}
                
                -- package-zip: {package_zip}
            "},
            fastn_base = fastn_base,
            package_zip = zip,
        );
    }

    if let Some(ref diff) = lib.translated_data.diff {
        fastn_base = format!(
            indoc::indoc! {"
                {fastn_base}
                
                -- diff: 
                
                {diff}    
            "},
            fastn_base = fastn_base,
            diff = diff,
        );
    }

    if let Some(ref status) = lib.translated_data.status {
        fastn_base = format!(
            indoc::indoc! {"
                {fastn_base}
                
                -- translation-status: 
                
                {translation_status}    
            "},
            fastn_base = fastn_base,
            translation_status = status,
        );
    }

    if lib.config.config.package.translation_of.is_some()
        || lib.config.config.package.translations.has_elements()
    {
        fastn_base = format!(
            indoc::indoc! {"
                {fastn_base}
                
                -- translation-status-url: //{package_name}/-/translation-status/

            "},
            fastn_base = fastn_base,
            package_name = lib.config.config.package.name,
        );
    }

    if let Ok(number_of_documents) = futures::executor::block_on(
        fastn_core::utils::get_number_of_documents(&lib.config.config),
    ) {
        fastn_base = format!(
            indoc::indoc! {"
                {fastn_base}
                
                -- number-of-documents: {number_of_documents}    
            "},
            fastn_base = fastn_base,
            number_of_documents = number_of_documents,
        );
    }

    if let Some(last_modified_on) = futures::executor::block_on(
        fastn_core::utils::get_last_modified_on(&lib.config.config.root),
    ) {
        fastn_base = format!(
            indoc::indoc! {"
                {fastn_base}
                
                -- last-modified-on: {last_modified_on}    
            "},
            fastn_base = fastn_base,
            last_modified_on = last_modified_on,
        );
    }

    if let Some(last_modified_on) =
        futures::executor::block_on(fastn_core::utils::get_current_document_last_modified_on(
            &lib.config.config,
            lib.document_id.as_str(),
        ))
    {
        fastn_base = format!(
            indoc::indoc! {"
                {fastn_base}
                
                -- current-document-last-modified-on: {last_modified_on}    
            "},
            fastn_base = fastn_base,
            last_modified_on = last_modified_on,
        );
    }

    if let Some(ref language) = lib.config.config.package.selected_language {
        fastn_base = format!(
            indoc::indoc! {"
                {fastn_base}
                
                -- language: {language}     
            "},
            fastn_base = fastn_base,
            language = fastn_core::utils::language_to_human(language),
        );
    }

    if let Some(ref last_marked_on) = lib.translated_data.last_marked_on {
        let rfc3339 = fastn_core::utils::nanos_to_rfc3339(last_marked_on);
        fastn_base = format!(
            indoc::indoc! {"
                {fastn_base}
                
                -- last-marked-on: {last_marked_on}
                -- last-marked-on-rfc3339: {rfc3339}    
            "},
            fastn_base = fastn_base,
            last_marked_on = last_marked_on,
            rfc3339 = rfc3339,
        );
    }
    if let Some(ref original_latest) = lib.translated_data.original_latest {
        let rfc3339 = fastn_core::utils::nanos_to_rfc3339(original_latest);
        fastn_base = format!(
            indoc::indoc! {"
                {fastn_base}
                
                -- original-latest: {original_latest}
                -- original-latest-rfc3339: {rfc3339}    
            "},
            fastn_base = fastn_base,
            original_latest = original_latest,
            rfc3339 = rfc3339,
        );
    }
    if let Some(ref translated_latest) = lib.translated_data.translated_latest {
        let rfc3339 = fastn_core::utils::nanos_to_rfc3339(translated_latest);
        fastn_base = format!(
            indoc::indoc! {"
                {fastn_base}
                        
                -- translated-latest: {translated_latest}
                -- translated-latest-rfc3339: {rfc3339}    
            "},
            fastn_base = fastn_base,
            translated_latest = translated_latest,
            rfc3339 = rfc3339,
        );
    }
    if let Some((ref filename, ref content)) = lib.markdown {
        fastn_base = format!(
            indoc::indoc! {"
                {fastn_base}
                
                -- string markdown-filename: {filename}                        
                -- string markdown-content:
    
                {content}
            "},
            fastn_base = fastn_base,
            filename = filename,
            content = content,
        );
    }

    if let Ok(original_path) = lib.config.config.original_path() {
        let base_url = lib
            .base_url
            .as_str()
            .trim_end_matches('/')
            .trim_start_matches('/')
            .to_string();
        let base_url = if !base_url.is_empty() {
            format!("/{base_url}/")
        } else {
            String::from("/")
        };
        if let Ok(original_snapshots) =
            futures::executor::block_on(fastn_core::snapshot::get_latest_snapshots(&original_path))
        {
            if let Ok(translation_status) =
                fastn_core::commands::translation_status::get_translation_status(
                    &original_snapshots,
                    &lib.config.config.root,
                )
            {
                let mut never_marked_files = "".to_string();
                let mut missing_files = "".to_string();
                let mut outdated_files = "".to_string();
                let mut upto_date_files = "".to_string();
                let mut translation_status_list = "".to_string();

                for (file, status) in translation_status.iter() {
                    translation_status_list = format!(
                        indoc::indoc! {"
                            {list}
                            
                            -- status:
                            file: {file}
                            status: {status}                                    
                        "},
                        list = translation_status_list,
                        file = file,
                        status = status.as_str()
                    );
                    let url = match file.as_str().rsplit_once('.') {
                        Some(("index", "ftd")) => {
                            // Index.ftd found. Return index.html
                            format!("{base_url}index.html")
                        }
                        Some((file_path, "ftd")) | Some((file_path, "md")) => {
                            format!("{base_url}{file_path}/index.html")
                        }
                        Some(_) | None => {
                            // Unknown file found, create URL
                            format!(
                                "{base_url}{file_path}/index.html",
                                file_path = file.as_str()
                            )
                        }
                    };
                    let static_attrs = indoc::indoc! {"
                    is-disabled: false
                    is-heading: false"};

                    match status {
                        fastn_core::commands::translation_status::TranslationStatus::Missing => {
                            missing_files = format!(
                                indoc::indoc! {"
                                    {list}
                                    
                                    -- missing-files:
                                    title: {file}
                                    url: {url}
                                    {static_attrs}
                                "},
                                list = missing_files,
                                file = file,
                                url = url,
                                static_attrs = static_attrs,
                            );
                        }
                        fastn_core::commands::translation_status::TranslationStatus::NeverMarked => {
                            never_marked_files = format!(
                                indoc::indoc! {"
                                    {list}
                                    
                                    -- never-marked-files:
                                    title: {file}
                                    url: {url}
                                    {static_attrs}
                                    
                                "},
                                list = never_marked_files,
                                file = file,
                                url = url,
                                static_attrs = static_attrs,
                            );
                        }
                        fastn_core::commands::translation_status::TranslationStatus::Outdated => {
                            outdated_files = format!(
                                indoc::indoc! {"
                                    {list}
                                    
                                    -- outdated-files:
                                    title: {file}
                                    url: {url}
                                    {static_attrs}
                                    
                                "},
                                list = outdated_files,
                                file = file,
                                url = url,
                                static_attrs = static_attrs,
                            );
                        }
                        fastn_core::commands::translation_status::TranslationStatus::UptoDate => {
                            upto_date_files = format!(
                                indoc::indoc! {"
                                    {list}
                                    
                                    -- upto-date-files:
                                    title: {file}
                                    url: {url}
                                    {static_attrs}
                                    
                                "},
                                list = upto_date_files,
                                file = file,
                                url = url,
                                static_attrs = static_attrs,
                            );
                        }
                    }
                }

                fastn_base = format!(
                    indoc::indoc! {"
                        {fastn_base}
                        
                        -- record status-data:
                        string file:
                        string status:
                        
                        -- status-data list status:

                        {translation_status_list}

                        {missing_files}
                        
                        {never_marked_files}
                        
                        {outdated_files}
                        
                        {upto_date_files}
                    "},
                    fastn_base = fastn_base,
                    translation_status_list = translation_status_list,
                    missing_files = missing_files,
                    never_marked_files = never_marked_files,
                    outdated_files = outdated_files,
                    upto_date_files = upto_date_files
                );
            }
        }
    }

    if lib.config.config.package.translations.has_elements() {
        let mut translation_status_list = "".to_string();
        for translation in lib.config.config.package.translations.iter() {
            if let Some(ref status) = translation.translation_status_summary {
                if let Some(ref language) = translation.selected_language {
                    let url = format!("https://{}/-/translation-status/", translation.name);
                    let status = {
                        let mut status_data = format!(
                            indoc::indoc! {"
                                -- all-language-translation-status:
                                language: {language}
                                url: {url}
                                never-marked: {never_marked}
                                missing: {missing}
                                out-dated: {out_dated}
                                upto-date: {upto_date}
                            "},
                            language = language,
                            url = url,
                            never_marked = status.never_marked,
                            missing = status.missing,
                            out_dated = status.out_dated,
                            upto_date = status.upto_date
                        );
                        if let Some(ref last_modified_on) = status.last_modified_on {
                            status_data = format!(
                                indoc::indoc! {"
                                    {status}last-modified-on: {last_modified_on}
                                "},
                                status = status_data,
                                last_modified_on = last_modified_on
                            );
                        }
                        status_data
                    };
                    translation_status_list = format!(
                        indoc::indoc! {"
                            {list}
                            
                            {status}
                            
                        "},
                        list = translation_status_list,
                        status = status
                    );
                }
            }
        }

        fastn_base = format!(
            indoc::indoc! {"
                {fastn_base}
            
                {translation_status_list}
            "},
            fastn_base = fastn_base,
            translation_status_list = translation_status_list
        );
    }

    let other_language_packages =
        if let Some(translation_of) = lib.config.config.package.translation_of.as_ref() {
            let mut other_language_packages = translation_of
                .translations
                .iter()
                .collect::<Vec<&fastn_core::Package>>();
            other_language_packages.insert(0, translation_of);
            other_language_packages
        } else {
            lib.config
                .config
                .package
                .translations
                .iter()
                .collect::<Vec<&fastn_core::Package>>()
        };

    if other_language_packages.has_elements() {
        let mut languages = "".to_string();
        let doc_id = if lib.document_id.eq("index.ftd") {
            "".to_string()
        } else {
            lib.document_id.replace(".ftd", "/")
        };

        for lang_package in other_language_packages {
            let language = if let Some(ref lang) = lang_package.selected_language {
                fastn_core::utils::language_to_human(lang)
            } else {
                continue;
            };

            let domain = if lang_package.name.ends_with('/') {
                format!("https://{}{}", lang_package.name, doc_id)
            } else {
                format!("https://{}/{}", lang_package.name, doc_id)
            };

            languages = format!(
                indoc::indoc! {"
                    {languages}
                    - {language}
                      url: {domain}
                "},
                languages = languages,
                domain = domain,
                language = language,
            );
        }

        if !languages.trim().is_empty() {
            fastn_base = format!(
                indoc::indoc! {"
                    {fastn_base}
                    
                    -- language-toc:
                    {processor_marker}: toc
        
                    {languages}
                "},
                fastn_base = fastn_base,
                languages = languages,
                processor_marker = ftd::PROCESSOR_MARKER,
            );
        }
    }

    fastn_base
}

pub(crate) async fn get2(lib: &fastn_core::Library2) -> String {
    let lib = fastn_core::Library {
        config: lib.config.clone(),
        markdown: lib.markdown.clone(),
        document_id: lib.document_id.clone(),
        translated_data: lib.translated_data.clone(),
        asset_documents: Default::default(),
        base_url: lib.base_url.clone(),
    };
    get(&lib).await
}

pub(crate) async fn get2022(lib: &fastn_core::Library2022) -> String {
    let lib = fastn_core::Library {
        config: lib.clone(),
        markdown: lib.markdown.clone(),
        document_id: lib.document_id.clone(),
        translated_data: lib.translated_data.clone(),
        asset_documents: Default::default(),
        base_url: lib.base_url.clone(),
    };
    get2022_(&lib).await
}

fn capital_fastn(lib: &fastn_core::Library) -> String {
    let mut s = format!(
        indoc::indoc! {"
            -- package-data package: {package_name}
        "},
        package_name = lib.config.config.package.name,
    );

    if let Some(ref zip) = lib.config.config.package.zip {
        s.push_str(format!("zip: {}", zip).as_str());
    }

    if let Some(ref favicon) = lib.config.config.package.favicon {
        s.push_str(format!("\nfavicon: {}", favicon).as_str());
    }

    s
}
