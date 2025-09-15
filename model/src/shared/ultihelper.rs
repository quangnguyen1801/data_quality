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
    pub postgresql_connection: String,
    pub settings_path: String,
    pub root_datasets_path: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TestExecution {
    pub test_id: i32,
    pub test_name: String,
    pub test_parameter: Vec<ParameterSet>,
    pub test_data: Vec<HashMap<String, String>>,
    pub test_columns: Vec<String>,
    pub sope_id: i32,
    pub matrix_id: i32,
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

use once_cell::sync::Lazy;
use std::{env, fs};

pub static CONFIGS: Lazy<ENVConfig> = Lazy::new(|| CommonFuntion::fn_repo_com_get_config());
pub struct CommonFuntion {}
impl CommonFuntion {
    pub fn fn_repo_com_get_config() -> ENVConfig {
        let mut path = env::current_dir().expect("CommonFuntion: Can't find folder!");
        path.push("configs/config.json");
        let config_data = fs::read_to_string(path).unwrap();
        let config = serde_json::from_str::<ENVConfig>(&config_data).unwrap();
        config
    }
}
