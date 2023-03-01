use reqwest::Url;

pub(crate) struct Urls {
    balance_url: Url,
    task_creation_url: Url,
    task_result_url: Url,
}

impl Urls {
    pub(crate) fn from(service_url: Url) -> Self {
        Self {
            balance_url: Self::get_full_url(&service_url, "getBalance"),
            task_creation_url: Self::get_full_url(&service_url, "createTask"),
            task_result_url: Self::get_full_url(&service_url, "getTaskResult"),
        }
    }
    
    pub(crate) fn get_balance_url(&self) -> Url {
        self.balance_url.clone()
    }
    
    pub(crate) fn task_creation_url(&self) -> Url {
        self.task_creation_url.clone()
    }
    
    pub(crate) fn get_task_result_url(&self) -> Url {
        self.task_result_url.clone()
    }
    
    fn get_full_url(url: &Url, url_path: &str) -> Url {
        let mut url = url.clone();
        url.set_path(url_path);
        url
    }
}
