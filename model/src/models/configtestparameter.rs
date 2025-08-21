use serde::{Deserialize, Serialize};
use tiberius_mappers::TryFromRow;

#[derive(Debug, Serialize, Deserialize, Clone, TryFromRow)]
pub struct ConfigTestParameter {
    pub id: i32,
    pub test_id: i32,
    pub test_parameter_id: i32,
    pub inc_excl: String,
    pub operator: i32,
    pub vlow: String,
    pub vhigh: String,
}
