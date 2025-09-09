use crate::utils::test_logic::TestLogic;

pub struct TestCaseForexDailyIsInCheck;
impl TestLogic for TestCaseForexDailyIsInCheck {
    fn fn_com_test_run(
        &self,
        row: &std::collections::HashMap<String, String>,
        metric: &str,
        params: &model::modelviews::configtestparameter_view::ConfigTestParameterView,
    ) -> anyhow::Result<(bool, Option<String>)> {
        todo!()
    }
}
