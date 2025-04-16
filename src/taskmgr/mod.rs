use cfg::urls::Urls;
use cfg::{APIRootUrls, Settings};
use error::GetUserAgentError;
use serde::Serialize;
use serde::de::DeserializeOwned;

use self::task::CMCTask;
use crate::cfg::limits::{Limits, LimitsTrait};
use crate::error::{GetBalanceError, RequestCreatorError};
use crate::*;

mod task;
mod timeouts;

const FIXED_SETTINGS: Settings = Settings {
    api_roots: &APIRootUrls {
        solving: "https://api.capmonster.cloud",
        site: "https://capmonster.cloud/api/",
    },

    soft_id: "60",
};

pub(crate) struct Solver<'a> {
    rc: RequestCreator<'a>,
}

impl<'a> Solver<'a> {
    pub(crate) fn new(client_key: &'a str) -> Result<Self, RequestCreatorError> {
        let urls = Urls::new(&FIXED_SETTINGS.api_roots).map_err(RequestCreatorError::Urls)?;

        Ok(Self {
            rc: RequestCreator::new(urls, FIXED_SETTINGS.soft_id, client_key)?,
        })
    }

    pub(crate) async fn get_user_agent_async(&self) -> Result<String, GetUserAgentError> {
        let ua = self
            .rc
            .getUserAgent()
            .await
            .map_err(GetUserAgentError::Request)?;

        Ok(ua)
    }

    pub(crate) async fn get_balance_async(
        &self,
    ) -> Result<<GetBalanceResp as SvcRespTypeTrait>::Value, GetBalanceError> {
        let resp_obj = self
            .rc
            .getBalance()
            .await
            .map_err(GetBalanceError::Request)?;

        resp_obj.get_result().map_err(GetBalanceError::SvcResponse)
    }

    pub(crate) async fn solve_impl<
        T: TaskReqTrait + Serialize + Send + Sync + 'a,
        Y: TaskRespTrait + DeserializeOwned + std::fmt::Debug + 'a,
    >(
        &self,
        data: T,
    ) -> Result<Y, SolveError>
    where
        Limits<T>: LimitsTrait,
    {
        let mut task = CMCTask::<T, Y>::new(&self.rc, data)
            .await
            .map_err(SolveError::TaskCreation)?;

        let res = task
            .get_task_result_in_loop()
            .await
            .map_err(SolveError::GetTaskResultInLoop)?;

        Ok(res)
    }
}

#[cfg(test)]
impl<'a> Solver<'a> {
    pub(crate) fn new_for_tests(
        api_roots: &'a APIRootUrls<'a>,
        client_key: &'a str,
    ) -> Result<Self, RequestCreatorError> {
        let urls = Urls::new(api_roots).map_err(RequestCreatorError::Urls)?;

        Ok(Self {
            rc: RequestCreator::new(urls, FIXED_SETTINGS.soft_id, client_key)?,
        })
    }
}
