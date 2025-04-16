pub mod limits;
pub mod urls;

#[derive(Clone)]
pub(crate) struct APIRootUrls<'a> {
    pub(crate) solving: &'a str,
    pub(crate) site: &'a str,
}

pub(crate) struct Settings<'a> {
    pub(crate) api_roots: &'a APIRootUrls<'a>,
    pub(crate) soft_id: &'a str,
}
