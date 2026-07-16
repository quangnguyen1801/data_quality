use std::collections::HashMap;

use polars::{error::PolarsResult, frame::DataFrame, prelude::*};

pub struct TestHelper;
impl TestHelper {
    pub fn fn_run_helper_convert_to_dataframe(
        data: Vec<HashMap<String, String>>,
    ) -> PolarsResult<DataFrame> {
        let mut all_keys = match data.first() {
            Some(row) => row.keys().cloned().collect(),
            None => vec![],
        };
        let mut columns = Vec::new();
        for key in all_keys {
            let values: Vec<String> = data
                .iter()
                .map(|row| row.get(&key).cloned().unwrap_or_default())
                .collect();
            //let column = Series::new(key.as_str().into(), values).cast(&DataType::String);
            let col = Column::new(key.as_str().into(), values);
            columns.push(col);
        }
        DataFrame::new(columns.into())
    }
}
