use reqwest::Url;

use crate::errors::ClientOptionsError;

pub struct ClientOptions<'a> {
    pub(crate) client_key: &'a str,
    // pub(crate) callback_url: Option<&'a str>,
    pub(crate) service_uri: Url,
    pub(crate) soft_id: u32,
}

impl<'a> ClientOptions<'a> {
    const DEFAULT_API_URI: &'a str = "https://api.capmonster.cloud";
    const DEFAULT_SOFT_ID: u32 = 0;
    
    pub fn new(client_key: &'a str) -> Result<Self, ClientOptionsError> {
        Self::new_ex(client_key, None, None)
    }
    
    pub fn new_ex(
        client_key: &'a str,
        service_uri: Option<&'a str>,
        soft_id: Option<u32>,
    ) -> Result<Self, ClientOptionsError> {
        Ok(Self {
            client_key,
            service_uri: Url::parse(service_uri.unwrap_or(Self::DEFAULT_API_URI)).unwrap(),
            soft_id: soft_id.unwrap_or(Self::DEFAULT_SOFT_ID),
        })
    }
}
