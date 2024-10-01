#![allow(non_snake_case, reason = "API")]

use crate::*;
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
    pub images: ImagesArray<'a>,

    pub metadata: ComplexImageTaskMetadata<'a>,
    pub exampleImageUrls: Option<&'a [&'a str]>,
    pub exampleImagesBase64: Option<&'a [&'a str]>,
    pub userAgent: Option<&'a str>,
    pub websiteUrl: Option<&'a str>,
}

#[allow(non_camel_case_types, reason = "API")]
#[skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
pub enum ImagesArray<'a> {
    imageUrls(&'a [&'a str]),
    imagesBase64(&'a [&'a str]),
}

#[skip_serializing_none]
#[derive(Serialize, Clone, Debug)]
pub struct ComplexImageTaskMetadata<'a> {
    pub Grid: &'a str,
    pub TaskDefinition: Option<&'a str>,
    pub Task: Option<&'a str>,
}