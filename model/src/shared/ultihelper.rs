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
