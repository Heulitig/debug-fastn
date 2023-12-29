pub(crate) mod apps;
pub(crate) mod document;
pub(crate) mod fetch_file;
pub(crate) mod figma_tokens;
pub(crate) mod figma_typography_tokens;
pub(crate) mod get_data;
pub(crate) mod google_sheets;
pub(crate) mod http;
pub(crate) mod lang;
pub(crate) mod lang_details;
pub(crate) mod package_query;
pub(crate) mod package_tree;
pub(crate) mod pg;
pub(crate) mod query;
pub(crate) mod request_data;
pub(crate) mod sitemap;
pub(crate) mod sql;
pub(crate) mod sqlite;
pub(crate) mod toc;
pub(crate) mod user_details;
pub(crate) mod user_group;

// pub enum Processor {
//     Toc,
//     GetData,
//     Sitemap,
//     FullSitemap,
//     DocumentReaders,
//     DocumentWriters,
//     UserGroupById,
//     UserGroups,
//     RequestData,
// }
//
// impl std::str::FromStr for Processor {
//     type Err = fastn_core::Error;
//
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         match s {
//             "toc" => Ok(Self::Toc),
//             "request-data" => Ok(Self::RequestData),
//             "sitemap" => Ok(Self::Sitemap),
//             _ => fastn_core::usage_error(format!("processor not found {s}")),
//         }
//     }
// }
