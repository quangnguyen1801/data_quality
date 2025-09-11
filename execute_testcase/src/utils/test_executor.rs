use anyhow::Ok;
use service::utils::common::Common;

use crate::utils::{test_registry::TestRegistry, test_result::TestResult};
#[allow(dead_code)]
pub struct TestExecutor;
impl TestExecutor {
    #[allow(dead_code)]
    pub async fn fn_run_execute_all() -> anyhow::Result<Vec<TestResult>> {
        let mut result = Vec::new();
        let list_test_executions = Common::fn_ser_com_build_execution_plans().await.unwrap();
        let list_test_registry = TestRegistry::fn_run_com_get_test_registry().unwrap_or_default();
        for test in list_test_executions.iter().cloned() {
            match list_test_registry.get(&test.test_name) {
                Some(logic) => {
                    let test_clone = test.clone();
                    let (passed, message) = logic.fn_run_com_test_run(test).unwrap_or_default();
                    let mut test_result = TestResult::default();
                    test_result.scope_id = test_clone.sope_id;
                    test_result.test_id = test_clone.test_id;
                    test_result.fn_run_com_test_add_check(passed, message);
                    result.push(test_result);
                }
                None => eprintln!("Can not find the {}", test.test_name),
            }
        }
        Ok(result)
    }
}
