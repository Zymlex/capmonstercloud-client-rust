use crate::cfg::limits;
use crate::cfg::limits::{Limits, LimitsTrait};
use crate::error::{CMCTaskError, RequestCreatorError, TaskResultError};
use crate::*;
use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::marker::PhantomData;
use std::time::{Duration, Instant};
use tokio::time::sleep;

pub(crate) struct CMCTask<
    'a,
    T: TaskReqTrait,
    Y: TaskRespTrait + DeserializeOwned + std::fmt::Debug + 'a,
> where
    Limits<T>: LimitsTrait,
{
    task_id: u32,

    creation_time: Instant,
    requests_counter: u8,

    rc: &'a RequestCreator<'a>,

    #[allow(non_snake_case)]
    #[doc(hidden)]
    __: PhantomData<T>,

    #[allow(non_snake_case)]
    #[doc(hidden)]
    ___: PhantomData<Y>,
}

impl<
        'a,
        T: TaskReqTrait + Serialize,
        Y: TaskRespTrait + DeserializeOwned + std::fmt::Debug + 'a,
    > CMCTask<'a, T, Y>
where
    Limits<T>: LimitsTrait,
{
    pub(crate) async fn new(
        rc: &'a RequestCreator<'a>,
        task_body: T,
    ) -> Result<CMCTask<'a, T, Y>, CMCTaskError>
    where
        Limits<T>: LimitsTrait,
    {
        let resp = rc
            .createTask(task_body)
            .await
            .map_err(CMCTaskError::TaskCreationError)?;

        let task_id = resp
            .get_result()
            .map_err(CMCTaskError::CreateTaskGetResultError)?;

        Ok(Self {
            task_id,

            creation_time: Instant::now(),
            requests_counter: 0,

            rc,

            __: PhantomData,
            ___: PhantomData,
        })
    }

    pub(crate) async fn get_task_result_in_loop(
        &mut self,
    ) -> Result<SvcResponse<GetTaskResultResp<Y>>, TaskResultError> {
        enum State<Y: TaskRespTrait + DeserializeOwned + std::fmt::Debug> {
            Good(SvcResponse<GetTaskResultResp<Y>>),
            Repeat,
            Bad(TaskResultError),
        }

        loop {
            self.add_request_count();

            let resp_result = self
                .rc
                .getTaskResult::<Y>(self.task_id)
                .await;

            let state: State<Y> = match resp_result {
                Err(e) => match e {
                    RequestCreatorError::NonSuccessResponseStatus(s) => match s {
                        StatusCode::INTERNAL_SERVER_ERROR
                        | StatusCode::BAD_GATEWAY
                        | StatusCode::SERVICE_UNAVAILABLE => State::Repeat,

                        _ => State::Bad(TaskResultError::BadStatusCode(s)),
                    },
                    _ => State::Bad(TaskResultError::RequestError(e)),
                },
                Ok(resp) => State::Good(resp),
            };

            return match state {
                State::Good(r) => Ok(r),
                State::Repeat => {
                    self.check_and_wait_req_interval().await?;
                    continue;
                }
                State::Bad(e) => Err(e),
            };
        }
    }

    async fn check_and_wait_req_interval(&self) -> Result<(), TaskResultError> {
        let lifetime_on_next_check = self.creation_time.elapsed() + Self::request_interval();
        let count = self.requests_counter;

        if lifetime_on_next_check < Self::result_timeout() {
            if count < limits::TASK_REQUEST_MAX {
                sleep(Self::request_interval()).await;
                Ok(())
            } else {
                Err(TaskResultError::RequestsLimitReached)
            }
        } else {
            Err(TaskResultError::GetResultTotalTimeout)
        }
    }

    fn add_request_count(&mut self) {
        self.requests_counter += 1;
    }

    const fn request_interval() -> Duration {
        Limits::<T>::REQUEST_INTERVAL
    }

    const fn result_timeout() -> Duration {
        Limits::<T>::RESULT_TIMEOUT
    }
}
