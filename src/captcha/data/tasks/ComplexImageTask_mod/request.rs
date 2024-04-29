use crate::TaskReqTrait;
use serde::Serialize;
use serde_with_macros::skip_serializing_none;

impl<'a> TaskReqTrait for ComplexImageTask<'a> {}

#[skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
#[serde(tag = "type")]
pub(crate) struct ComplexImageTask<'a> {
    pub(crate) class: &'a str,

    #[serde(flatten)]
    pub(crate) data: ComplexImageTaskData<'a>,
}

#[skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
#[serde(tag = "type")]
pub struct ComplexImageTaskData<'a> {
    #[serde(flatten)]
    images: ImagesArray<'a>,

    metadata: ComplexImageTaskMetadata<'a>,
    exampleImageUrls: Option<&'a [&'a str]>,
    exampleImagesBase64: Option<&'a [&'a str]>,
    userAgent: Option<&'a str>,
    websiteUrl: Option<&'a str>,
}

#[allow(non_camel_case_types)]
#[skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
pub enum ImagesArray<'a> {
    imageUrls(&'a [&'a str]),
    imagesBase64(&'a [&'a str]),
}

#[skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
struct ComplexImageTaskMetadata<'a> {
    pub Grid: &'a str,
    pub TaskDefinition: Option<&'a str>,
    pub Task: Option<&'a str>,
}
