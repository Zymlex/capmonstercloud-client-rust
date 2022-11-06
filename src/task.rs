use std::time::Instant;

use crate::errors::TaskInvalid;

pub(crate) struct Task {
    is_valid: bool,
    // task_id: u32,
    time: Instant,
    requests_counter: u8,
}

impl Task {
    pub(crate) fn new() -> Self {
        Self {
            is_valid: true,
            time: Instant::now(),
            requests_counter: 0,
        }
    }
    
    pub(crate) fn add_count(&mut self) {
        self.requests_counter = self.requests_counter + 1;
    }
    
    pub(crate) fn check_state(&self) -> Result<(), TaskInvalid> {
        if self.is_valid {
            if self.requests_counter <= 120 {
                Ok(())
            } else {
                Err(TaskInvalid::RequestsLimitReached)
            }
        } else {
            Err(TaskInvalid::TaskInvalid)
        }
    }
}
