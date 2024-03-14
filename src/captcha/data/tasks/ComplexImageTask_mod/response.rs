use crate::TaskRespTrait;
use serde::Deserialize;

impl TaskRespTrait for ComplexImageTaskGridResp {}

impl TaskRespTrait for ComplexImageTaskCoordinateResp {}

#[derive(Deserialize, Clone, Debug)]
pub struct ComplexImageTaskGridResp {
    answer: Vec<bool>,
    metadata: Option<ComplexImageTaskMetadata>,
}

//TODO parse types

#[derive(Deserialize, Clone, Debug)]
pub struct ComplexImageTaskCoordinateResp {
    answer: Vec<ComplexImageTaskCoordinates>,
    metadata: Option<ComplexImageTaskMetadata>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ComplexImageTaskMetadata {
    AnswerType: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ComplexImageTaskCoordinates {
    X: f64,
    Y: f64,
}
