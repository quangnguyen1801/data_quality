use anyhow::Ok;
use model::shared::ultihelper::{ParameterSet, TestExecution};
use polars::error::PolarsResult;

use crate::utils::{test_helper::TestHelper, test_logic::TestLogic};

pub struct TestCaseForexDailyIsInCheck;
impl TestLogic for TestCaseForexDailyIsInCheck {
    fn fn_run_com_test_run(
        &self,
        test_case: TestExecution,
    ) -> anyhow::Result<(bool, Option<String>)> {
        let parameters = test_case.test_parameter;
        if test_case.test_data.is_empty() {
            return Ok((
                false,
                Some(
                    format!(
                        "Test '{}-{}' cannot run - no data available!",
                        test_case.test_id, test_case.test_name
                    )
                    .to_string(),
                ),
            ));
        } else {
            let mut data_frame =
                TestHelper::fn_run_helper_convert_to_dataframe(test_case.test_data);
            match data_frame {
                Result::Ok(df) => {
                    let mut cols = test_case.test_columns.clone();
                    let params: Vec<ParameterSet> = parameters
                        .iter()
                        .filter(|p| p.ref_columns == 1)
                        .cloned()
                        .collect();
                    // cols.extend(
                    //     parameters
                    //         .iter()
                    //         .filter(|p| p.ref_columns == 1)
                    //         .map(|p| p.param_value.iter().clone()),
                    // );
                    // let column = parameters.iter().filter(|p| p.ref_columns == 0).map(|p| p.param_value.first())
                    let actually_df = df.select(&cols).unwrap();

                    return Ok((true, None));
                }
                Result::Err(msg) => {
                    return Ok((
                        false,
                        Some(
                            format!(
                                "Test '{}-{}' cannot run - data not match with struct!",
                                test_case.test_id, test_case.test_name
                            )
                            .to_string(),
                        ),
                    ));
                }
            }
        }
    }
}
