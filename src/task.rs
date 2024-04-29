use std::time::{Duration, Instant};
use tokio::time::sleep;

use crate::error::TaskResultError;
use crate::limits;
use crate::limits::{Limits, LimitsTrait};
use crate::requests::tasks_names::TaskReqTrait;

pub(crate) struct Task<T: TaskReqTrait>
where
    Limits<T>: LimitsTrait,
{
    pub(crate) task_id: u32,
    creation_time: Instant,
    requests_counter: u8,
    limits: Limits<T>,
}

impl<T: TaskReqTrait> Task<T>
where
    Limits<T>: LimitsTrait,
{
    pub(crate) fn new(task_id: u32) -> Self {
        Self {
            task_id,
            creation_time: Instant::now(),
            requests_counter: 0,
            limits: Limits::<T>::new(),
        }
    }

    fn request_interval(&self) -> Duration {
        self.limits.request_interval()
    }

    pub(crate) async fn check_and_wait_req_interval(&self) -> Result<(), TaskResultError> {
        let lifetime_on_next_check = self.creation_time.elapsed() + self.request_interval();
        let count = self.requests_counter;

        if lifetime_on_next_check < self.limits.result_timeout() {
            if count < limits::TASK_REQUEST_MAX {
                sleep(self.request_interval()).await;
                Ok(())
            } else {
                Err(TaskResultError::RequestsLimitReached)
            }
        } else {
            Err(TaskResultError::GetResultTimeout)
        }
    }

    pub(crate) fn add_request_count(&mut self) {
        self.requests_counter += 1;
    }
}
