use crate::*;
use serde::Deserialize;

impl TaskRespTrait for ComplexImageTaskGridResp {}

impl TaskRespTrait for ComplexImageTaskCoordinateResp {}

#[derive(Deserialize, Clone, Debug)]
pub struct ComplexImageTaskGridResp {
    answer: Vec<bool>,
    metadata: Option<ComplexImageTaskMetadataResp>,
}

//TODO parse types

#[derive(Deserialize, Clone, Debug)]
pub struct ComplexImageTaskCoordinateResp {
    answer: Vec<ComplexImageTaskCoordinates>,
    metadata: Option<ComplexImageTaskMetadataResp>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ComplexImageTaskMetadataResp {
    AnswerType: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ComplexImageTaskCoordinates {
    X: f64,
    Y: f64,
}