use std::{env, fs};

use anyhow::Ok;
use model::shared::ultihelper::ENVConfig;
use once_cell::sync::Lazy;

pub static CONFIGS: Lazy<ENVConfig> = Lazy::new(|| CommonFuntion::fn_com_get_config().unwrap());
pub struct CommonFuntion {}
impl CommonFuntion {
    pub fn fn_com_get_config() -> anyhow::Result<ENVConfig> {
        let mut path = env::current_dir().expect("CommonFuntion: Can't find folder!");
        path.push("configs/config.json");
        let config_data = fs::read_to_string(path).unwrap();
        let config = serde_json::from_str::<ENVConfig>(&config_data)?;
        Ok(config)
    }
}
