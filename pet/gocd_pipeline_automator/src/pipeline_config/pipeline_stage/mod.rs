mod task;
mod approval;

use serde::{Serialize, Deserialize};
use crate::pipeline_config::pipeline_stage::task::Task;
use crate::pipeline_config::pipeline_stage::approval::Approval;

#[derive(Serialize, Deserialize)]
pub struct EnvironmentVariable {
    key: String,
    value: String,
    encrypted: bool,
    secure: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Job {
    #[serde(alias = "name")]
    #[serde(rename = "name")]
    pub job_name: String,
    pub timeout: u32,
    pub environment_variables: Vec<EnvironmentVariable>,
    pub run_instance_count: Option<String>,
    pub resources: Vec<String>,
    pub tasks: Vec<Task>,
}

#[derive(Serialize, Deserialize)]
pub struct PipelineStage {
    #[serde(alias = "name")]
    #[serde(rename = "name")]
    pub stage_name: String,
    pub fetch_materials: bool,
    pub clean_working_directory: bool,
    pub never_cleanup_artifacts: bool,
    pub approval: Approval,
    pub environment_variables: Vec<EnvironmentVariable>,
    pub jobs: Vec<Job>,
}
