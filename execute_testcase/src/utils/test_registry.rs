use std::collections::HashMap;

use anyhow::Ok;
use strum::IntoEnumIterator;

use crate::utils::{test_enum_custom::EnumTestType, test_logic::TestLogic};

pub struct TestRegistry {}
impl TestRegistry {
    pub fn fn_run_com_get_test_registry() -> anyhow::Result<HashMap<String, Box<dyn TestLogic>>> {
        let mut registry: HashMap<String, Box<dyn TestLogic>> = HashMap::new();
        let all_testcases: Vec<EnumTestType> = EnumTestType::iter().collect();
        for test in all_testcases.iter() {
            registry.insert(
                test.fn_run_enum_get_name().to_string(),
                test.fn_run_enum_get_logic(),
            );
        }
        Ok(registry)
    }
}
