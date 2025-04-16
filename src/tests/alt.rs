use crate::{
    CapMonsterCloudClient, cfg::APIRootUrls, error::CapMonsterCloudClientError, taskmgr::Solver,
};

impl<'a> CapMonsterCloudClient<'a> {
    pub(super) fn new_for_tests(
        api_roots: &'a APIRootUrls<'a>,
        client_key: &'a str,
    ) -> Result<Self, CapMonsterCloudClientError> {
        Ok(Self {
            taskmgr: Solver::new_for_tests(api_roots, client_key)
                .map_err(CapMonsterCloudClientError::ClientImpl)?,
        })
    }
}
