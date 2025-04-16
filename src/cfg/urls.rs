use reqwest::Url;

use crate::error::UrlsError;

use super::APIRootUrls;

pub(crate) struct Urls {
    pub(crate) user_agent: Url,
    pub(crate) balance: Url,
    pub(crate) task_creation: Url,
    pub(crate) task_result: Url,
}

impl Urls {
    pub(crate) fn new<'a>(api_roots: &APIRootUrls<'a>) -> Result<Self, UrlsError> {
        let site_root_uri =
            Url::parse(api_roots.site).map_err(|e| UrlsError::UrlParse(e.to_string()))?;

        println!("Urls site_root_uri: {site_root_uri}");

        let solving_root_uri =
            Url::parse(api_roots.solving).map_err(|e| UrlsError::UrlParse(e.to_string()))?;

        println!("Urls solving_root_uri: {solving_root_uri}");

        #[rustfmt::skip]
        Ok(Self {
            user_agent:    Self::get_full_url(&site_root_uri, "useragent/actual")?,
            balance:       Self::get_full_url(&solving_root_uri, "getBalance")?,
            task_creation: Self::get_full_url(&solving_root_uri, "createTask")?,
            task_result:   Self::get_full_url(&solving_root_uri, "getTaskResult")?,
        })
    }

    fn get_full_url(url: &Url, url_path: &str) -> Result<Url, UrlsError> {
        let url = url
            .join(url_path)
            .map_err(|e| UrlsError::UrlParse(e.to_string()))?;
        Ok(url)
    }
}
