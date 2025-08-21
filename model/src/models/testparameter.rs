use serde::{Deserialize, Serialize};
use tiberius_mappers::TryFromRow;

#[derive(Debug, Serialize, Deserialize, Clone, TryFromRow)]
pub struct TestParameter {
    pub id: i32,
    pub test_id: i32,
    pub name: String,
    pub description: String,
}
