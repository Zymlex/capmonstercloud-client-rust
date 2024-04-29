use reqwest::{Client, Response, Url};

use crate::errors::HttpClientError;

pub(crate) struct HttpClient {
    http_client: Client,
}

impl HttpClient {
    pub(crate) fn new(http_client: Option<Client>) -> Self {
        Self {
            http_client: http_client.unwrap_or_else(|| Client::new()),
        }
    }
    
    pub(crate) async fn post<'a>(&self, url: Url, body: String) -> Result<Response, HttpClientError> {
        let response = self.http_client.post(url).body(body).send().await.unwrap(); //.map_err(|e| HttpClientError::PostRequestError(e))?
        
        Ok(response)
    }
}
