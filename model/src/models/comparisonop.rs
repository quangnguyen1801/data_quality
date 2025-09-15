use serde::{Deserialize, Serialize};
use tiberius_mappers::TryFromRow;

#[derive(Debug, Serialize, Deserialize, Clone, TryFromRow)]
pub struct ComparisonOp {
    pub id: i32,
    pub inc_excl: String,
    pub operator: String,
    pub setting_version_id: i32,
}
