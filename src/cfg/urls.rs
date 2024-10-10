use reqwest::Url;

use crate::error::UrlsError;

pub(crate) struct Urls {
    pub(crate) user_agent   : Url,
    pub(crate) balance      : Url,
    pub(crate) task_creation: Url,
    pub(crate) task_result  : Url,
}

impl Urls {
    pub(crate) fn from(solving_api_uri: &Url, site_api_uri: &Url) -> Result<Self, UrlsError> {
        Ok(Self {
            user_agent   : Self::get_full_url(site_api_uri, "useragent/actual")?,
            balance      : Self::get_full_url(solving_api_uri, "getBalance")?,
            task_creation: Self::get_full_url(solving_api_uri, "createTask")?,
            task_result  : Self::get_full_url(solving_api_uri, "getTaskResult")?,
        })
    }
    
    fn get_full_url(url: &Url, url_path: &str) -> Result<Url, UrlsError> {
        let url = url.join(url_path).map_err(|e| UrlsError::UrlParseError(e.to_string()))?;
        Ok(url)
    }
}