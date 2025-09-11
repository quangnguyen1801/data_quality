use model::shared::ultihelper::TestExecution;

use crate::utils::test_logic::TestLogic;

pub struct TestCaseForexDailyOutlierTolerance;
impl TestLogic for TestCaseForexDailyOutlierTolerance {
    fn fn_run_com_test_run(
        &self,
        test_case: TestExecution,
    ) -> anyhow::Result<(bool, Option<String>)> {
        todo!()
    }
}
