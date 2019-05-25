use crate::pipeline_config::material::Material;
use crate::pipeline_config::pipeline_stage::PipelineStage;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum LockBehavior {
    lockOnFailure,
    unlockWhenFinished,
}

#[derive(Serialize, Deserialize)]
pub struct Pipeline {
    #[serde(alias = "name")]
    #[serde(rename = "name")]
    pub pipeline_name: String,
    pub label_template: Option<String>,
    pub lock_behavior: Option<LockBehavior>,
    pub materials: Vec<Material>,
    #[serde(alias = "stages")]
    #[serde(rename = "stages")]
    pub pipeline_stages: Vec<PipelineStage>,
}