use std::time::Duration;

pub(crate) struct GetResultTimeouts {
    first_request_delay: Duration,
    first_request_no_cache_delay: Duration,
    requests_interval: Duration,
    timeout: Duration,
}

// TODO Time

// impl<T: TaskReqTrait> GetResultTimeouts<T> {
//
// }
