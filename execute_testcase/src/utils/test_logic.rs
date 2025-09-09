use std::collections::HashMap;

use model::modelviews::configtestparameter_view::ConfigTestParameterView;

pub trait TestLogic {
    fn fn_com_test_run(
        &self,
        row: &HashMap<String, String>,
        metric: &str,
        params: &ConfigTestParameterView,
    ) -> anyhow::Result<(bool, Option<String>)>;
}
