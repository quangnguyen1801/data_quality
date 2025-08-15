use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConfigTestParameter {
    pub id: i32,
    pub test_id: i32,
    pub test_parameter_id: i32,
    pub inc_excl: String,
    pub operator: i32,
    pub vlow: String,
    pub vhigh: String,
}
