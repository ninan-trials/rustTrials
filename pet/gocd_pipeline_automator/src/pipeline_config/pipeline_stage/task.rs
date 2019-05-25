use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum TaskType {
    exec,
    ant,
    nant,
    rake,
    fetch,
    pluggable_task,
}

#[derive(Serialize, Deserialize)]
pub enum TaskRunCondition {
    passed,
    failed,
    any,
}

#[derive(Serialize, Deserialize)]
pub struct TaskAttribute {
    #[serde(alias = "run_if")]
    #[serde(rename = "run_if")]
    pub task_run_condition: Vec<TaskRunCondition>,
    pub command: String,
}

#[derive(Serialize, Deserialize)]
pub struct Task {
    #[serde(alias = "type")]
    #[serde(rename = "type")]
    pub task_type: TaskType,
    pub attributes: TaskAttribute,
}