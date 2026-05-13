use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RobotState {
    pub id: u32,
    pub ts: f64,
    #[serde(default)]
    pub joints: HashMap<String, f64>,
    #[serde(default)]
    pub servos: Vec<f64>,
    #[serde(default)]
    pub imu: HashMap<String, f64>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct JointDef {
    pub index: usize,
    pub name: String,
    pub min_deg: f64,
    pub max_deg: f64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RobotDescriptor {
    pub id: u32,
    pub model: String,
    #[serde(default)]
    pub urdf_model: String,
    pub joints: Vec<JointDef>,
}
