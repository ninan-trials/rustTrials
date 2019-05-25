mod pipeline;
mod material;
mod pipeline_stage;

use serde::{Serialize, Deserialize};
use crate::pipeline_config::pipeline::Pipeline;

#[derive(Serialize, Deserialize)]
pub struct PipelineConfig {
    pub group: String,
    pub pipeline: Pipeline,
}