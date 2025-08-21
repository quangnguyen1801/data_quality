use serde::{Deserialize, Serialize};
use tiberius_mappers::TryFromRow;

#[derive(Debug, Serialize, Deserialize, Clone, TryFromRow)]
pub struct Selector {
    pub id: i32,
    pub field_name: String,
    pub inc_excl: String,
    pub operator: i32,
    pub vlow: String,
    pub vhigh: String,
}
