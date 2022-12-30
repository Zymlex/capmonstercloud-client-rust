use std::time::{Duration, Instant};

use crate::error::TaskResultError;
use crate::limits::{Limits, LimitsTrait};
use crate::requests::TaskReqTrait;

pub(crate) struct Task<T: TaskReqTrait> where Limits<T>: LimitsTrait {
    pub(crate) task_id: u32,
    creation_time: Instant,
    requests_counter: u8,
    limits: Limits<T>,
}

impl<T: TaskReqTrait> Task<T> where Limits<T>: LimitsTrait {
    pub(crate) fn new(task_id: u32) -> Self {
        Self {
            task_id,
            creation_time: Instant::now(),
            requests_counter: 0,
            limits: Limits::<T>::new(),
        }
    }
    
    pub(crate) fn request_interval(&self) -> Duration {
        self.limits.request_interval()
    }
    
    pub(crate) fn add_request_count(&mut self) {
        self.requests_counter += 1;
    }
    
    pub(crate) fn check_before_wait(&self) -> Result<(), TaskResultError> {
        let elapsed = self.creation_time.elapsed() + self.request_interval();
        let count = self.requests_counter;
    
        if elapsed < self.limits.result_timeout() {
            if count < 120 {
                Ok(())
            } else {
                Err(TaskResultError::RequestsLimitReached)
            }
        } else {
            Err(TaskResultError::GetResultTimeout)
        }
    }
}