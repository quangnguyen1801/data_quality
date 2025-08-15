use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TestParameter {
    pub id: i32,
    pub test_id: i32,
    pub name: String,
    pub description: String,
}
