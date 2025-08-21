use serde::{Deserialize, Serialize};
use tiberius_mappers::TryFromRow;

#[derive(Debug, Serialize, Deserialize, Clone, TryFromRow)]
pub struct MatrixExecute {
    pub id: i32,
    pub test_id: i32,
    pub config_test_paramter_id: i32,
    pub scope_id: String,
}
