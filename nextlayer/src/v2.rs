use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum ResourceType {
    #[serde(rename = "register")]
    Register,
    #[serde(rename = "memory")]
    Memory,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Resource {
    pub kind: ResourceType,
    pub id: u32,
    pub width: u32,
    pub path: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Interface {
    pub name: String,
    pub instance: String,
    pub clock: String,
    pub reset: String,
    #[serde(rename = "resource")]
    pub resources: Vec<Resource>,
}
