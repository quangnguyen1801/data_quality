use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ENVConfig {
    pub app_setting: AppSetting,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppSetting {
    pub env: String,
    pub dbtype: String,
    pub sqlserver_connection: String,
    pub postgressql_connection: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TestExecution {
    pub test_id: i32,
    pub test_name: String,
    pub test_parameter: Vec<ParameterSet>,
    pub test_data: Vec<HashMap<String, String>>,
    pub test_columns: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ParameterSet {
    pub param_group: i32,
    pub param_id: i32,
    pub param_name: String,
    pub param_value: Vec<String>,
    pub operator: String,
    pub incl_excl: String,
}
