use model::shared::ultihelper::TestExecution;

pub trait TestLogic {
    fn fn_run_com_test_run(
        &self,
        test_execution: TestExecution,
    ) -> anyhow::Result<(bool, Option<String>)>;
}
