use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum TriggerType {
    success,
    manual,
}

#[derive(Serialize, Deserialize)]
pub struct Authorization {
    pub roles: Vec<String>,
    pub users: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Approval {
    #[serde(alias = "type")]
    #[serde(rename = "type")]
    pub trigger_type: TriggerType,
    pub authorization: Authorization,
}
