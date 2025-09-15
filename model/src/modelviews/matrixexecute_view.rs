use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MatrixExecuteView {
    pub id: i32,
    pub test_id: i32,
    pub config_test_paramter_id: i32,
    pub scope_id: Vec<i32>,
    pub setting_version_id: i32,
}
