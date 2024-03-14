use crate::error::RequestCreatorError;
use crate::{GetTaskResult, GetTaskResultResp, RequestCreator, SvcResponse, TaskRespTrait};
use serde::de::DeserializeOwned;

pub(super) struct GetTaskResultRepeater<'a> {
    td: GetTaskResult<'a>,
    rc: &'a RequestCreator<'a>,
}

impl<'a> GetTaskResultRepeater<'a> {
    pub(crate) const fn new(taskId: u32, client_key: &'a str, rc: &'a RequestCreator<'a>) -> Self {
        Self {
            td: GetTaskResult {
                clientKey: client_key,
                taskId,
            },
            rc,
        }
    }

    pub(crate) async fn check<Y: TaskRespTrait + DeserializeOwned + std::fmt::Debug + 'a>(
        &self,
    ) -> Result<SvcResponse<GetTaskResultResp<Y>>, RequestCreatorError> {
        self.rc.getTaskResult::<Y>(&self.td).await
    }
}
