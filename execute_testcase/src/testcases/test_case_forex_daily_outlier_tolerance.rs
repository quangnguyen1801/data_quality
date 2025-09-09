use crate::utils::test_logic::TestLogic;

pub struct TestCaseForexDailyOutlierTolerance;
impl TestLogic for TestCaseForexDailyOutlierTolerance {
    fn fn_com_test_run(
        &self,
        row: &std::collections::HashMap<String, String>,
        metric: &str,
        params: &model::modelviews::configtestparameter_view::ConfigTestParameterView,
    ) -> anyhow::Result<(bool, Option<String>)> {
        todo!()
    }
}
