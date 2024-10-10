use reqwest::Url;

use crate::error::OptionsError::{self, UrlParseError, UrlsError};
use self::urls::Urls;

pub mod urls;
pub mod limits;

pub struct Config<'a> {
    pub(crate) client_key: &'a str,
    pub(crate) urls: Urls,
}

impl<'a> Config<'a> {
    const SOLVING_API_URI: &'a str = "https://api.capmonster.cloud";
    const SITE_API_URI   : &'a str = "https://capmonster.cloud/api/";

    pub(crate) const SOFT_ID: &'a str = "60";

    pub fn new(client_key: &'a str) -> Result<Self, OptionsError>
    {
        let solving_api_uri = Url::parse(Self::SOLVING_API_URI).map_err(|e| UrlParseError(e.to_string()))?;
        let site_api_uri = Url::parse(Self::SITE_API_URI).map_err(|e| UrlParseError(e.to_string()))?;

        Ok(Self {
            client_key,
            urls: Urls::from(&solving_api_uri, &site_api_uri).map_err(UrlsError)?,
        })
    }
}
