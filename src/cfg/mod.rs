use reqwest::Url;

use crate::error::OptionsError::{self, UrlParseError};
use self::urls::Urls;

pub mod urls;
pub mod limits;

pub struct Config<'a> {
    pub(crate) client_key: &'a str,
    pub(crate) urls: Urls,
}

impl<'a> Config<'a> {
    const API_URI: &'a str = "https://api.capmonster.cloud";

    pub(crate) const SOFT_ID: &'a str = "60";

    pub fn new(client_key: &'a str) -> Result<Self, OptionsError>
    {
        let url = Url::parse(Self::API_URI).map_err(|e| UrlParseError(e.to_string()))?;

        Ok(Self {
            client_key,
            urls: Urls::from(&url),
        })
    }
}
