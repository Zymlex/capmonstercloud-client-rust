use reqwest::Url;

pub(crate) struct Urls {
    pub(crate) balance_url: Url,
    pub(crate) task_creation_url: Url,
    pub(crate) task_result_url: Url,
}

impl Urls {
    pub(crate) fn from(service_url: &Url) -> Self {
        Self {
            balance_url      : Self::get_full_url(service_url, "getBalance"),
            task_creation_url: Self::get_full_url(service_url, "createTask"),
            task_result_url  : Self::get_full_url(service_url, "getTaskResult"),
        }
    }
    
    fn get_full_url(url: &Url, url_path: &str) -> Url {
        let mut url = url.clone();
        url.set_path(url_path);
        url
    }
}