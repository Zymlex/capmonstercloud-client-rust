use reqwest::Url;

pub(crate) struct Urls {
    pub(crate) balance: Url,
    pub(crate) task_creation: Url,
    pub(crate) task_result: Url,
}

impl Urls {
    pub(crate) fn from(service_url: &Url) -> Self {
        Self {
            balance      : Self::get_full_url(service_url, "getBalance"),
            task_creation: Self::get_full_url(service_url, "createTask"),
            task_result  : Self::get_full_url(service_url, "getTaskResult"),
        }
    }
    
    fn get_full_url(url: &Url, url_path: &str) -> Url {
        let mut url = url.clone();
        url.set_path(url_path);
        url
    }
}