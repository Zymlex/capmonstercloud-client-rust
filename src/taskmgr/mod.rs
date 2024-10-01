use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::cfg::Config;
use crate::error::{GetBalanceError, RequestCreatorError};
use self::task::CMCTask;
use crate::*;
use crate::cfg::limits::{Limits, LimitsTrait};

mod task;
mod timeouts;

pub(crate) struct Solver<'a> {
    rc: RequestCreator<'a>,
}

impl<'a> Solver<'a> {
    pub(crate) fn new(cfg: Config<'a>) -> Result<Self, RequestCreatorError> {
        Ok(Self {
            rc: RequestCreator::new(cfg.client_key, cfg.urls)?,
        })
    }

    pub(crate) async fn get_balance_async(
        &self,
    ) -> Result<<GetBalanceResp as SvcRespTypeTrait>::Value, GetBalanceError> {
        let resp_obj = self
            .rc
            .getBalance()
            .await
            .map_err(GetBalanceError::RequestError)?;

        Ok(resp_obj.get_result().map_err(GetBalanceError::SvcResponseError)?)
    }

    pub(crate) async fn solve_impl<
        T: TaskReqTrait + Serialize + 'a,
        Y: TaskRespTrait + DeserializeOwned + std::fmt::Debug + 'a,
    >(
        &self,
        data: T,
    ) -> Result<<GetTaskResultResp<Y> as SvcRespTypeTrait>::Value, SolveError>
    where
        Limits<T>: LimitsTrait,
    {
        let mut task = CMCTask::<T, Y>::new(&self.rc, data)
            .await
            .map_err(SolveError::TaskCreationError)?;

        let res = task
            .get_task_result_in_loop()
            .await
            .map_err(SolveError::GetTaskResultErrorInLoop)?;

        Ok(res)
    }
}
